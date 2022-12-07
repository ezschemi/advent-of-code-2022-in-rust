use std::fs;

fn are_chars_the_same(c0: char, c1: char, c2: char, c3: char) -> bool {
    let mut v = vec![c0, c1, c2, c3];

    v.sort();
    v.dedup();

    v.len() != 4
}
fn find_start_of_packet_marker(s: &str) -> usize {
    if s.len() < 4 {
        panic!(
            "Passed in string should have at least 4 characters, but got:\n{}",
            s
        );
    }

    let chars: Vec<char> = s.chars().collect();

    let mut current_slice_begin = 0;

    let max_begin_index = s.len() - 4;

    // println!("string length: {}", s.len());
    // println!("max begin index: {max_begin_index}");

    while current_slice_begin <= max_begin_index {
        let c0 = chars[current_slice_begin + 0];
        let c1 = chars[current_slice_begin + 1];
        let c2 = chars[current_slice_begin + 2];
        let c3 = chars[current_slice_begin + 3];

        if !are_chars_the_same(c0, c1, c2, c3) {
            // current_slice_begin now contains the *beginning* of the marker,
            // *not* the start of the data packet. The marker is 4 characters.
            return current_slice_begin + 4;
        }

        current_slice_begin += 1;
    }

    panic!("No start of packet marker was found in this:\n{}", s);
}
fn main() {
    let lines = vec![
        "bvwbjplbgvbhsrlpgdmjqwftvncz",
        "nppdvjthqldpwncqszvftbrmjlhg",
        "nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg",
        "zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw",
    ];
    let input_filename = String::from("input.txt");

    let content = fs::read_to_string(&input_filename).unwrap();

    let lines = content.lines();

    for line in lines {
        let start_of_packet = find_start_of_packet_marker(line);

        println!("Start of Packet for {}: {}", line, start_of_packet);
    }
}
