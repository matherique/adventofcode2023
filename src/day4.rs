use std::env;

use std::{collections::HashSet, str::FromStr, string::ParseError};

mod utils;

fn main() {
    println!("Day 4 Part 1")
}

#[derive(Debug, Default)]
struct Card {
    id: usize,
    winners: HashSet<i32>,
    play: HashSet<i32>,
}

impl Card {
    fn count(&self) -> usize {
        self.winners.intersection(&self.play).count()
    }

    fn points(&self) -> i32 {
        let total = self.count();

        if total == 0 {
            return 0;
        }

        return 1 << total - 1;
    }
}

impl FromStr for Card {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
        let (card_id, rest) = s.split_once(":").unwrap();
        let id = card_id.replace("Card ", "");

        let (winner, play) = rest.split_once("|").unwrap();

        let winner_numbers: HashSet<i32> = winner
            .trim_end()
            .split(" ")
            .filter(|n| !n.is_empty())
            .map(|num| num.trim_end().parse::<i32>().unwrap_or(0))
            .collect();

        let play_numbers: HashSet<i32> = play
            .trim_end()
            .split(" ")
            .filter(|n| !n.is_empty())
            .map(|num| num.trim_end().parse::<i32>().unwrap_or(0))
            .collect();

        Ok(Card {
            id: id.parse::<usize>().unwrap_or(0),
            play: play_numbers,
            winners: winner_numbers,
        })
    }
}

fn part1(input: String) -> i32 {
    let mut total = 0;
    input.lines().for_each(|line| {
        if let Ok(card) = line.parse::<Card>() {
            let points = card.points();
            total += points;
        }
    });

    return total;
}

fn part2(input: String) -> usize {
    let cards: Vec<Card> = input
        .trim_end()
        .lines()
        .map(|line| line.parse::<Card>().unwrap())
        .collect();

    let mut r = vec![1; cards.len()];

    for (index, card) in cards.iter().enumerate() {
        let c = card.count();

        for i in index + 1..(index + 1 + c) {
            r[i] += r[index]
        }
    }

    return r.iter().sum::<usize>();
}

#[test]
fn test_part1() {
    let content = utils::read_file("4").unwrap();
    let response = part1(content);

    if let Ok(_) = env::var("EXAMPLE") {
        assert_eq!(response, 14);
    } else {
        assert_eq!(response, 20407);
    }
}

#[test]
fn test_part2_example() {
    let content = utils::read_file("4").unwrap();
    let response = part2(content);

    if let Ok(_) = std::env::var("EXAMPLE") {
        assert_eq!(response, 30);
    } else {
        assert_eq!(response, 23806951);
    }
}
