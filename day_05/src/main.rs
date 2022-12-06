use std::fs;

#[derive(Debug)]
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
#[derive(Debug)]
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

    pub fn apply(&mut self, instruction: MoveInstruction) {
        let from_stack: &mut Stack = &mut self.stacks[instruction.from - 1];

        let count = instruction.count;

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

    pub fn crates_on_top(&self) -> String {
        let mut result = String::new();

        for stack in self.stacks.iter() {
            let crate_on_top = stack.stack.last().unwrap().to_string();
            result.push_str(&String::from(crate_on_top));
            // result += stack.stack.last().unwrap().to_string();
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
fn main() {
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

    let mut stacks = Stacks::new(initial_stack_layout_strs);

    // println!("Stacks:\n{:#?}", stacks);

    let mut instructions = Vec::new();

    for line in instructions_strs {
        let instruction = MoveInstruction::new_from_string(line);

        instructions.push(instruction);
    }

    // println!("Instructions:\n{:#?}", instructions);

    for instruction in instructions {
        // println!("Applying instruction: {:#?}", instruction);
        stacks.apply(instruction);

        // println!("Stacks:\n{:#?}\n", stacks);
    }

    let crates_on_top = stacks.crates_on_top();
    println!(
        "Crates on top after applying all the instructions:\n{:#?}",
        crates_on_top
    );
}
