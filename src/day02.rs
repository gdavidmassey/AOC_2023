use std::fs;
use std::io::{prelude::*, BufReader};

struct Handful {
    r: u32,
    g: u32,
    b: u32,
}

enum Colour {
    R(u32),
    G(u32),
    B(u32),
}

fn check_handful(text: &str) -> bool {
    let mut sample = Handful { r: 0, g: 0, b: 0 };
    text.split(", ").for_each(|x| {
        match count_colour(x) {
            Colour::R(n) => sample.r = n,
            Colour::G(n) => sample.g = n,
            Colour::B(n) => sample.b = n,
        };
    });

    is_valid(sample)
}

fn get_handful(text: &str) -> Handful {
    let mut sample = Handful { r: 0, g: 0, b: 0 };
    text.split(", ").for_each(|x| {
        match count_colour(x) {
            Colour::R(n) => sample.r = n,
            Colour::G(n) => sample.g = n,
            Colour::B(n) => sample.b = n,
        };
    });

    sample
}

fn is_valid(h: Handful) -> bool {
    h.r <= 12 && h.g <= 13 && h.b <= 14
}

fn all_valid(text: &str) -> u32 {
    let mut split = text.split(": ");
    let id = get_game_id(split.nth(0).expect("all_valid id"));
    if split
        .nth(0)
        .expect("all_valid games")
        .split("; ")
        .all(|x| check_handful(x))
    {
        id
    } else {
        0
    }
}

fn get_game_id(text: &str) -> u32 {
    text.split(" ").nth(1).unwrap().parse::<u32>().unwrap()
}

fn count_colour(text: &str) -> Colour {
    let mut split = text.split(" ");
    let n = split.next().unwrap().parse::<u32>().unwrap();

    let c = split.next().unwrap();
    let result = match c {
        "red" => Some(Colour::R(n)),
        "green" => Some(Colour::G(n)),
        "blue" => Some(Colour::B(n)),
        _ => None,
    };

    result.expect("count_colour")
}

fn min_power(text: &str) -> u32 {
    let mut min_set = Handful { r: 0, g: 0, b: 0 };
    let mut split = text.split(": ");
    split
        .nth(1)
        .expect("all_valid games")
        .split("; ")
        .for_each(|x| {
            let h = get_handful(x);
            if h.r > min_set.r {
                min_set.r = h.r
            }
            if h.g > min_set.g {
                min_set.g = h.g
            }
            if h.b > min_set.b {
                min_set.b = h.b
            }
        });
    power(min_set)
}

fn power(h: Handful) -> u32 {
    h.r * h.g * h.b
}

pub fn day02_p1() {
    let mut id_sum = 0;
    let input_file = fs::File::open("../input/day02.txt").expect("input file");
    let input_buffer = BufReader::new(input_file);

    input_buffer
        .lines()
        .for_each(|x| id_sum += all_valid(&x.expect("valid input")));

    println!("\nDay 02\nPart 1 - Sum of valid games: {}", id_sum);
}

pub fn day02_p2() {
    let mut power_sum = 0;
    let input_file = fs::File::open("../input/day02.txt").expect("input file");
    let input_buffer = BufReader::new(input_file);

    input_buffer
        .lines()
        .for_each(|x| power_sum += min_power(&x.expect("valid input")));

    println!("Part 2 - Sum of minimum power: {}", power_sum);
}
