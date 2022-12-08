use std::fs;

use color_eyre::eyre::Context;

use itertools::Itertools;

fn read_input(path: String) -> color_eyre::Result<String> {
    let input = fs::read_to_string(path).wrap_err("reading input file")?;
    Ok(input)
}

fn iterative_style() -> color_eyre::Result<usize> {
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

    Ok(max_calories)
}

fn more_functional_style() -> color_eyre::Result<usize> {
    let lines = include_str!("../input.txt")
        .lines()
        .map(|v| v.parse::<usize>().ok())
        .collect::<Vec<_>>();
    let max_calories = lines
        .split(|line| line.is_none())
        .map(|group| group.iter().map(|v| v.unwrap()).sum::<usize>())
        .max()
        .unwrap();

    Ok(max_calories)
}

// in more_functional_style(), the collect() potentially uses a lot of memory.
// To avoid that, use iterators instead.
fn with_iterators_batching() -> color_eyre::Result<usize> {
    let max_calories = include_str!("../input.txt")
        .lines()
        .map(|v| v.parse::<usize>().ok())
        .batching(|it| {
            let mut sum = None;
            while let Some(Some(v)) = it.next() {
                sum = Some(sum.unwrap_or(0) + v);
            }
            sum
        })
        .max();

    Ok(max_calories.unwrap())
}
fn with_iterators_coalesce() -> color_eyre::Result<usize> {
    let max_calories = include_str!("../input.txt")
        .lines()
        .map(|v| v.parse::<usize>().ok())
        .coalesce(|a, b| match (a, b) {
            (None, None) => Ok(None),
            (None, Some(b)) => Ok(Some(b)),
            (Some(a), Some(b)) => Ok(Some(a + b)),
            (Some(a), None) => Err((Some(a), None)),
        })
        .max()
        .flatten()
        .unwrap_or_default();

    Ok(max_calories)
}
fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;

    let max_calories = more_functional_style()?;
    println!("Max calories: {}", max_calories);

    let max_calories = iterative_style()?;
    println!("Max calories: {}", max_calories);

    let max_calories = with_iterators_batching()?;
    println!("Max calories: {}", max_calories);

    let max_calories = with_iterators_coalesce()?;
    println!("Max calories: {}", max_calories);

    Ok(())
}
