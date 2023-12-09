use std::cmp::Ordering;
use std::collections::HashMap;
use std::fs;
use std::io::{prelude::*, BufReader};

#[derive(Debug)]
struct Hand {
    cards: String,
    bet: usize,
}

#[derive(Debug, PartialEq, PartialOrd)]
enum Score {
    High,
    Pair,
    TwoPair,
    Triple,
    FullHouse,
    Four,
    Five,
    None,
}

impl Score {
    fn cmp(&self, score: &Score) -> Ordering {
        if self > score {
            Ordering::Greater
        } else if self == score {
            Ordering::Equal
        } else {
            Ordering::Less
        }
    }
}

fn card(c: char) -> Option<usize> {
    match c {
        '2' => Some(1),
        '3' => Some(2),
        '4' => Some(3),
        '5' => Some(4),
        '6' => Some(5),
        '7' => Some(6),
        '8' => Some(7),
        '9' => Some(8),
        'T' => Some(9),
        'J' => Some(10),
        'Q' => Some(11),
        'K' => Some(12),
        'A' => Some(13),
        _ => None,
    }
}

fn score_hand(hand: &str) -> Score {
    let mut tally = HashMap::new();

    hand.chars().for_each(|x| match tally.get_mut(&x) {
        Some(v) => {
            *v += 1;
        }
        None => {
            tally.insert(x, 1);
        }
    });

    let mut tally: Vec<usize> = tally.values().copied().collect();

    tally.sort_by(|x, y| y.cmp(x));

    match tally[..] {
        [1, 1, 1, 1, 1] => Score::High,
        [2, 1, 1, 1] => Score::Pair,
        [3, 1, 1] => Score::Triple,
        [2, 2, 1] => Score::TwoPair,
        [3, 2] => Score::FullHouse,
        [4, 1] => Score::Four,
        [5] => Score::Five,
        _ => Score::None,
    }
}

fn cmp_high(h1: &str, h2: &str) -> Ordering {
    for (h1c, h2c) in h1.chars().zip(h2.chars()) {
        match card(h1c).cmp(&card(h2c)) {
            Ordering::Less => return Ordering::Less,
            Ordering::Equal => (),
            Ordering::Greater => return Ordering::Greater,
        }
    }
    Ordering::Equal
}

pub fn day07_p1() {
    let input_file = fs::File::open("../input/day07.txt").expect("input file");
    let input_buffer = BufReader::new(input_file);
    let mut games: Vec<Hand> = input_buffer
        .lines()
        .map(|x| x.unwrap())
        .map(|x| {
            let mut line = x.split_whitespace();
            Hand {
                cards: line.next().unwrap().to_owned(),
                bet: line.next().unwrap().parse().unwrap(),
            }
        })
        .collect();

    games.sort_by(
        |x, y| match score_hand(&x.cards).cmp(&score_hand(&y.cards)) {
            Ordering::Less => Ordering::Less,
            Ordering::Greater => Ordering::Greater,
            Ordering::Equal => cmp_high(&x.cards, &y.cards),
        },
    );

    let result = games
        .iter()
        .enumerate()
        .fold(0, |acc, (i, v)| acc + (i + 1) * v.bet);

    println!("\nDay 07\nPart 1 - total: {}", result);
}

#[cfg(Test)]

mod tests {
    use super::*;

    #[Test]
    fn test() {
        assert_eq!(score_hand("32TJK"), Score::High);
        assert_eq!(score_hand("32T3K"), Score::Pair);
        assert_eq!(score_hand("KK677"), Score::TwoPair);
        assert_eq!(score_hand("T55J5"), Score::Triple);
        assert_eq!(score_hand("QQJJQ"), Score::FullHouse);
        assert_eq!(score_hand("3333K"), Score::Four);
        assert_eq!(score_hand("QQQQQ"), Score::Five);
        assert_eq!(cmp_high("39JQK", "39J2A"), Ordering::Greater);
    }
}
