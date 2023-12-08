use std::fs;
use std::io::{prelude::*, BufReader};

#[derive(Debug)]
struct Card {
    win: Vec<usize>,
    num: Vec<usize>,
    copies: usize,
}

impl Card {
    fn score(&self) -> usize {
        self.num.iter().filter(|x| self.win.contains(x)).fold(
            0,
            |acc, _| {
                if acc == 0 {
                    1
                } else {
                    acc * 2
                }
            },
        )
    }

    fn wins(&self) -> usize {
        self.num
            .iter()
            .filter(|x| self.win.contains(x))
            .fold(0, |acc, _| acc + 1)
    }
}

pub fn day04() {
    let input_file = fs::File::open("../input/day04.txt").expect("input file");
    let input_buffer = BufReader::new(input_file);
    let mut cards = input_buffer
        .lines()
        .map(|x| {
            let text = x.unwrap();
            let mut split = text.split(" | ");
            let win = split
                .next()
                .unwrap()
                .split_whitespace()
                .filter_map(|x| x.parse::<usize>().ok())
                .collect::<Vec<_>>();
            let num = split
                .next()
                .unwrap()
                .split_whitespace()
                .filter_map(|x| match x.parse::<usize>() {
                    Ok(x) => Some(x),
                    Err(_) => None,
                })
                .collect::<Vec<_>>();
            Card {
                win,
                num,
                copies: 1,
            }
        })
        .collect::<Vec<_>>();

    let total_score = cards.iter().fold(0, |acc, x| acc + x.score());

    println!("\nDay 04\nPart 1 - Total score is: {}", total_score);

    for i in 0..cards.len() {
        let wins = cards[i].wins();
        for _ in 0..cards[i].copies {
            for j in 1..=wins {
                cards[i + j].copies += 1;
            }
        }
    }

    let count = cards.iter().fold(0, |acc, x| acc + x.copies);

    println!("Part 2 - Total number of cards is: {}", count);
}
