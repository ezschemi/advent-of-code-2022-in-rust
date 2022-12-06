use std::fs;

#[derive(Clone, Copy, Debug, PartialEq)]
enum HandShape {
    Rock,
    Paper,
    Scissors,
}

enum Outcome {
    Win,
    Lose,
    Draw,
}

#[derive(Debug)]
struct Round {
    opponents_play: HandShape,
    my_play: HandShape,
    score: usize,
}

fn calculate_score(opponents_play: HandShape, my_play: HandShape) -> usize {
    let shape_score = match my_play {
        HandShape::Rock => 1,
        HandShape::Paper => 2,
        HandShape::Scissors => 3,
    };

    let outcome_score = match opponents_play {
        HandShape::Rock => {
            if my_play == HandShape::Rock {
                3
            } else if my_play == HandShape::Paper {
                6
            } else {
                // Scissors
                0
            }
        }
        HandShape::Paper => {
            if my_play == HandShape::Rock {
                0
            } else if my_play == HandShape::Paper {
                3
            } else {
                // Scissors
                6
            }
        }
        HandShape::Scissors => {
            if my_play == HandShape::Rock {
                6
            } else if my_play == HandShape::Paper {
                0
            } else {
                // Scissors
                3
            }
        }
    };

    shape_score + outcome_score
}

impl Round {
    fn new_plays_from_chars(char_opponents_play: char, char_my_play: char) -> Round {
        let opponents_play = match char_opponents_play {
            'A' => HandShape::Rock,
            'B' => HandShape::Paper,
            'C' => HandShape::Scissors,
            _ => unreachable!(),
        };
        let my_play = match char_my_play {
            'X' => HandShape::Rock,
            'Y' => HandShape::Paper,
            'Z' => HandShape::Scissors,
            _ => unreachable!(),
        };
        Round {
            opponents_play,
            my_play,
            score: calculate_score(opponents_play, my_play),
        }
    }
    fn new_outcomes_from_chars(char_opponents_play: char, char_outcome: char) -> Round {
        let opponents_play = match char_opponents_play {
            'A' => HandShape::Rock,
            'B' => HandShape::Paper,
            'C' => HandShape::Scissors,
            _ => unreachable!(),
        };
        let outcome = match char_outcome {
            'X' => Outcome::Lose,
            'Y' => Outcome::Draw,
            'Z' => Outcome::Win,
            _ => unreachable!(),
        };
        let my_play = match outcome {
            Outcome::Lose => {
                if opponents_play == HandShape::Rock {
                    HandShape::Scissors
                } else if opponents_play == HandShape::Paper {
                    HandShape::Rock
                } else {
                    // opponents_play == HandShape::Scissors
                    HandShape::Paper
                }
            }
            Outcome::Draw => {
                if opponents_play == HandShape::Rock {
                    HandShape::Rock
                } else if opponents_play == HandShape::Paper {
                    HandShape::Paper
                } else {
                    // opponents_play == HandShape::Scissors
                    HandShape::Scissors
                }
            }
            Outcome::Win => {
                if opponents_play == HandShape::Rock {
                    HandShape::Paper
                } else if opponents_play == HandShape::Paper {
                    HandShape::Scissors
                } else {
                    // opponents_play == HandShape::Scissors
                    HandShape::Rock
                }
            }
        };
        Round {
            opponents_play,
            my_play,
            score: calculate_score(opponents_play, my_play),
        }
    }
}
fn main() {
    let input_filename = String::from("input.txt");

    let content = fs::read_to_string(&input_filename).unwrap();

    let lines = content.lines();

    let mut rounds_puzzle_1 = Vec::new();
    let mut rounds_puzzle_2 = Vec::new();

    for line in lines {
        let line = line.trim();
        let char_0 = line.chars().nth(0).unwrap();
        let char_1 = line.chars().last().unwrap();

        rounds_puzzle_1.push(Round::new_plays_from_chars(char_0, char_1));

        rounds_puzzle_2.push(Round::new_outcomes_from_chars(char_0, char_1));
    }

    println!(
        "Rounds: {} and {}",
        rounds_puzzle_1.len(),
        rounds_puzzle_2.len()
    );

    // println!("Rounds: {:?}", rounds);

    let total_score_puzzle_1: usize = rounds_puzzle_1.iter().map(|r| r.score).sum();
    println!("Total score for Puzzle 1: {}", total_score_puzzle_1);

    let total_score_puzzle_2: usize = rounds_puzzle_2.iter().map(|r| r.score).sum();
    println!("Total score for Puzzle 2: {}", total_score_puzzle_2);
}
