use itertools::Itertools;
use std::collections::HashSet;
use std::fs;

// returns true if at least one character is found multiple times in v
fn contains_duplicate_chars(v: &[char]) -> bool {
    let l = v.len();

    for i in 0..l {
        for j in 0..l {
            if i == j {
                // same character, dont look at this one
                continue;
            }

            if v[i] == v[j] {
                return true;
            }
        }
    }

    return false;
}
fn find_marker_by_distinct_chars(s: &str, n_distinct_chars: usize) -> usize {
    if s.len() < n_distinct_chars {
        panic!(
            "Passed in string should have at least {} characters, but got:\n{}",
            n_distinct_chars, s
        );
    }

    let chars: Vec<char> = s.chars().collect();

    let mut current_slice_begin = 0;

    let max_begin_index = s.len() - n_distinct_chars;

    // println!("string length: {}", s.len());
    // println!("max begin index: {max_begin_index}");

    while current_slice_begin <= max_begin_index {
        let v = &chars[current_slice_begin..current_slice_begin + n_distinct_chars];

        if !contains_duplicate_chars(v) {
            // current_slice_begin now contains the *beginning* of the marker,
            // *not* the start of the data packet. The marker is n characters.
            return current_slice_begin + n_distinct_chars;
        }

        current_slice_begin += 1;
    }

    panic!("No start of packet marker was found in this:\n{}", s);
}
fn find_start_of_packet_marker(s: &str) -> usize {
    find_marker_by_distinct_chars(s, 4)
}

fn find_start_of_message_marker(s: &str) -> usize {
    find_marker_by_distinct_chars(s, 14)
}

fn imperative_style() -> color_eyre::Result<()> {
    let lines_start_of_packets = vec![
        "bvwbjplbgvbhsrlpgdmjqwftvncz",
        "nppdvjthqldpwncqszvftbrmjlhg",
        "nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg",
        "zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw",
    ];
    let lines_start_of_messages = vec![
        "mjqjpqmgbljsphdztnvjfqwrcgsmlb",
        "bvwbjplbgvbhsrlpgdmjqwftvncz",
        "nppdvjthqldpwncqszvftbrmjlhg",
        "nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg",
        "zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw",
    ];
    let input_filename = String::from("input.txt");
    let content = fs::read_to_string(&input_filename).unwrap();
    let lines_start_of_packets = content.lines();
    let lines_start_of_messages = lines_start_of_packets.clone();

    for line in lines_start_of_packets {
        let start_of_packet = find_start_of_packet_marker(line);

        println!("Start of Packet for {}: {}", line, start_of_packet);
    }

    for line in lines_start_of_messages {
        let start_of_message = find_start_of_message_marker(line);

        println!("Start of Message for {}: {}", line, start_of_message);
    }

    Ok(())
}

fn find_marker(input: &str, window_size: usize) -> Option<usize> {
    input
        .as_bytes()
        .windows(window_size)
        .position(|window| window.iter().unique().count() == window_size)
        .map(|pos| pos + window_size)
}

fn functional_style() -> color_eyre::Result<()> {
    let marker = find_marker(include_str!("../input.txt"), 4).unwrap();
    println!("Marker: {marker}");
    Ok(())
}

fn main() -> color_eyre::Result<()> {
    imperative_style()?;

    functional_style()?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::find_marker;
    use test_case::test_case;

    #[test_case(7, "mjqjpqmgbljsphdztnvjfqwrcgsmlb", 4)]
    #[test_case(5, "bvwbjplbgvbhsrlpgdmjqwftvncz", 4)]
    #[test_case(6, "nppdvjthqldpwncqszvftbrmjlhg", 4)]
    #[test_case(10, "nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg", 4)]
    #[test_case(11, "zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw", 4)]
    fn test_find_marker(index: usize, input: &str, window_size: usize) {
        assert_eq!(Some(index), find_marker(input, window_size));
    }
}
