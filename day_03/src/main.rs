use std::fs;

#[derive(Debug)]
struct Rucksack {
    compartment_0: String,
    compartment_1: String,
    common_item: char,
    priority: usize,
}

pub fn get_priority(c: char) -> usize {
    if 'A' <= c && c <= 'Z' {
        c as usize - 'A' as usize + 27
    } else {
        c as usize - 'a' as usize + 1
    }
}

impl Rucksack {
    pub fn new(s: String) -> Self {
        let l = s.len();

        let compartment_0 = s[0..l / 2].to_string();
        let compartment_1 = s[l / 2..].to_string();

        let mut common_item = ' ';

        for item_0 in compartment_0.chars() {
            for item_1 in compartment_1.chars() {
                if item_0 == item_1 {
                    common_item = item_0;
                    break;
                }
            }
        }

        if common_item == ' ' {
            panic!("No common item was found with {compartment_0} and {compartment_1}");
        }

        Rucksack {
            compartment_0,
            compartment_1,
            common_item,
            priority: get_priority(common_item),
        }
    }
    pub fn get_priority(&self) -> usize {
        self.priority
    }
}
fn main() {
    let lines = vec![
        "vJrwpWtwJgWrhcsFMMfFFhFp",
        "jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL",
        "PmmdzqPrVvPwwTWBwg",
        "wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn",
        "ttgJtRGJQctTZtZT",
        "CrZsJsPPZsGzwwsLwLmpwMDw",
    ];

    let input_filename = String::from("input.txt");

    let content = fs::read_to_string(&input_filename).unwrap();

    let lines = content.lines();

    let mut rucksacks = Vec::new();
    for line in lines {
        rucksacks.push(Rucksack::new(line.to_string()));
    }

    println!("Rucksacks: {}", rucksacks.len());

    // println!("Rucksacks: {:#?}", rucksacks);

    let sum_priorities: usize = rucksacks.iter().map(|r| r.get_priority()).sum();
    println!("Sum of priorities: {}", sum_priorities);
}
