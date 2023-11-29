use std::fs;
use itertools::Itertools;

#[cfg(test)]
mod day6 {
    use super::*;
    
    #[test]
    fn test_get_first_marker() {
        assert_eq!(5, get_first_marker(4, "bvwbjplbgvbhsrlpgdmjqwftvncz"));
        assert_eq!(6, get_first_marker(4, "nppdvjthqldpwncqszvftbrmjlhg"));
        assert_eq!(10, get_first_marker(4, "nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg"));
        assert_eq!(11, get_first_marker(4, "zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw"));
    }

    #[test]
    fn test_part_2() {
        assert_eq!(19, get_first_marker(14, "mjqjpqmgbljsphdztnvjfqwrcgsmlb"));
        assert_eq!(23, get_first_marker(14, "bvwbjplbgvbhsrlpgdmjqwftvncz"));
        assert_eq!(23, get_first_marker(14, "nppdvjthqldpwncqszvftbrmjlhg"));
        assert_eq!(29, get_first_marker(14, "nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg"));
        assert_eq!(26, get_first_marker(14, "zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw"));
    }
}

pub fn get_first_marker(window_size: usize, stream: &str) -> usize {
    for (idx, _) in stream.chars().enumerate() {
        if idx < window_size { continue; }
        let window = &stream[(idx-window_size)..idx];
        let unique_window_len = window
            .chars()
            .unique()
            .collect::<Vec<char>>()
            .len();

        if unique_window_len == window_size { return idx; }
    }

    0
}

fn main() {
    let input = fs::read_to_string("data/day6.txt").unwrap();

    println!("Part 1: {}", get_first_marker(4, &input));
    println!("Part 2: {}", get_first_marker(14, &input));
}
