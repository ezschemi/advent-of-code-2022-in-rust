use core::fmt;
use std::fmt::Write;

#[derive(Debug)]
enum MoveDirection {
    Right,
    Up,
    Left,
    Down,
}
#[derive(Debug)]
struct MoveInstruction {
    direction: MoveDirection,
    count: usize,
}

fn create_instructions(lines: &Vec<&str>) -> Vec<MoveInstruction> {
    let mut instructions = Vec::new();

    for line in lines {
        let tokens = line.split_whitespace().collect::<Vec<&str>>();
        let direction = match tokens[0].as_bytes() {
            b"R" => MoveDirection::Right,
            b"U" => MoveDirection::Up,
            b"L" => MoveDirection::Left,
            b"D" => MoveDirection::Down,
            _ => panic!("unsupported character."),
        };

        let count = tokens[1].parse::<usize>().unwrap();

        instructions.push(MoveInstruction { direction, count });
    }

    instructions
}

#[derive(Debug, Clone, Copy)]
enum GridPosition {
    Dot,
    Hash,
    Head,
    Tail,
}

struct Grid {
    grid: Vec<Vec<GridPosition>>,
    head_pos: (isize, isize),
    tail_pos: (isize, isize),
}

impl fmt::Debug for Grid {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for y in (0..self.grid.len()).rev() {
            let mut output_line = String::new();
            for x in 0..self.grid[y].len() {
                let p = self.at(x, y);
                let c = match p {
                    GridPosition::Dot => '.',
                    GridPosition::Hash => '#',
                    GridPosition::Head => 'H',
                    GridPosition::Tail => 'T',
                };
                output_line.push(c);
            }
            let output = format!("{output_line}\n");
            f.write_str(&output)?;
        }
        Ok(())
    }
}

impl Grid {
    pub fn at(&self, x: usize, y: usize) -> GridPosition {
        self.grid[y][x]
    }
    pub fn set(&mut self, x: usize, y: usize, p: GridPosition) {
        self.grid[y][x] = p;
    }
    pub fn new() -> Self {
        let mut grid = Vec::new();
        let line = vec![
            GridPosition::Dot,
            GridPosition::Dot,
            GridPosition::Dot,
            GridPosition::Dot,
            GridPosition::Dot,
            GridPosition::Dot,
        ];
        grid.push(line);

        let line = vec![
            GridPosition::Dot,
            GridPosition::Dot,
            GridPosition::Dot,
            GridPosition::Dot,
            GridPosition::Dot,
            GridPosition::Dot,
        ];
        grid.push(line);

        let line = vec![
            GridPosition::Dot,
            GridPosition::Dot,
            GridPosition::Dot,
            GridPosition::Dot,
            GridPosition::Dot,
            GridPosition::Dot,
        ];
        grid.push(line);

        let line = vec![
            GridPosition::Dot,
            GridPosition::Dot,
            GridPosition::Dot,
            GridPosition::Dot,
            GridPosition::Dot,
            GridPosition::Dot,
        ];
        grid.push(line);

        let line = vec![
            GridPosition::Dot,
            GridPosition::Dot,
            GridPosition::Dot,
            GridPosition::Dot,
            GridPosition::Dot,
            GridPosition::Dot,
        ];
        grid.push(line);

        let head_pos: (isize, isize) = (0, 0);
        let tail_pos: (isize, isize) = (0, 0);

        let mut grid = Grid {
            grid,
            head_pos,
            tail_pos,
        };

        grid.set(tail_pos.0 as usize, tail_pos.1 as usize, GridPosition::Tail);
        grid.set(head_pos.0 as usize, head_pos.1 as usize, GridPosition::Head);

        grid
    }

    pub fn apply(&mut self, instruction: &MoveInstruction) {
        let dir: (isize, isize) = match instruction.direction {
            MoveDirection::Right => (1, 0),
            MoveDirection::Up => (0, 1),
            MoveDirection::Left => (-1, 0),
            MoveDirection::Down => (0, -1),
        };
        for i in 0..instruction.count {
            let old_head_pos = self.head_pos;
            let new_head_pos = (old_head_pos.0 + dir.0, old_head_pos.1 + dir.1);

            self.set(
                old_head_pos.0 as usize,
                old_head_pos.1 as usize,
                GridPosition::Dot,
            );
            self.set(
                new_head_pos.0 as usize,
                new_head_pos.1 as usize,
                GridPosition::Head,
            );

            self.head_pos = new_head_pos;

            let delta_head_tail = (
                self.head_pos.0 - self.tail_pos.0,
                self.head_pos.1 - self.tail_pos.1,
            );

            println!("delta_head_tail: {:?}", delta_head_tail);

            if delta_head_tail.0 > 1 || delta_head_tail.1 > 1 {
                // need to move the tail now too
                let old_tail_pos = self.tail_pos;
                let mut new_tail_pos = old_tail_pos.clone();

                if delta_head_tail.0 > 1 {
                    new_tail_pos.0 += 1;
                } else if delta_head_tail.0 < 1 {
                    new_tail_pos.0 -= 1;
                }
                if delta_head_tail.1 > 1 {
                    new_tail_pos.1 += 1;
                } else if delta_head_tail.1 < 1 {
                    new_tail_pos.1 -= 1;
                }

                if new_tail_pos.0 < 0 {
                    new_tail_pos.0 = 0;
                }

                if new_tail_pos.1 < 0 {
                    new_tail_pos.1 = 0;
                }

                self.set(
                    old_tail_pos.0 as usize,
                    old_tail_pos.1 as usize,
                    GridPosition::Dot,
                );
                self.set(
                    new_tail_pos.0 as usize,
                    new_tail_pos.1 as usize,
                    GridPosition::Tail,
                );

                self.tail_pos = new_tail_pos;
            }
        }
    }
}

fn main() {
    let mut grid = Grid::new();

    println!("Grid:\n{:#?}", grid);

    let lines = include_str!("../input-small2.txt").lines().collect();

    let instructions = create_instructions(&lines);

    println!("Instructions: {}", instructions.len());

    for ins in instructions {
        println!("Instruction: {:#?}", ins);
        grid.apply(&ins);

        println!("Grid:\n{:#?}", grid);
    }
}

#[cfg(test)]
mod tests {
    use crate::create_instructions;
    use crate::Grid;
    use test_case::test_case;

    #[test_case(2, 2)]
    fn test_head_moving(expected_head_pos_x: usize, expected_head_pos_y: usize) {
        let mut grid = Grid::new();

        println!("Grid:\n{:#?}", grid);

        let lines = include_str!("../input-small.txt").lines().collect();

        let instructions = create_instructions(&lines);

        println!("Instructions: {}", instructions.len());

        for ins in instructions {
            println!("Instruction: {:#?}", ins);
            grid.apply(&ins);

            println!("Grid:\n{:#?}", grid);
        }

        assert_eq!(grid.head_pos.0 as usize, expected_head_pos_x);
        assert_eq!(grid.head_pos.1 as usize, expected_head_pos_y);
    }

    #[test_case(1, 2)]
    fn test_tail_moving(expected_tail_pos_x: usize, expected_tail_pos_y: usize) {
        let mut grid = Grid::new();

        println!("Grid:\n{:#?}", grid);

        let lines = include_str!("../input-small.txt").lines().collect();

        let instructions = create_instructions(&lines);

        println!("Instructions: {}", instructions.len());

        for ins in instructions {
            println!("Instruction: {:#?}", ins);
            grid.apply(&ins);

            println!("Grid:\n{:#?}", grid);
        }

        assert_eq!(grid.tail_pos.0 as usize, expected_tail_pos_x);
        assert_eq!(grid.tail_pos.1 as usize, expected_tail_pos_y);
    }
}
