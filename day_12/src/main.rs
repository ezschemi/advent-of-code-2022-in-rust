use day_12::HeightmapGrid;

fn main() {
    let mut grid = HeightmapGrid::parse(include_str!("../input.txt"));

    // println!("{grid:?}");

    while !grid.is_at_end() {
        grid.step_breadth_first();
    }

    println!("Steps: {}", grid.num_steps());

    let mut grid = HeightmapGrid::parse(include_str!("../input.txt"));

    // println!("{grid:?}");

    while !grid.is_at_elevation_0() {
        grid.step_breadth_first_reverse();
    }

    println!("Steps: {}", grid.num_steps());
}
