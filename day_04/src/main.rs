use std::fs;
#[derive(Debug)]
struct SectionAssignments {
    start_0: usize,
    end_0: usize,
    start_1: usize,
    end_1: usize,
}

impl SectionAssignments {
    fn new_from_string(s: String) -> Self {
        let assignment_tokens = s.split(",").collect::<Vec<&str>>();
        if assignment_tokens.len() != 2 {
            panic!(
                "Expected 2 parts, but got {} from:\n{}",
                assignment_tokens.len(),
                s
            );
        }
        let first_assignment_str = assignment_tokens[0];
        let second_assignment_str = assignment_tokens[1];

        let first_section_tokens = first_assignment_str.split("-").collect::<Vec<&str>>();
        let second_section_tokens = second_assignment_str.split("-").collect::<Vec<&str>>();

        if first_section_tokens.len() != 2 {
            panic!(
                "Expected 2 parts, but got {} from:\n{}",
                first_section_tokens.len(),
                first_assignment_str
            );
        }
        if second_section_tokens.len() != 2 {
            panic!(
                "Expected 2 parts, but got {} from:\n{}",
                second_section_tokens.len(),
                second_assignment_str
            );
        }

        let start_0 = first_section_tokens[0].parse().unwrap();
        let end_0 = first_section_tokens[1].parse().unwrap();

        let start_1 = second_section_tokens[0].parse().unwrap();
        let end_1 = second_section_tokens[1].parse().unwrap();

        SectionAssignments {
            start_0,
            end_0,
            start_1,
            end_1,
        }
    }

    pub fn does_one_fully_contain_the_other(&self) -> bool {
        // check if the second section is fully contained within the first section
        if (self.start_1 >= self.start_0 && self.start_1 <= self.end_0)
            && (self.end_1 >= self.start_0 && self.end_1 <= self.end_0)
        {
            return true;
        }
        // check if the first section is fully contained within the second section
        if (self.start_0 >= self.start_1 && self.start_0 <= self.end_1)
            && (self.end_0 >= self.start_1 && self.end_0 <= self.end_1)
        {
            return true;
        }
        false
    }
}
fn main() {
    let lines = vec![
        "2-4,6-8", "2-3,4-5", "5-7,7-9", "2-8,3-7", "6-6,4-6", "2-6,4-8",
    ];

    let input_filename = String::from("input.txt");

    let content = fs::read_to_string(&input_filename).unwrap();

    let lines = content.lines();

    let mut assignments = Vec::new();

    for line in lines {
        assignments.push(SectionAssignments::new_from_string(line.to_string()));
    }

    // println!("Assignments: {:#?}", assignments);
    println!("Assignments: {}", assignments.len());

    let mut n_sections_fully_contained = 0;
    for a in assignments {
        if a.does_one_fully_contain_the_other() {
            n_sections_fully_contained += 1;
            // println!("{:#?}", a);
        }
    }
    // let n_sections_fully_contained: usize = assignments
    //     .iter()
    //     .filter(|a| a.does_one_fully_contain_the_other())
    //     .sum();

    println!("n_sections_fully_contained: {}", n_sections_fully_contained);
}
