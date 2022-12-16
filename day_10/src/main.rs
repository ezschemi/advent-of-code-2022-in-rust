use std::fs;

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

impl CPU {
    fn new() -> CPU {
        CPU { X: 1, cycle: 0 }
    }

    fn execute_instructions_with_signal_strengths(
        &mut self,
        instructions: Vec<Instruction>,
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
    let lines = vec!["noop", "addx 3", "addx -5"];

    let input = fs::read_to_string("input.txt").unwrap();
    let lines = input.lines().collect::<Vec<_>>();

    let mut cpu = CPU::new();
    let instructions = create_instructions(&lines);

    // the descriptions says to sum up the *first six* signal strengths
    let signal_strengths = &cpu.execute_instructions_with_signal_strengths(instructions);

    assert!(signal_strengths.len() == 6);

    println!("CPU:\n{:#?}", cpu);

    println!("Signal Strengths:\n{:#?}", signal_strengths);
    println!("Sum: {}", signal_strengths.iter().sum::<isize>());
}
