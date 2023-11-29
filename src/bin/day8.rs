use std::fs;

#[cfg(test)]
mod day8 {
    use super::*;
    
    const TEST_DATA: &str = "30373
    25512
    65332
    33549
    35390";

    #[test]
    fn test_parse_grid() {
        let grid: Vec<Vec<u32>> = parse_grid(TEST_DATA);

        assert_eq!(grid[0], vec![3, 0, 3, 7, 3]);
        assert_eq!(grid[1], vec![2, 5, 5, 1, 2]);
        assert_eq!(grid[2], vec![6, 5, 3, 3, 2]);
        assert_eq!(grid[3], vec![3, 3, 5, 4, 9]);
        assert_eq!(grid[4], vec![3, 5, 3, 9, 0]);
    }

    #[test]
    fn test_count_visible() {
        let grid = parse_grid(TEST_DATA);

        assert_eq!(21, count_visible(grid));
    }

    #[test]
    fn test_scenic_score() {
        let grid = parse_grid(TEST_DATA);

        assert_eq!(8, get_max_scenic_score(grid));
    }
}

fn heights_from_line(line: &str) -> Vec<u32> {
    line
        .chars()
        .map(|i| i.to_digit(10).unwrap())
        .collect::<Vec<u32>>()
}

pub fn parse_grid(data: &str) -> Vec<Vec<u32>> {
    data.lines()
        .map(|line| line.trim())
        .map(|line| heights_from_line(line))
        .collect::<Vec<Vec<u32>>>()
}

pub fn count_visible(grid: Vec<Vec<u32>>) -> usize {
    let mut total = 0;

    for (line_idx, row) in grid.iter().enumerate() {
        for (row_idx, height) in row.iter().enumerate() {
            // println!("Considering {},{} - {}", line_idx, row_idx, height);
            let mut is_blocked = false;
            
            // Check up
            for inner_idx in (0..line_idx).rev() {
                if grid[inner_idx][row_idx] >= *height { is_blocked = true; }
            }
            if ! is_blocked {
                // println!("Visible! (up)");
                total += 1;
                continue;
            }
            
            // Check left
            is_blocked = false;
            for inner_idx in (0..row_idx).rev() {
                if grid[line_idx][inner_idx] >= *height { is_blocked = true; }
            }
            if ! is_blocked {
                // println!("Visible! (left)");
                total += 1;
                continue;
            }

            // Check down
            is_blocked = false;
            for inner_idx in line_idx+1..grid.len() {
                if grid[inner_idx][row_idx] >= *height { is_blocked = true; }
            }
            if ! is_blocked {
                // println!("Visible! (down)");
                total += 1;
                continue;
            }

            // Check right
            is_blocked = false;
            for inner_idx in row_idx+1..row.len() {
                if grid[line_idx][inner_idx] >= *height { is_blocked = true; }
            }
            if ! is_blocked {
                // println!("Visible! (right)");
                total += 1;
                continue;
            }
        }
    }

    total
}

pub fn get_max_scenic_score(grid: Vec<Vec<u32>>) -> u32 {
    let mut scores: Vec<u32> = Vec::new();

    for (line_idx, row) in grid.iter().enumerate() {
        for (row_idx, height) in row.iter().enumerate() {
            // Check up
            let mut up: u32 = 0;
            for inner_idx in (0..line_idx).rev() {
                up += 1;
                if grid[inner_idx][row_idx] >= *height { break }
            }
            
            // Check left
            let mut left: u32 = 0;
            for inner_idx in (0..row_idx).rev() {
                left += 1;
                if grid[line_idx][inner_idx] >= *height { break }
            }

            // Check down
            let mut down: u32 = 0;
            for inner_idx in line_idx+1..grid.len() {
                down += 1;
                if grid[inner_idx][row_idx] >= *height { break }
            }

            // Check right
            let mut right = 0;
            for inner_idx in row_idx+1..row.len() {
                right += 1;
                if grid[line_idx][inner_idx] >= *height { break }
            }

            println!("Considering {},{} [{}] - {}, {}, {}, {}", line_idx, row_idx, height, up, left, down, right);
            scores.push(up * left * down * right);
        }
    }

    scores.iter()
        .max()
        .unwrap()
        .clone()
}

fn main() {
    let input = fs::read_to_string("data/day8.txt").unwrap();
    let grid = parse_grid(&input);

    println!("Part 1: {}", count_visible(grid.clone()));
    println!("Part 2: {}", get_max_scenic_score(grid.clone()));
}
