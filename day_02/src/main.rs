use std::fs;
use std::str::FromStr;

#[derive(Clone, Copy, Debug, PartialEq)]
enum Move {
    Rock,
    Paper,
    Scissors,
}

impl TryFrom<char> for Move {
    type Error = color_eyre::Report;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            'A' => Ok(Self::Rock),
            'B' => Ok(Self::Paper),
            'C' => Ok(Self::Scissors),
            _ => Err(color_eyre::eyre::eyre!("not a valid move: {value:?}")),
        }
    }
}

impl Move {
    const ALL_MOVES: [Move; 3] = [Move::Rock, Move::Paper, Move::Scissors];

    fn winning_move(self) -> Self {
        Self::ALL_MOVES
            .iter()
            .copied()
            .find(|m| m.beats(self))
            .expect("at least one move needs to beat me.")
    }

    fn losing_move(self) -> Self {
        Self::ALL_MOVES
            .iter()
            .copied()
            .find(|&m| self.beats(m))
            .expect("at least one move needs to beat me.")
    }

    fn drawing_move(self) -> Self {
        self
    }

    fn inherent_points(self) -> usize {
        match self {
            Move::Rock => 1,
            Move::Paper => 2,
            Move::Scissors => 3,
        }
    }

    fn beats(self, opponents_play: Move) -> bool {
        matches!(
            (self, opponents_play),
            (Move::Rock, Move::Scissors)
                | (Move::Paper, Move::Rock)
                | (Move::Scissors, Move::Paper)
        )
    }

    fn outcome(self, opponents_play: Move) -> Outcome {
        if self.beats(opponents_play) {
            Outcome::Win
        } else if opponents_play.beats(self) {
            Outcome::Loss
        } else {
            Outcome::Draw
        }
    }
}
#[derive(Copy, Clone, Debug)]
enum Outcome {
    Loss,
    Draw,
    Win,
}

impl Outcome {
    fn inherent_points(self) -> usize {
        match self {
            Outcome::Win => 6,
            Outcome::Draw => 3,
            Outcome::Loss => 0,
        }
    }

    fn matching_move(self, opponents_play: Move) -> Move {
        match self {
            Outcome::Win => opponents_play.winning_move(),
            Outcome::Draw => opponents_play.drawing_move(),
            Outcome::Loss => opponents_play.losing_move(),
        }
    }
}

impl TryFrom<char> for Outcome {
    type Error = color_eyre::Report;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            'X' => Ok(Outcome::Loss),
            'Y' => Ok(Outcome::Draw),
            'Z' => Ok(Outcome::Win),
            _ => Err(color_eyre::eyre::eyre!("not a valid outcome: {value:?}")),
        }
    }
}

#[derive(Debug, Clone, Copy)]
struct Round {
    // these two are not used right now, but I want to keep them
    #[allow(dead_code)]
    opponents_play: Move,
    #[allow(dead_code)]
    my_play: Move,

    score: usize,
}

impl FromStr for Round {
    type Err = color_eyre::Report;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut chars = s.chars();

        let (Some(opponents_play), Some(' '), Some(outcome), None) = (chars.next(), chars.next(), chars.next(), chars.next()) else {
            return Err(color_eyre::eyre::eyre!("expected <theirs>SP<ours>EOF, got {s:?}"));
        };

        let opponents_play = Move::try_from(opponents_play)?;
        let outcome = Outcome::try_from(outcome)?;
        let my_play = outcome.matching_move(opponents_play);

        Ok(Self {
            opponents_play,
            my_play,
            score: 0,
        })
    }
}

impl Round {
    fn outcome(self) -> Outcome {
        self.my_play.outcome(self.opponents_play)
    }

    fn my_score(self) -> usize {
        self.my_play.inherent_points() + self.outcome().inherent_points()
    }
}

fn more_rustlike() -> color_eyre::Result<()> {
    // this collects all the rounds into the vector
    // thus using the memory.
    // BUT: All that's needed is the sum!
    let rounds: Vec<Round> = include_str!("../input.txt")
        .lines()
        .map(|line| line.parse())
        .collect::<Result<_, _>>()?;

    let total_score: usize = rounds.iter().map(|round| round.my_score()).sum();
    println!("total score: {}", total_score);

    // with this imperative approach, we can calculate only the
    // sum without storing the rounds in a vector.
    let mut total_score = 0;
    for round in include_str!("../input.txt").lines().map(Round::from_str) {
        total_score += round?.my_score();
    }

    println!("total score: {}", total_score);

    Ok(())
}
fn with_iterators() -> color_eyre::Result<()> {
    let total_score: usize = itertools::process_results(
        include_str!("../input.txt")
            .lines()
            .map(Round::from_str)
            .map(|round| round.map(|round| round.my_score())),
        |it| it.sum(),
    )?;

    println!("total score: {}", total_score);

    Ok(())
}

fn calculate_score(opponents_play: Move, my_play: Move) -> usize {
    let shape_score = match my_play {
        Move::Rock => 1,
        Move::Paper => 2,
        Move::Scissors => 3,
    };

    let outcome_score = match opponents_play {
        Move::Rock => {
            if my_play == Move::Rock {
                3
            } else if my_play == Move::Paper {
                6
            } else {
                // Scissors
                0
            }
        }
        Move::Paper => {
            if my_play == Move::Rock {
                0
            } else if my_play == Move::Paper {
                3
            } else {
                // Scissors
                6
            }
        }
        Move::Scissors => {
            if my_play == Move::Rock {
                6
            } else if my_play == Move::Paper {
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
            'A' => Move::Rock,
            'B' => Move::Paper,
            'C' => Move::Scissors,
            _ => unreachable!(),
        };
        let my_play = match char_my_play {
            'X' => Move::Rock,
            'Y' => Move::Paper,
            'Z' => Move::Scissors,
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
            'A' => Move::Rock,
            'B' => Move::Paper,
            'C' => Move::Scissors,
            _ => unreachable!(),
        };
        let outcome = match char_outcome {
            'X' => Outcome::Loss,
            'Y' => Outcome::Draw,
            'Z' => Outcome::Win,
            _ => unreachable!(),
        };
        let my_play = match outcome {
            Outcome::Loss => {
                if opponents_play == Move::Rock {
                    Move::Scissors
                } else if opponents_play == Move::Paper {
                    Move::Rock
                } else {
                    // opponents_play == HandShape::Scissors
                    Move::Paper
                }
            }
            Outcome::Draw => {
                if opponents_play == Move::Rock {
                    Move::Rock
                } else if opponents_play == Move::Paper {
                    Move::Paper
                } else {
                    // opponents_play == HandShape::Scissors
                    Move::Scissors
                }
            }
            Outcome::Win => {
                if opponents_play == Move::Rock {
                    Move::Paper
                } else if opponents_play == Move::Paper {
                    Move::Scissors
                } else {
                    // opponents_play == HandShape::Scissors
                    Move::Rock
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

fn imperative() -> color_eyre::Result<()> {
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

    // println!("Rounds: {:?}", rounds);

    let total_score_puzzle_1: usize = rounds_puzzle_1.iter().map(|r| r.score).sum();
    println!("Total score for Puzzle 1: {}", total_score_puzzle_1);

    let total_score_puzzle_2: usize = rounds_puzzle_2.iter().map(|r| r.score).sum();
    println!("Total score for Puzzle 2: {}", total_score_puzzle_2);

    Ok(())
}
fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;

    imperative()?;

    more_rustlike()?;

    with_iterators()?;

    Ok(())
}
