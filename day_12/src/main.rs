use day_12::HeightmapGrid;

fn main() {
    let grid = HeightmapGrid::parse(include_str!("../input-sample.txt"));

    println!("{grid:?}");
}
