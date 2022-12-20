use std::fmt;

#[derive(Clone, Copy, Debug)]
enum HeightMapCell {
    Start,
    End,
    Square(u8),
}

struct HeightmapGrid {
    width: usize,
    height: usize,
    cells: Vec<HeightMapCell>,
}

impl HeightmapGrid {
    fn parse(input: &str) -> Self {
        let first_line = input.lines().next().unwrap();
        let width = first_line.len();
        let height = input.lines().count();

        let mut cells = vec![];

        for c in input.chars() {
            let cell = match c {
                'S' => HeightMapCell::Start,
                'E' => HeightMapCell::End,
                'a'..='z' => HeightMapCell::Square(c as u8 - b'a'),
                '\r' | '\n' => continue,
                _ => panic!("Invalid Character: {c}"),
            };
            cells.push(cell);
        }

        HeightmapGrid {
            width,
            height,
            cells,
        }
    }

    fn in_bounds(&self, coord: GridCoord) -> bool {
        coord.x < self.width && coord.y < self.height
    }

    fn get_cell(&self, coord: GridCoord) -> Option<&HeightMapCell> {
        if !self.in_bounds(coord) {
            return None;
        }

        Some(&self.cells[coord.y * self.width + coord.x])
    }

    fn get_cell_mut(&mut self, coord: GridCoord) -> Option<&HeightMapCell> {
        if !self.in_bounds(coord) {
            return None;
        }

        Some(&self.cells[coord.y * self.width + coord.x])
    }
}

impl fmt::Debug for HeightmapGrid {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "{}x{} Grid: ", self.width, self.height)?;

        for y in 0..self.height {
            for x in 0..self.width {
                let cell = self.get_cell((x, y).into()).unwrap();
                let c = match cell {
                    HeightMapCell::Start => 'S',
                    HeightMapCell::End => 'E',
                    HeightMapCell::Square(elevation) => (b'a' + elevation) as char,
                };
                write!(f, "{c}")?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
struct GridCoord {
    x: usize,
    y: usize,
}

impl From<(usize, usize)> for GridCoord {
    fn from((x, y): (usize, usize)) -> Self {
        Self { x, y }
    }
}

fn main() {
    let grid = HeightmapGrid::parse(include_str!("../input-sample.txt"));

    println!("{grid:?}");
}
