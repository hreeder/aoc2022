use std::{
    fs,
    str::FromStr
};

#[cfg(test)]
mod day2 {
    use super::*;

    const TEST_DATA: &str = "A Y
    B X
    C Z";

    #[test]
    fn test_load_choices() {
        let mut choices = load_choices(TEST_DATA);

        // first
        let (first_opponent, first) = choices.next().unwrap();
        println!("{:?} {:?}", first_opponent, first);
        assert!(matches!(first_opponent, Choice::Rock));
        assert!(matches!(first, Choice::Paper));
        assert_eq!(2, first.score());
        assert_eq!(8, first.against(first_opponent));

        let (second_opponent, second) = choices.next().unwrap();
        assert!(matches!(second_opponent, Choice::Paper));
        assert!(matches!(second, Choice::Rock));
        assert_eq!(1, second.score());
        assert_eq!(1, second.against(second_opponent));

        let (third_opponent, third) = choices.next().unwrap();
        assert!(matches!(third_opponent, Choice::Scissors));
        assert!(matches!(third, Choice::Scissors));
        assert_eq!(3, third.score());
        assert_eq!(6, third.against(third_opponent));
    }

    #[test]
    fn test_get_max_score() {
        let score: u16 = load_choices(TEST_DATA)
            .map(|(opponent, hand)| hand.against(opponent))
            .sum();

        assert_eq!(15, score);
    }

    #[test]
    fn test_load_strategy() {
        let mut choices = load_choices_with_strategy(TEST_DATA);

        let (_, first) = choices.next().unwrap();
        assert!(matches!(first, Choice::Rock));

        let (_, second) = choices.next().unwrap();
        assert!(matches!(second, Choice::Rock));

        let (_, third) = choices.next().unwrap();
        assert!(matches!(third, Choice::Rock));
    }

    #[test]
    fn test_score_with_strategy() {
        let score: u16 = load_choices_with_strategy(TEST_DATA)
            .map(|(opponent, hand)| hand.against(opponent))
            .sum();

        assert_eq!(12, score);
    }
}

#[derive(Debug)]
enum Choice { Rock, Paper, Scissors }

enum Decision { Win, Lose, Draw }

impl FromStr for Choice {
    type Err = ();

    fn from_str(s: &str) -> Result<Choice, ()> {
        match s {
            "A" | "X" => Ok(Choice::Rock),
            "B" | "Y" => Ok(Choice::Paper),
            "C" | "Z" => Ok(Choice::Scissors),
            _         => Err(()),
        }
    }
}

impl FromStr for Decision {
    type Err = ();

    fn from_str(s: &str) -> Result<Decision, ()> {
        match s {
            "X" => Ok(Decision::Lose),
            "Y" => Ok(Decision::Draw),
            "Z" => Ok(Decision::Win),
            _   => Err(()),
        }
    }
}

impl Choice {
    fn score(&self) -> u16 {
        match self {
            Choice::Rock     => 1,
            Choice::Paper    => 2,
            Choice::Scissors => 3,
        }
    }

    fn against(&self, opponent: Choice) -> u16 {
        let play_score = self.score();
        
        let result = match self {
            Choice::Rock     => {
                match opponent {
                    Choice::Rock     => 3,
                    Choice::Paper    => 0,
                    Choice::Scissors => 6,
                }
            },
            Choice::Paper    => {
                match opponent {
                    Choice::Rock     => 6,
                    Choice::Paper    => 3,
                    Choice::Scissors => 0,
                }
            },
            Choice::Scissors => {
                match opponent {
                    Choice::Rock     => 0,
                    Choice::Paper    => 6,
                    Choice::Scissors => 3,
                }
            },
        };

        play_score + result
    }
}

impl Decision {
    fn get_hand(&self, opponent: &Choice) -> Choice {
        match opponent {
            Choice::Rock => {
                match self {
                    Decision::Draw => Choice::Rock,
                    Decision::Lose => Choice::Scissors,
                    Decision::Win => Choice::Paper,
                }
            },
            Choice::Paper => {
                match self {
                    Decision::Draw => Choice::Paper,
                    Decision::Lose => Choice::Rock,
                    Decision::Win => Choice::Scissors,
                }
            },
            Choice::Scissors => {
                match self {
                    Decision::Draw => Choice::Scissors,
                    Decision::Lose => Choice::Paper,
                    Decision::Win => Choice::Rock,
                }
            },
        }
    }
}

fn load_choices<'a>(data: &'a str) -> Box<dyn Iterator<Item = (Choice, Choice)> + 'a> {
    Box::new(data
        .trim()
        .lines()
        .map(|line| {
            let mut parts = line.trim().splitn(2, " ");
            let first_str = parts.next().unwrap();
            let first = Choice::from_str(first_str).unwrap();

            let second_str = parts.next().unwrap();
            let second = Choice::from_str(second_str).unwrap();

            (first, second)
        })
    )
}

fn load_choices_with_strategy<'a>(data: &'a str) -> Box<dyn Iterator<Item = (Choice, Choice)> + 'a> {
    Box::new(data
        .trim()
        .lines()
        .map(|line| {
            let mut parts = line.trim().splitn(2, " ");
            let first_str = parts.next().unwrap();
            let first = Choice::from_str(first_str).unwrap();

            let second_str = parts.next().unwrap();
            let second = Decision::from_str(second_str).unwrap().get_hand(&first);

            (first, second)
        })
    )
}

fn main() {
    let input = fs::read_to_string("data/day2.txt").unwrap();

    let part_1: u16 = load_choices(&input)
        .map(|(opponent, hand)| hand.against(opponent))
        .sum();

    let part_2: u16 = load_choices_with_strategy(&input)
        .map(|(opponent, hand)| hand.against(opponent))
        .sum();

    println!("Part 1: {}", part_1);
    println!("Part 2: {}", part_2);
}