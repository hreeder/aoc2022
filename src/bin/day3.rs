use std::fs;

#[cfg(test)]
mod day3 {
    use super::*;

    const TEST_DATA: &str = "vJrwpWtwJgWrhcsFMMfFFhFp
    jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
    PmmdzqPrVvPwwTWBwg
    wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
    ttgJtRGJQctTZtZT
    CrZsJsPPZsGzwwsLwLmpwMDw";

    #[test]
    fn test_get_pairs() {
        let mut lines = TEST_DATA.lines();

        let first = lines.next().unwrap();
        let (left, right) = get_pair(first);
        assert_eq!("vJrwpWtwJgWr", left);
        assert_eq!("hcsFMMfFFhFp", right);
    }

    #[test]
    fn test_get_common() {
        let mut lines = TEST_DATA.lines();

        for target in vec!['p', 'L', 'P', 'v', 't', 's'] {
            let line = lines.next().unwrap();
            let (left, right) = get_pair(line);
            let common = get_common(left, right);
            assert_eq!(target, common);
        }
    }

    #[test]
    fn test_get_common_triple() {
        let mut lines = TEST_DATA.lines();

        for target in vec!['r', 'Z'] {
            let first = lines.next().unwrap();
            let second = lines.next().unwrap();
            let third = lines.next().unwrap();

            let common = get_common_triple(first, second, third);

            assert_eq!(target, common);
        }
    }

    #[test]
    fn test_get_priority() {
        assert_eq!(1, get_priority('a'));
        assert_eq!(26, get_priority('z'));
        assert_eq!(27, get_priority('A'));
        assert_eq!(52, get_priority('Z'));
    }

    #[test]
    fn test_get_sum() {
        assert_eq!(157, get_sum(TEST_DATA));
    }

    #[test]
    fn test_get_sum_stickers() {
        assert_eq!(70, get_sum_stickers(TEST_DATA));
    }
}

pub fn get_pair(line: &str) -> (&str, &str) {
    line.split_at(line.len()/2)
}

pub fn get_common<'a>(left: &'a str, right: &'a str) -> char {
    let left_chars: Vec<char> = left.chars().collect();
    let right_chars: Vec<char> = right.chars().collect();

    let mut found: char = '.';


    for character in left_chars {
        if right_chars.contains(&character) {
            found = character;
        }
    }

    found
}

pub fn get_common_triple<'a>(first: &'a str, second: &'a str, third: &'a str) -> char {
    let first_chars: Vec<char> = first.chars().collect();
    let second_chars: Vec<char> = second.chars().collect();
    let third_chars: Vec<char> = third.chars().collect();

    let mut found: char = '.';

    for character in first_chars {
        if second_chars.contains(&character) && third_chars.contains(&character) {
            found = character;
        }
    }

    found
}

pub fn get_priority(char: char) -> u32 {
    let mut codepoint = char as u32;
    codepoint = codepoint - 38;

    if char.is_ascii_lowercase() {
        codepoint = codepoint - 58;
    }

    codepoint
}

pub fn get_sum(data: &str) -> u32 {
    data.trim()
        .lines()
        .into_iter()
        .map(|line| get_pair(line))
        .map(|(left, right)| get_common(left, right))
        .map(|common| get_priority(common))
        .sum()
}

pub fn get_sum_stickers(data: &str) -> u32 {
    let mut lines = data.trim().lines();
    
    let mut counter = 0;
    
    loop {
        let first = lines.next();
        if first.is_none() { break; }

        let second = lines.next().unwrap();
        let third = lines.next().unwrap();

        let common = get_common_triple(first.unwrap(), second, third);
        counter += get_priority(common);
    }
    
    counter
}

fn main() {
    let input = fs::read_to_string("data/day3.txt").unwrap();

    println!("Part 1: {}", get_sum(&input));
    println!("Part 2: {}", get_sum_stickers(&input));
}
