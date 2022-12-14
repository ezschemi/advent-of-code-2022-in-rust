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

        grid.set(head_pos.0 as usize, head_pos.1 as usize, GridPosition::Head);
        grid.set(tail_pos.0 as usize, tail_pos.1 as usize, GridPosition::Tail);

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
        }
    }
}

fn main() {
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
}

#[cfg(test)]
mod tests {
    use crate::create_instructions;
    use crate::Grid;
    use test_case::test_case;

    #[test_case(2, 2)]
    fn test_head_move_instructions(expected_head_pos_x: usize, expected_head_pos_y: usize) {
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
}
