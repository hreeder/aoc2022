use std::{fs, ops::Range};

#[cfg(test)]
mod day4 {
    use super::*;

    const TEST_DATA: &str = "2-4,6-8
    2-3,4-5
    5-7,7-9
    2-8,3-7
    6-6,4-6
    2-6,4-8";

    #[test]
    fn test_get_ranges() {
        let mut lines = TEST_DATA.lines();
        let expected = vec![
            ((2..5), (6..9)),
            ((2..4), (4..6)),
            ((5..8), (7..10)),
            ((2..9), (3..8)),
            ((6..7), (4..7)),
            ((2..7), (4..9)),
        ];

        for (left_exp, right_exp) in expected {
            let line = lines.next().unwrap();
            let (left, right) = get_ranges(line);
            assert_eq!(left_exp, left);
            assert_eq!(right_exp, right);
        }
    }

    #[test]
    fn test_range_overlap() {
        assert_eq!(false, ranges_overlap(&(2..5), &(6..9)));
        assert_eq!(false, ranges_overlap(&(2..4), &(4..6)));
        assert_eq!(true, ranges_overlap(&(5..8), &(7..10)));
        assert_eq!(true, ranges_overlap(&(2..9), &(3..8)));
        assert_eq!(true, ranges_overlap(&(6..7), &(4..7)));
    }

    #[test]
    fn test_range_overlap_entirely() {
        assert_eq!(false, ranges_overlap_entirely(&(2..5), &(6..9)));
        assert_eq!(false, ranges_overlap_entirely(&(5..8), &(7..10)));
        assert_eq!(true, ranges_overlap_entirely(&(2..9), &(3..8)));
        assert_eq!(true, ranges_overlap_entirely(&(6..7), &(4..7)));
    }

    #[test]
    fn test_part_1() {
        assert_eq!(2, count_overlaps(TEST_DATA, ranges_overlap_entirely))
    }

    #[test]
    fn test_part_2() {
        assert_eq!(4, count_overlaps(TEST_DATA, ranges_overlap));
    }
}

pub fn count_overlaps(data: &str, filter_fn: fn(&Range<i32>, &Range<i32>) -> bool) -> u32 {
    data.trim()
        .lines()
        .into_iter()
        .map(|line| get_ranges(line.trim()))
        .filter(|(left, right)| filter_fn(left, right))
        .count()
        .try_into()
        .unwrap()
}

pub fn ranges_overlap_entirely(left: &Range<i32>, right: &Range<i32>) -> bool {
    if left.start >= right.start && left.end <= right.end {
        return true;
    } else if right.start >= left.start && right.end <= left.end {
        return true;
    }

    false
}

pub fn ranges_overlap(left: &Range<i32>, right: &Range<i32>) -> bool {
    if left.start >= right.start && left.start <= (right.end-1) {
        return true;
    } else if right.start >= left.start && right.start <= (left.end-1) {
        return true;
    }

    false
}

pub fn get_ranges(line: &str) -> (Range<i32>, Range<i32>) {
    let (left, right) = line.trim().split_once(",").unwrap();

    (get_range_partial(left), get_range_partial(right))
}

fn get_range_partial(partial: &str) -> Range<i32> {
    let (lower_s, upper_s) = partial.split_once("-").unwrap();
    let lower = lower_s.parse::<i32>().unwrap();
    let upper = (upper_s.parse::<i32>().unwrap()) + 1;

    lower..upper
}

fn main() {
    let input = fs::read_to_string("data/day4.txt").unwrap();

    println!("Part 1: {}", count_overlaps(&input, ranges_overlap_entirely));
    println!("Part 2: {}", count_overlaps(&input, ranges_overlap));
}