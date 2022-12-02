use std::fs;

#[cfg(test)]
mod day1 {
    use super::*;

    const TEST_DATA: &str = "1000
    2000
    3000
    
    4000
    
    5000
    6000
    
    7000
    8000
    9000
    
    10000";

    #[test]
    fn test_find_totals() {
        let totals = find_totals(TEST_DATA);
        assert_eq!(24000, *totals.iter().max().unwrap())
    }

    #[test]
    fn test_get_top_three() {
        let totals = find_totals(TEST_DATA);
        let sum_last_three = totals.iter().rev().take(3).sum();

        assert_eq!(45000, sum_last_three)
    }
}

pub fn find_totals(data: &str) -> Vec<i32> {
    let split = data.split("\n");
    let mut totals: Vec<i32> = Vec::new();
    let mut current_total: i32 = 0;

    for s in split {
        if s.trim() == "" {
            totals.push(current_total);
            current_total = 0;

            continue;
        }

        match s.trim().parse::<i32>() {
            Ok(val) => current_total += val,
            Err(err) => panic!("{} ({})", err, s),
        }
    }

    // Handle the last element
    totals.push(current_total);

    totals.sort();

    totals
}

fn main() {
    let input = fs::read_to_string("data/day1.txt").unwrap();

    let totals = find_totals(&input);

    println!("Part 1: {}", *totals.iter().max().unwrap());
    println!("Part 2: {}", totals.iter().rev().take(3).sum::<i32>());
}
