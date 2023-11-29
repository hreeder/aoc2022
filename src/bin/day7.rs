#![allow(dead_code)]
#![allow(unused_variables)]

#[cfg(test)]
mod day7 {    
    const TEST_DATA: &str = "$ cd /
    $ ls
    dir a
    14848514 b.txt
    8504156 c.dat
    dir d
    $ cd a
    $ ls
    dir e
    29116 f
    2557 g
    62596 h.lst
    $ cd e
    $ ls
    584 i
    $ cd ..
    $ cd ..
    $ cd d
    $ ls
    4060174 j
    8033020 d.log
    5626152 d.ext
    7214296 k";
}

struct FSEntry {
    name: String,
    is_directory: bool,
    size: usize,
    children: Vec<FSEntry>,
}

impl FSEntry {
    fn new(name: String) -> FSEntry {
        FSEntry { name, is_directory: true, size: 0, children: vec![] }
    }
}

pub fn parse_fs(history: &str) {
    let root = FSEntry::new(String::from("/"));
    let current_path = "/";

    for line in history.trim().lines() {
        if line.starts_with("$ ") {
            // Line is a command
            let mut args = line.split(" ");
            args.next(); // Skip the $
            let command = args.next().unwrap();
            let arg = args.next().unwrap();

            if command == "cd" && arg != ".." {

            }
        }
    }
}

fn main() {
    // let input = fs::read_to_string("data/day7.txt").unwrap();

    println!("Part 1: {}", "aa");
    println!("Part 2: {}", "aa");
}
