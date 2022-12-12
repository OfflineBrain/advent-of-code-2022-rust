use std::cmp::Ordering;
use std::str::FromStr;

use crate::Move::{Paper, Rock, Scissors};

#[derive(PartialEq, Copy, Clone)]
enum Move {
    Rock = 1,
    Paper = 2,
    Scissors = 3,
}

impl PartialOrd for Move {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        if self == &Scissors && other == &Rock {
            Some(Ordering::Less)
        } else if self == &Rock && other == &Scissors {
            Some(Ordering::Greater)
        } else {
            Some((*self as u8).cmp(&(*other as u8)))
        }
    }
}

impl FromStr for Move {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "A" | "X" => Ok(Rock),
            "B" | "Y" => Ok(Paper),
            "C" | "Z" => Ok(Scissors),
            _ => Err("Not a move".to_string()),
        }
    }
}

pub fn part1(input: &str) -> String {
    let result = input
        .lines()
        .map(|line| {
            let moves = line
                .split(" ")
                .map(|mov| mov.parse::<Move>().unwrap())
                .collect::<Vec<_>>();

            match moves[0].partial_cmp(&moves[1]) {
                None => panic!("moves should be comparable"),
                Some(Ordering::Equal) => 3 + moves[1] as u32,
                Some(Ordering::Greater) => moves[1] as u32,
                Some(Ordering::Less) => 6 + moves[1] as u32,
            }
        })
        .sum::<u32>();

    result.to_string()
}

pub fn part2(input: &str) -> String {
    let result = input
        .lines()
        .map(|line| {
            let moves = line.split(" ").collect::<Vec<_>>();

            let opponent = moves[0].parse::<Move>().unwrap();

            let player = match moves[1] {
                "X" => match opponent {
                    Rock => Scissors,
                    Paper => Rock,
                    Scissors => Paper,
                },
                "Y" => opponent,
                "Z" => match opponent {
                    Rock => Paper,
                    Paper => Scissors,
                    Scissors => Rock,
                },
                _ => !panic!(),
            };

            match opponent.partial_cmp(&player) {
                None => panic!("moves should be comparable"),
                Some(Ordering::Equal) => 3 + player as u32,
                Some(Ordering::Greater) => player as u32,
                Some(Ordering::Less) => 6 + player as u32,
            }
        })
        .sum::<u32>();

    result.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "A Y
B X
C Z";

    #[test]
    fn part1_works() {
        let result = part1(INPUT);
        assert_eq!(result, "15");
    }

    #[test]
    fn part2_works() {
        let result = part2(INPUT);
        assert_eq!(result, "12");
    }
}
