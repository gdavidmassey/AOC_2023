use std::fs::File;
use std::io::{prelude::*, BufReader};

fn get_num(text: &str) -> u32 {
    let mut numeric = text
        .chars()
        .filter_map(|x| x.to_digit(10))
        .collect::<Vec<_>>();
    numeric[0] * 10 + numeric.pop().unwrap()
}

pub fn day01_p1() {
    let input_file = File::open("../input/day01.txt").expect("input file");
    let input_buffer = BufReader::new(input_file);
    let p2_sum = input_buffer
        .lines()
        .map(|x| x.unwrap())
        .fold(0, |acc, x| acc + get_num(&x));
    //.sum();
    println!("\nDay 01\nPart 1 total sum: {}", p2_sum);
}

pub fn day01_p2() {
    let sub = [
        ("one", "o1e"),
        ("two", "t2o"),
        ("three", "t3e"),
        ("four", "f4r"),
        ("five", "f5e"),
        ("six", "s6x"),
        ("seven", "s7n"),
        ("eight", "e8t"),
        ("nine", "n9e"),
    ];

    let input_file = File::open("../input/day01.txt").expect("input file");
    let input_buffer = BufReader::new(input_file);
    let p2_sum = input_buffer
        .lines()
        .map(|x| {
            sub.iter()
                .fold(x.unwrap(), |repl, s| repl.replace(s.0, s.1))
        })
        .fold(0, |acc, x| acc + get_num(&x));
    //.sum();
    println!("Part 2 total sum: {}", p2_sum);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(get_num("ds3dsg3s56dg"), 36);
    }
}
