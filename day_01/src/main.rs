use std::fs;

use color_eyre::eyre::Context;

fn read_input(path: String) -> color_eyre::Result<String> {
    let input = fs::read_to_string(path).wrap_err("reading input file")?;
    Ok(input)
}
fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;

    let input_filename = String::from("input.txt");
    let input = read_input(input_filename)?;

    let mut max_calories = 0;

    // the replace() is for correct Windows handling of newlines
    for group in input.replace("\r\n", "\n").split("\n\n") {
        let mut sum = 0;
        for line in group.lines() {
            let value = line.parse::<usize>()?;
            sum += value;
        }
        if sum > max_calories {
            max_calories = sum;
        }
    }

    println!("Max calories: {}", max_calories);

    Ok(())
}
