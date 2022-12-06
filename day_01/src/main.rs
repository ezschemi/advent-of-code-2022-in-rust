use std::{fs, iter::Sum};
#[derive(Debug)]
struct Elf {
    calories: u32,
}

impl Elf {
    fn new(total_calories: u32) -> Elf {
        Elf {
            calories: total_calories,
        }
    }
}
fn main() {
    let input_filename = String::from("input.txt");

    let content = fs::read_to_string(&input_filename).unwrap();

    let lines = content.lines();

    let mut elves = Vec::new();

    let mut current_calories = 0;

    for l in lines {
        if l.len() == 0 {
            // current elf's calories list is finised, so store it and go to the next one
            let elf = Elf::new(current_calories);

            elves.push(elf);
            current_calories = 0;
        } else {
            let v = l.parse::<u32>().unwrap();

            current_calories += v;
        }
    }

    println!("Got {} elves.", elves.len());

    // C-like
    let mut max_calories = 0;
    for elf in elves.iter() {
        if elf.calories > max_calories {
            max_calories = elf.calories;
        }
    }
    println!("Max calories: {}", max_calories);

    // more Rust-like
    let max_calories = elves.iter().map(|elf| elf.calories).max().unwrap();
    println!("Max calories: {}", max_calories);

    // the easies way would be to sort the vector:
    // this sorts the calories in ascending order
    elves.sort_by(|a, b| a.calories.cmp(&b.calories));

    // println!("elves: {:#?}", elves);
    let max_calories = elves.last().unwrap().calories;
    println!("Max calories: {}", max_calories);

    let mut total_calories = 0;
    for i in 0..3 {
        let calories = elves[elves.len() - i - 1].calories;

        println!("calories: {calories}");

        total_calories += calories;
    }
    println!("Total calories: {}", total_calories);
}
