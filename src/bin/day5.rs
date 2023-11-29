use std::{
    fs,
    collections::VecDeque
};
use regex::Regex;

#[cfg(test)]
mod day5 {
    use super::*;
    
    const TEST_DATA: &str = "    [D]    
[N] [C]    
[Z] [M] [P]
 1   2   3 

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2";

    #[test]
    fn test_parse_representation() {
        let parsed = parse_representation(TEST_DATA);

        // Three columns
        assert_eq!(3, parsed.len());
        assert_eq!(parsed[0], ['N', 'Z']);
        assert_eq!(parsed[1], ['D', 'C', 'M']);
        assert_eq!(parsed[2], ['P']);
    }

    #[test]
    fn test_parse_operations() {
        let mut data = parse_representation(TEST_DATA);
        operate(TEST_DATA, &mut data, false);

        assert_eq!(data[0], ['C']);
        assert_eq!(data[1], ['M']);
        assert_eq!(data[2], ['Z', 'N', 'D', 'P']);
    }

    #[test]
    fn test_parse_multi_operatios() {
        let mut data = parse_representation(TEST_DATA);
        operate(TEST_DATA, &mut data, true);

        assert_eq!(data[0], ['M']);
        assert_eq!(data[1], ['C']);
        assert_eq!(data[2], ['D', 'N', 'Z', 'P']);
    }

    #[test]
    fn test_part_1() {
        let mut data = parse_representation(TEST_DATA);
        operate(TEST_DATA, &mut data, false);

        assert_eq!("CMZ", get_tops(data));
    }
}

pub fn parse_representation(data: &str) -> Vec<VecDeque<char>> {
    let mut representation: Vec<VecDeque<char>> = Vec::new();
    
    for line in data.lines() {
        if !line.contains("[") {
            break;
        }

        let elems = line.chars();
        
        for (bucket, elem) in elems.collect::<Vec<char>>().chunks(4).enumerate() {
            let character = elem.get(1).unwrap();
            for index in 0..bucket+1 {
                if representation.get(index).is_none() {
                    representation.push(VecDeque::new());
                }
            }

            if character != &' ' {
                representation[bucket].push_back(character.clone());
            }
        }
    }
    
    representation
}

pub fn operate(commands: &str, data: &mut Vec<VecDeque<char>>, can_handle_multiple: bool) {
    let re = Regex::new(r"move (?P<count>\d*) from (?P<src>\d) to (?P<dst>\d)").unwrap();
    for line in commands.lines() {
        if ! re.is_match(line) {  continue; }
        let caps = re.captures(line).unwrap();
        
        let count = caps.name("count").unwrap().as_str().parse::<u32>().unwrap();
        let src = caps.name("src").unwrap().as_str().parse::<usize>().unwrap() - 1;
        let dst = caps.name("dst").unwrap().as_str().parse::<usize>().unwrap() - 1;


        if can_handle_multiple {
            let mut elems: VecDeque<char> = VecDeque::new();
            for _ in 0..count {
                if let Some(item) = data[src].pop_front() {
                    elems.push_back(item);
                }
            }
            
            elems
            .iter()
            .rev()
            .for_each(|it| data[dst].push_front(it.clone()));
        } else {
            for _ in 0..count {
                if let Some(item) = data[src].pop_front() {
                    data[dst].push_front(item);
                }
            }
        }
    }
}

pub fn get_tops(data: Vec<VecDeque<char>>) -> String {
    data.into_iter()
        .filter(|col| !col.is_empty())
        .map(|col| col.front().unwrap().clone())
        .collect::<String>()
}

fn main() {
    let input = fs::read_to_string("data/day5.txt").unwrap();

    let mut data = parse_representation(&input);
    operate(&input, &mut data, false);

    let mut part2data = parse_representation(&input);
    operate(&input, &mut part2data, true);

    println!("Part 1: {}", get_tops(data));
    println!("Part 2: {}", get_tops(part2data));
}
