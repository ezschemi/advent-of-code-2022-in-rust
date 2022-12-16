use std::{
    fmt, fs,
    ops::{Add, Sub},
};

#[derive(Clone, Copy, Debug)]
enum InstructionType {
    Noop,
    Addx,
}
#[derive(Clone, Copy, Debug)]
struct Instruction {
    type_: InstructionType,
    cycles_needed: usize,
    v: isize,
}

#[derive(Clone, Copy, Debug)]
struct CPU {
    X: isize,
    cycle: usize,
}
#[derive(Clone)]
struct CRT {
    rows: [[char; 40]; 6],
    current_x: usize,
    current_row: usize,
}
impl CRT {
    fn new() -> Self {
        let rows = [['.'; 40]; 6];

        CRT {
            rows,
            current_x: 0,
            current_row: 0,
        }
    }

    // draw will advance the draw pointers.
    pub fn draw(&mut self, c: char) {
        self.rows[self.current_row][self.current_x] = c;

        self.current_x += 1;

        if self.current_x == 40 {
            self.current_x = 0;
            self.current_row += 1;
        }
    }

    pub fn draw_sprite(&mut self, pos_x: usize) {
        // pos X is the center position of a 3-char sprite
        // if any of the three chars is currently being drawn,
        // draw the sprite with a '#',
        // otherwise draw a '.'

        let x: isize = self.current_x as isize;

        if x.sub(1) == pos_x as isize || x == pos_x as isize || x.add(1) == pos_x as isize {
            self.draw('#');
        } else {
            self.draw('.');
        }
    }
}

impl fmt::Debug for CRT {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut s = String::new();
        for row in &self.rows {
            for c in row {
                s.push(*c);
            }
            s.push('\n');
        }

        f.write_str(&s)
    }
}

impl CPU {
    fn new() -> CPU {
        CPU { X: 1, cycle: 0 }
    }

    fn execute_instructions_with_signal_strengths(
        &mut self,
        instructions: &Vec<Instruction>,
    ) -> Vec<isize> {
        let mut signal_strengths = Vec::new();
        let mut cycle_number_of_interest = 20;

        for i in instructions {
            match i.type_ {
                InstructionType::Noop => {
                    if self.cycle == cycle_number_of_interest {
                        let signal_strength = self.cycle as isize * self.X;
                        signal_strengths.push(signal_strength);
                        println!(
                            "signal_strength at cycle {} = {} = {} * {}",
                            self.cycle, signal_strength, self.cycle as isize, self.X
                        );

                        cycle_number_of_interest += 40;
                    }

                    self.cycle += i.cycles_needed;
                }
                InstructionType::Addx => {
                    for _ in 0..i.cycles_needed {
                        self.cycle += 1;

                        if self.cycle == cycle_number_of_interest {
                            let signal_strength = self.cycle as isize * self.X;
                            signal_strengths.push(signal_strength);
                            println!(
                                "signal_strength at cycle {} = {} = {} * {}",
                                self.cycle, signal_strength, self.cycle as isize, self.X
                            );

                            cycle_number_of_interest += 40;
                        }
                    }
                    self.X += i.v;
                }
            }
        }

        signal_strengths
    }

    fn execute_instructions_with_CRT(&mut self, instructions: &Vec<Instruction>, crt: &mut CRT) {
        for i in instructions {
            match i.type_ {
                InstructionType::Noop => {
                    self.cycle += i.cycles_needed;

                    // crt.draw('.');
                    crt.draw_sprite(self.X as usize);
                }
                InstructionType::Addx => {
                    for _ in 0..i.cycles_needed {
                        self.cycle += 1;

                        // crt.draw('#');
                        crt.draw_sprite(self.X as usize);
                    }
                    self.X += i.v;
                }
            }
        }
    }
}

fn create_instructions(lines: &Vec<&str>) -> Vec<Instruction> {
    let mut instructions: Vec<Instruction> = Vec::new();

    for line in lines {
        if line.as_bytes() == b"noop" {
            instructions.push(Instruction {
                type_: InstructionType::Noop,
                cycles_needed: 1,
                v: 0,
            });
        } else if line.starts_with("addx ") {
            let tokens = line.split_whitespace().collect::<Vec<_>>();
            let v = tokens[1].parse::<isize>().unwrap();

            instructions.push(Instruction {
                type_: InstructionType::Addx,
                cycles_needed: 2,
                v,
            });
        } else {
            panic!("Unknown instruction: {}", line);
        }
    }

    instructions
}

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();
    let lines = input.lines().collect::<Vec<_>>();

    let mut cpu = CPU::new();
    let instructions = create_instructions(&lines);

    // the descriptions says to sum up the *first six* signal strengths
    let signal_strengths = &cpu.execute_instructions_with_signal_strengths(&instructions);

    assert!(signal_strengths.len() == 6);

    println!("CPU:\n{:#?}", cpu);

    println!("Signal Strengths:\n{:#?}", signal_strengths);
    println!("Sum: {}", signal_strengths.iter().sum::<isize>());

    let mut cpu = CPU::new();
    let mut crt = CRT::new();
    cpu.execute_instructions_with_CRT(&instructions, &mut crt);

    println!("CRT:\n{:?}", crt);
}
