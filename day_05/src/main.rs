use std::fmt;
use std::fs;

use itertools::Itertools;
use nom::bytes::complete::take_while1;
use nom::multi::separated_list1;
// use bytes::complete as all the bytes are there, and no
// streaming parser is needed (bytes::streaming)
use nom::{
    branch::alt,
    bytes::complete::{tag, take},
    character::complete::digit1,
    combinator::{all_consuming, map, map_res, opt},
    sequence::{delimited, preceded, tuple},
    Finish, IResult,
};
use smallvec::SmallVec;

use std::alloc::System;
use tracking_allocator::{
    AllocationGroupId, AllocationGroupToken, AllocationRegistry, AllocationTracker, Allocator,
};

#[global_allocator]
static GLOBAL: tracking_allocator::Allocator<std::alloc::System> =
    tracking_allocator::Allocator::system();

// #[global_allocator]
// static GLOBAL: Allocator<System> = Allocator::system();

struct StdoutTracker;

impl AllocationTracker for StdoutTracker {
    fn allocated(
        &self,
        addr: usize,
        object_size: usize,
        wrapped_size: usize,
        group_id: AllocationGroupId,
    ) {
        println!(
            "allocation -> addr=0x{:0x} object_size={} wrapped_size={} group_id={:?}",
            addr, object_size, wrapped_size, group_id
        );
        // panic!();
    }

    fn deallocated(
        &self,
        addr: usize,
        object_size: usize,
        wrapped_size: usize,
        source_group_id: AllocationGroupId,
        current_group_id: AllocationGroupId,
    ) {
        println!(
            "deallocation -> addr=0x{:0x} object_size={} wrapped_size={} source_group_id={:?} current_group_id={:?}",
            addr, object_size, wrapped_size, source_group_id, current_group_id
        );
    }
}

#[derive(Clone, Copy)]
struct Crate(char);

impl fmt::Debug for Crate {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}
impl fmt::Display for Crate {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Debug::fmt(self, f)
    }
}
#[derive(Clone, Debug)]
struct Instruction {
    quantity: usize,
    src: usize,
    dst: usize,
}

#[derive(Clone)]
struct Piles(Vec<Vec<Crate>>);

impl fmt::Debug for Piles {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for (i, pile) in self.0.iter().enumerate() {
            writeln!(f, "Pile {}: {:?}", i, pile)?;
        }
        Ok(())
    }
}
#[derive(Debug)]
enum InstructionApplicationType {
    Type_9000,
    Type_9001,
    Type_9001_smallVec,
}
impl Piles {
    fn apply_9000(&mut self, instruction: &Instruction) {
        for _ in 0..instruction.quantity {
            let e = self.0[instruction.src].pop().unwrap();
            self.0[instruction.dst].push(e);
        }
    }

    fn apply_9001(&mut self, instruction: &Instruction) {
        // cant do the following code, as the borrow checker
        // can't know that src and dst will never point to the same value.
        // "crate" is a keyword, so use different spelling.
        // for krate in (0..instruction.quantity)
        //     .map(|_| self.0[instruction.src].pop().unwrap())
        //     .rev()
        // {
        //     self.0[instruction.dst].push(krate);
        // }

        for krate in (0..instruction.quantity)
            .map(|_| self.0[instruction.src].pop().unwrap())
            .collect::<Vec<_>>()
            .into_iter()
            .rev()
        {
            self.0[instruction.dst].push(krate);
        }
    }

    fn apply_9001_smallvec(&mut self, instruction: &Instruction) {
        // see remarks in apply_9001()!

        for krate in (0..instruction.quantity)
            .map(|_| self.0[instruction.src].pop().unwrap())
            .collect::<SmallVec<[_; 64]>>()
            .into_iter()
            .rev()
        {
            self.0[instruction.dst].push(krate);
        }
    }
}

fn parse_crate(i: &str) -> IResult<&str, Crate> {
    let first_char = |s: &str| Crate(s.chars().next().unwrap());
    let f = delimited(tag("["), take(1_usize), tag("]"));

    map(f, first_char)(i)
}

fn parse_hole(i: &str) -> IResult<&str, ()> {
    map(tag("   "), drop)(i)
}

fn parse_crate_or_hole(i: &str) -> IResult<&str, Option<Crate>> {
    alt((map(parse_crate, Some), map(parse_hole, |_| None)))(i)
}

fn parse_crate_line(i: &str) -> IResult<&str, Vec<Option<Crate>>> {
    // let (mut i, c) = parse_crate_or_hole(i)?;
    // let mut v = vec![c];

    // loop {
    //     let (next_i, maybe_c) = opt(preceded(tag(" "), parse_crate_or_hole))(i)?;
    //     match maybe_c {
    //         Some(c) => v.push(c),
    //         None => break,
    //     }
    //     i = next_i;
    // }

    // Ok((i, v))

    separated_list1(tag(" "), parse_crate_or_hole)(i)
}

fn parse_number(i: &str) -> IResult<&str, usize> {
    map_res(digit1, |s: &str| s.parse::<usize>())(i)
}

// convert numbers to indexes
fn parse_pile_number(i: &str) -> IResult<&str, usize> {
    map(parse_number, |i| i - 1)(i)
}

fn parse_instruction(i: &str) -> IResult<&str, Instruction> {
    map(
        tuple((
            preceded(tag("move "), parse_number),
            preceded(tag(" from "), parse_pile_number),
            preceded(tag(" to "), parse_pile_number),
        )),
        |(quantity, src, dst)| Instruction { quantity, src, dst },
    )(i)
}

fn transpose_reverse<T>(v: Vec<Vec<Option<T>>>) -> Vec<Vec<T>> {
    assert!(!v.is_empty());

    let len = v[0].len();
    let mut iters: Vec<_> = v.into_iter().map(|n| n.into_iter()).collect();

    (0..len)
        .map(|_| {
            // trading extra memory usage now for less allocations later
            let mut v = Vec::with_capacity(256);
            v.extend(iters.iter_mut().rev().filter_map(|n| n.next().unwrap()));
            v
        })
        .collect()
}

fn functional_style() -> color_eyre::Result<()> {
    let mut lines = include_str!("../input.txt").lines();

    let crate_lines: Vec<_> = (&mut lines)
        .map_while(|line| {
            all_consuming(parse_crate_line)(line)
                .finish()
                .ok()
                .map(|(_, line)| line)
        })
        .collect();

    // consume the empty line between the stack config
    // and the instructions
    assert!(lines.next().unwrap().is_empty());

    let mut piles = Piles(transpose_reverse(crate_lines));
    let mut piles_9001 = piles.clone();
    let mut piles_9001_smallvec = piles.clone();
    println!("Piles:\n{piles:?}");

    let instructions = lines.map(|line| all_consuming(parse_instruction)(line).finish().unwrap().1);

    // for allocation tracking only
    let iat = InstructionApplicationType::Type_9001_smallVec;

    AllocationRegistry::set_global_tracker(StdoutTracker)
        .expect("no other global tracker should be set yet");

    AllocationRegistry::enable_tracking();
    match iat {
        InstructionApplicationType::Type_9000 => {
            for i in instructions {
                piles.apply_9000(&i);
            }
        }
        InstructionApplicationType::Type_9001 => {
            for i in instructions {
                piles_9001.apply_9001(&i);
            }
        }

        InstructionApplicationType::Type_9001_smallVec => {
            for i in instructions {
                piles_9001_smallvec.apply_9001_smallvec(&i);
            }
        }
    }
    println!("Allocations with {iat:?}:");
    AllocationRegistry::disable_tracking();

    // AllocationRegistry::enable_tracking();
    // for i in instructions_9001 {
    //     piles_9001.apply_9001(&i);
    // }
    // println!("Allocations:");
    // AllocationRegistry::disable_tracking();

    // AllocationRegistry::enable_tracking();
    // for i in instructions_9001_small_vec {
    //     piles_9001_smallvec.apply_9001_smallvec(&i);
    // }
    // println!("Allocations:");
    // AllocationRegistry::disable_tracking();

    println!("Piles 9000 after applying all the instructions:\n{piles:?}");

    println!(
        "answer = {}",
        piles.0.iter().map(|pile| pile.last().unwrap()).join("")
    );

    println!("Piles 9001 after applying all the instructions:\n{piles_9001:?}");

    println!(
        "answer = {}",
        piles_9001
            .0
            .iter()
            .map(|pile| pile.last().unwrap())
            .join("")
    );

    println!(
        "Piles 9001 with SmallVec after applying all the instructions:\n{piles_9001_smallvec:?}"
    );

    println!(
        "answer = {}",
        piles_9001_smallvec
            .0
            .iter()
            .map(|pile| pile.last().unwrap())
            .join("")
    );

    println!("The piles and answers might be incorrect as *ONLY* the pile of the selected type is actually being applied!");

    Ok(())
}

#[derive(Clone, Debug)]
struct Stack {
    name: usize,
    stack: Vec<char>,
}

impl Stack {
    fn new(number: usize) -> Stack {
        Stack {
            name: number,
            stack: vec![],
        }
    }
}
#[derive(Clone, Debug)]
struct Stacks {
    stacks: Vec<Stack>,
}

impl Stacks {
    fn new(initial_stack_layout_strs: Vec<&str>) -> Stacks {
        // a stack is represented by 3 chars: either three spaces, or [,char,]
        // two stacks are separated by a single space
        // a line describing two stacks is 3+1+3 chars
        // a line describing three stacks is 3+1+3+1+3=11 chars
        let mut stacks: Vec<Stack> = Vec::new();

        let mut n_stacks = 0;

        // NOTE the reversed line order here: this starts the stack construction
        // at the bottom, going up. The lines in the file are given from top to
        // bottom though, for humans to read!
        for line in initial_stack_layout_strs.iter().rev() {
            println!("line: {}", line);
            let chars: Vec<char> = line.chars().collect();

            if n_stacks == 0 {
                // TODO this way is kind of ugly...

                // this line contains the stack names, so grab the number of stacks
                // from there
                n_stacks = initial_stack_layout_strs
                    .last()
                    .unwrap()
                    .split_whitespace()
                    .count();

                continue;
            }

            if stacks.len() == 0 {
                // TODO is this the way to do it in Rust?
                for i in 0..n_stacks {
                    stacks.push(Stack::new(i + 1));
                }
            }

            for i in 0..n_stacks {
                // skip the bracket, skip the whitespace between stacks, skip the stacks
                let char_index = 1 + i * 3 + i * 1;

                let stack_char = chars[char_index];
                // println!("stack_char {}: {}", i, stack_char);

                if stack_char == ' ' {
                    // nothing here on the stack
                    continue;
                }

                stacks[i].stack.push(stack_char);
            }
        }

        Stacks { stacks }
    }

    pub fn apply_CrateMover9000(&mut self, instruction: &MoveInstruction) {
        let from_stack: &mut Stack = &mut self.stacks[instruction.from - 1];

        let count = instruction.count;

        // TODO how to do this without copying the values to a temporary stack?
        let mut temp_stack = Stack::new(usize::MAX);

        for _ in 0..count {
            let v = from_stack.stack.pop().unwrap();

            temp_stack.stack.push(v);
        }

        let to_stack: &mut Stack = &mut self.stacks[instruction.to - 1];
        for v in temp_stack.stack {
            to_stack.stack.push(v);
        }
    }

    pub fn apply_CrateMover9001(&mut self, instruction: &MoveInstruction) {
        let from_stack: &mut Stack = &mut self.stacks[instruction.from - 1];

        let count = instruction.count;

        // TODO how to do this without copying the values to a temporary stack?
        let mut temp_stack = Stack::new(usize::MAX);

        for _ in 0..count {
            let v = from_stack.stack.pop().unwrap();

            temp_stack.stack.push(v);
        }

        let to_stack: &mut Stack = &mut self.stacks[instruction.to - 1];
        for v in temp_stack.stack.iter().rev() {
            to_stack.stack.push(*v);
        }
    }

    pub fn crates_on_top(&self) -> String {
        let mut result = String::new();

        for stack in self.stacks.iter() {
            let crate_on_top = stack.stack.last().unwrap().to_string();
            result.push_str(&String::from(crate_on_top));
        }

        result
    }
}
#[derive(Debug)]
struct MoveInstruction {
    count: usize,
    from: usize,
    to: usize,
}

impl MoveInstruction {
    pub fn new_from_string(s: &str) -> Self {
        let iter: Vec<&str> = s.split_whitespace().collect();

        // TODO how to do this via the iterator?

        // example:
        // move 1 from 2 to 1
        let count = iter[1].parse::<usize>().unwrap();
        let from = iter[3].parse::<usize>().unwrap();
        let to = iter[5].parse::<usize>().unwrap();

        MoveInstruction { count, from, to }
    }
}
fn imperative_style() -> color_eyre::Result<()> {
    let input_str = "    [D]    
[N] [C]    
[Z] [M] [P]
 1   2   3 

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2";

    let lines = input_str.lines();

    let input_filename = String::from("input.txt");
    let content = fs::read_to_string(&input_filename).unwrap();
    let lines = content.lines();

    let mut initial_stack_layout_strs = Vec::new();
    let mut instructions_strs = Vec::new();

    let mut still_in_layout = true;

    for line in lines {
        if line.is_empty() {
            still_in_layout = false;

            continue;
        }
        if still_in_layout {
            initial_stack_layout_strs.push(line);
        } else {
            instructions_strs.push(line);
        }
    }

    println!("Initial layout: {:?}", initial_stack_layout_strs.len());
    println!("Instructions: {:?}", instructions_strs.len());

    let mut stacks_CrateMover9000 = Stacks::new(initial_stack_layout_strs);

    // println!("Stacks:\n{:#?}", stacks);

    let mut instructions = Vec::new();

    for line in instructions_strs {
        let instruction = MoveInstruction::new_from_string(line);

        instructions.push(instruction);
    }

    // println!("Instructions:\n{:#?}", instructions);

    let mut stack_CrateMover9001 = stacks_CrateMover9000.clone();

    for instruction in instructions {
        // println!("Applying instruction: {:#?}", instruction);
        stacks_CrateMover9000.apply_CrateMover9000(&instruction);

        stack_CrateMover9001.apply_CrateMover9001(&instruction);

        // println!("Stacks:\n{:#?}\n", stacks);
    }

    let crates_on_top = stacks_CrateMover9000.crates_on_top();
    println!(
        "Crates on top after applying all the instructions:\n{:#?}",
        crates_on_top
    );

    let crates_on_top = stack_CrateMover9001.crates_on_top();
    println!(
        "Crates on top after applying all the instructions:\n{:#?}",
        crates_on_top
    );

    Ok(())
}

fn main() -> color_eyre::Result<()> {
    imperative_style()?;

    functional_style()?;

    Ok(())
}
