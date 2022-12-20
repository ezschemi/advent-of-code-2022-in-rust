use std::{
    collections::{HashMap, HashSet},
    fmt,
};

use svg::Document;
use wasm_bindgen::prelude::*;

#[derive(Clone, Copy, Debug, PartialEq)]
enum HeightMapCell {
    Start,
    End,
    Square(u8),
}

impl HeightMapCell {
    fn get_elevation(self) -> u8 {
        match self {
            HeightMapCell::Start => 0,
            HeightMapCell::End => 25,
            HeightMapCell::Square(e) => e,
        }
    }
}

struct HeightMapCellRecord {
    prev: Option<GridCoord>,
}

#[wasm_bindgen]
pub struct HeightmapGrid {
    width: usize,
    height: usize,
    cells: Vec<HeightMapCell>,
    visited: HashMap<GridCoord, HeightMapCellRecord>,
    current: HashSet<GridCoord>,
    num_steps: usize,
}
#[wasm_bindgen]
impl HeightmapGrid {
    #[wasm_bindgen(constructor)]
    pub fn parse(input: &str) -> Self {
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
            visited: Default::default(),
            current: Default::default(),
            num_steps: 0,
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

    fn walkable_neighbors(&self, coord: GridCoord) -> impl Iterator<Item = GridCoord> + '_ {
        let current_elevation = self.get_cell(coord).unwrap().get_elevation();

        let deltas: [(isize, isize); 4] = [(-1, 0), (1, 0), (0, -1), (0, 1)];

        let x: isize = coord.x as isize;
        let y: isize = coord.y as isize;

        deltas.into_iter().filter_map(move |(dx, dy)| {
            Some(GridCoord {
                x: (x + dx) as usize,
                y: (y + dy) as usize,
            })
            .filter(|&coord| self.in_bounds(coord))
            .filter(|&coord| {
                let other_elevation = self.get_cell(coord).unwrap().get_elevation();
                other_elevation <= current_elevation + 1
            })
        })
    }

    #[wasm_bindgen]
    pub fn step_breadth_first(&mut self) {
        if self.current.is_empty() {
            let mut start_coord: Option<GridCoord> = None;
            for y in 0..self.height {
                for x in 0..self.width {
                    let coord: GridCoord = (x, y).into();
                    if let HeightMapCell::Start = self.get_cell(coord).unwrap() {
                        start_coord = Some(coord);
                        break;
                    }
                }
            }
            let start_coord = start_coord.unwrap();
            self.current.insert(start_coord);
            self.visited
                .insert(start_coord, HeightMapCellRecord { prev: None });
            return;
        }

        let current = std::mem::take(&mut self.current);
        let mut next = HashSet::new();
        let mut visited = std::mem::take(&mut self.visited);

        for curr in current {
            for ncoord in self.walkable_neighbors(curr) {
                if visited.contains_key(&ncoord) {
                    // been here already
                    continue;
                }
                visited.insert(ncoord, HeightMapCellRecord { prev: Some(curr) });
                next.insert(ncoord);
            }
        }
        self.current = next;
        self.visited = visited;
        self.num_steps += 1;
    }

    pub fn is_at_end(&self) -> bool {
        for coord in &self.current {
            let cell = self.get_cell(*coord).unwrap();

            if HeightMapCell::End == *cell {
                return true;
            }
        }
        false
    }

    #[wasm_bindgen]
    pub fn num_visited(&self) -> usize {
        self.visited.len()
    }

    #[wasm_bindgen]
    pub fn num_cells(&self) -> usize {
        self.width * self.height
    }

    #[wasm_bindgen]
    pub fn num_steps(&self) -> usize {
        self.num_steps
    }

    #[wasm_bindgen]
    pub fn to_svg(&self) -> String {
        const SIDE: usize = 64;
        let side = SIDE as f32;

        assert!(self.width <= SIDE);
        assert!(self.height <= SIDE);

        let mut document =
            Document::new().set("viewBox", (0, 0, self.width * SIDE, self.height * SIDE));

        for y in 0..self.height {
            for x in 0..self.width {
                let cell = self.get_cell((x, y).into()).unwrap();
                let (title, r, g, b) = match cell {
                    HeightMapCell::Start => ("start".to_string(), 27, 216, 96),
                    HeightMapCell::End => ("end".to_string(), 216, 27, 96),
                    HeightMapCell::Square(elevation) => {
                        let title = format!("elevation {elevation}");
                        // "25.0" because there are 25 characters in the ASCII lower-case alphabet
                        let elevation = *elevation as f32 / 25.0;
                        let f = (elevation * 180.0) as u8;
                        (title, f, f, f)
                    }
                };

                let rectangle = svg::node::element::Rectangle::new()
                    .set("x", x * SIDE)
                    .set("y", y * SIDE)
                    .set("width", SIDE)
                    .set("height", SIDE)
                    .set("fill", format!("rgb({r}, {g}, {b})"))
                    .set("stroke", "white")
                    .set("stroke-width", "2px")
                    .add(svg::node::element::Title::new().add(svg::node::Text::new(title)));

                document = document.add(rectangle);
            }
        }

        let definitions = svg::node::element::Definitions::new().add(
            svg::node::element::Marker::new()
                .set("id", "arrowhead")
                .set("markerWidth", 10)
                .set("markerHeight", 7)
                .set("refX", 10)
                .set("refY", 3.5)
                .set("orient", "auto")
                .add(
                    svg::node::element::Polygon::new()
                        .set("points", "0 0, 10 3.5, 0 7")
                        .set("fill", "#ffc107"),
                ),
        );
        document = document.add(definitions);

        for coord in self.visited.keys() {
            let circle = svg::node::element::Circle::new()
                .set("cx", (coord.x as f32 + 0.5) * side)
                .set("cy", (coord.y as f32 + 0.5) * side)
                .set("r", side * 0.1)
                .set("fill", "#fff");
            document = document.add(circle);
        }

        for coord in self.current.iter() {
            let circle = svg::node::element::Circle::new()
                .set("cx", (coord.x as f32 + 0.5) * side)
                .set("cy", (coord.y as f32 + 0.5) * side)
                .set("r", side * 0.1)
                .set("fill", "#ffc107");
            document = document.add(circle);

            let record = self.visited.get(coord).unwrap();
            let mut curr = record;
            let mut coord = *coord;
            while let Some(prev) = curr.prev.as_ref() {
                curr = self.visited.get(prev).unwrap();

                let (x, y) = (prev.x as f32, prev.y as f32);
                let dx = coord.x as f32 - x;
                let dy = coord.y as f32 - y;

                let line = svg::node::element::Line::new()
                    .set("x1", (x + 0.5 + dx * 0.2) * side)
                    .set("y1", (y + 0.5 + dy * 0.2) * side)
                    .set("x2", (x + 0.5 + dx * 0.8) * side)
                    .set("y2", (y + 0.5 + dy * 0.8) * side)
                    .set("stroke", "#ffc107")
                    .set("stroke-width", "1.5px")
                    .set("marker-end", "url(#arrowhead)");
                document = document.add(line);

                coord = *prev;
            }
        }

        // for y in 0..self.height {
        //     for x in 0..self.width {
        //         let coord: GridCoord = (x, y).into();
        //         for ncoord in self.walkable_neighbors(coord) {
        //             let (x, y) = (x as f32, y as f32);
        //             let dx = ncoord.x as f32 - x;
        //             let dy = ncoord.y as f32 - y;

        //             let line = svg::node::element::Line::new()
        //                 .set("x1", (x + 0.5 + dx * 0.05) * side)
        //                 .set("y1", (y + 0.5 + dy * 0.05) * side)
        //                 .set("x2", (x + 0.5 + dx * 0.45) * side)
        //                 .set("y2", (y + 0.5 + dy * 0.45) * side)
        //                 .set("stroke", "#ffc107")
        //                 .set("stroke-width", "1px")
        //                 .set("marker-end", "url(#arrowhead)");
        //             document = document.add(line);
        //         }
        //     }
        // }

        document.to_string()
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
