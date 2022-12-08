use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::fs;

use color_eyre::eyre::Context;

use itertools::FoldWhile;
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
        .batching(|mut it| (&mut it).map_while(|x| x).sum1::<usize>())
        .map(Reverse)
        .k_smallest(3)
        .map(|x| x.0)
        .sum::<usize>();

    Ok(max_calories)
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

fn puzzle_2_with_iterators_coalesce() -> color_eyre::Result<usize> {
    let sum_top_3_calories = include_str!("../input.txt")
        .lines()
        .map(|v| v.parse::<usize>().ok())
        .coalesce(|a, b| match (a, b) {
            (None, None) => Ok(None),
            (None, Some(b)) => Ok(Some(b)),
            (Some(a), Some(b)) => Ok(Some(a + b)),
            (Some(a), None) => Err((Some(a), None)),
        })
        .flatten()
        .sorted_by_key(|&v| std::cmp::Reverse(v))
        .take(3)
        .sum::<usize>();

    Ok(sum_top_3_calories)
}

fn puzzle_2_with_iterators_k_smallest() -> color_eyre::Result<usize> {
    let sum_top_3_calories = include_str!("../input.txt")
        .lines()
        .map(|v| v.parse::<usize>().ok())
        .batching(|it| {
            it.fold_while(None, |acc: Option<usize>, v| match v {
                Some(v) => FoldWhile::Continue(Some(acc.unwrap_or_default() + v)),
                None => FoldWhile::Done(acc),
            })
            .into_inner()
        })
        // this turns k_smallest into k_largest
        .map(Reverse)
        .k_smallest(3)
        // strip off the Reverse to sum up things
        .map(|x| x.0)
        .sum::<usize>();

    Ok(sum_top_3_calories)
}

// this stores only the largest three values at all times
fn puzzle_2_with_binary_heap() -> color_eyre::Result<usize> {
    let mut group_sums = include_str!("../input.txt")
        .lines()
        .map(|v| v.parse::<usize>().ok())
        .batching(|it| {
            it.fold_while(None, |acc: Option<usize>, v| match v {
                Some(v) => FoldWhile::Continue(Some(acc.unwrap_or_default() + v)),
                None => FoldWhile::Done(acc),
            })
            .into_inner()
        })
        .map(Reverse);

    let mut heap = BinaryHeap::new();
    for init in (&mut group_sums).take(3) {
        heap.push(init);
    }
    for rest in group_sums {
        heap.push(rest);
        heap.pop();
    }

    let sum_top_3_calories = heap.into_iter().map(|Reverse(v)| v).sum::<usize>();
    Ok(sum_top_3_calories)
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

    let sum_top_3_calories = puzzle_2_with_iterators_coalesce()?;
    println!("Sum of top 3 calories: {}", sum_top_3_calories);

    let sum_top_3_calories = puzzle_2_with_iterators_k_smallest()?;
    println!("Sum of top 3 calories: {}", sum_top_3_calories);

    let sum_top_3_calories = puzzle_2_with_binary_heap()?;
    println!("Sum of top 3 calories: {}", sum_top_3_calories);

    Ok(())
}
