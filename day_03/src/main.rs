use std::fs;

#[derive(Debug)]
struct Rucksack {
    compartment_0: String,
    compartment_1: String,
    common_item: char,
    priority: usize,
}

#[derive(Debug)]
struct ElfGroup<'a> {
    r0: &'a Rucksack,
    r1: &'a Rucksack,
    r2: &'a Rucksack,
    common_item: char,
    priority: usize,
}

fn determine_common_item(r0: &Rucksack, r1: &Rucksack, r2: &Rucksack) -> char {
    let c_0 = r0.compartment_0.clone() + &r0.compartment_1;
    let c_1 = r1.compartment_0.clone() + &r1.compartment_1;
    let c_2 = r2.compartment_0.clone() + &r2.compartment_1;

    let mut common_item = ' ';
    for item_0 in c_0.chars() {
        for item_1 in c_1.chars() {
            for item_2 in c_2.chars() {
                if item_0 == item_1 && item_1 == item_2 {
                    common_item = item_0;
                    break;
                }
            }
        }
    }

    if common_item == ' ' {
        panic!("Did not find a common_item with:\n{c_0}\n{c_1}\n{c_2}\n");
    }

    common_item
}

impl ElfGroup<'_> {
    pub fn new<'a>(r0: &'a Rucksack, r1: &'a Rucksack, r2: &'a Rucksack) -> ElfGroup<'a> {
        let common_item = determine_common_item(r0, r1, r2);
        let priority = get_priority(common_item);

        ElfGroup {
            r0,
            r1,
            r2,
            common_item,
            priority,
        }
    }

    pub fn get_priority(&self) -> usize {
        self.priority
    }
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
    let mut elf_groups = Vec::new();

    for line in lines {
        rucksacks.push(Rucksack::new(line.to_string()));
    }

    for i in 3..=rucksacks.len() {
        if i % 3 == 0 {
            println!("i = {}", i);
            let elf_group = ElfGroup::new(&rucksacks[i - 3], &rucksacks[i - 2], &rucksacks[i - 1]);

            elf_groups.push(elf_group);
        }
    }

    println!(
        "Elf Groups: {:?} - should be {}",
        elf_groups.len(),
        rucksacks.len() / 3
    );

    println!("Elf Groups: {:#?}", elf_groups);

    println!("Rucksacks: {}", rucksacks.len());

    // println!("Rucksacks: {:#?}", rucksacks);

    let sum_priorities: usize = rucksacks.iter().map(|r| r.get_priority()).sum();
    println!("Sum of priorities: {}", sum_priorities);

    let sum_group_priorities: usize = elf_groups.iter().map(|g| g.get_priority()).sum();
    println!("Sum of group priorities: {}", sum_group_priorities);
}
