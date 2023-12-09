use std::fs::File;
use std::io::{prelude::*, BufReader};

fn parse_input() -> Vec<Vec<isize>> {
    let input_file = File::open("../input/day09.txt").expect("input file");
    let input_buffer = BufReader::new(input_file);

    input_buffer
        .lines()
        .map(|x| {
            let line = x.expect("valid input");
            line.split_whitespace()
                .filter_map(|x| x.parse().ok())
                .collect()
        })
        .collect()
}

fn extend_pattern(v: &[isize]) -> Vec<isize> {
    let mut v_extend: Vec<isize> = v.to_vec();

    let v_reduced: Vec<isize> = v.windows(2).map(|x| x[1] - x[0]).collect();

    if v_reduced.iter().all(|x| *x == 0) {
        v_extend.push(*v_extend.last().unwrap());
        return v_extend;
    }

    v_extend.push(*v_extend.last().unwrap() + *extend_pattern(&v_reduced).last().unwrap());

    v_extend
}

fn extend_pattern_left(v: &[isize]) -> Vec<isize> {
    let mut v_extend: Vec<isize> = v.to_vec();

    let v_reduced: Vec<isize> = v.windows(2).map(|x| x[1] - x[0]).collect();

    if v_reduced.iter().all(|x| *x == 0) {
        v_extend.insert(0, *v_extend.last().unwrap());
        return v_extend;
    }

    v_extend.insert(0, v_extend[0] - extend_pattern_left(&v_reduced)[0]);

    v_extend
}

pub fn day09() {
    let input = parse_input();
    let part1_extended: Vec<Vec<isize>> = input.iter().map(|x| extend_pattern(x)).collect();
    let part2_extended: Vec<Vec<isize>> = input.iter().map(|x| extend_pattern_left(x)).collect();

    let part1_sum = part1_extended
        .iter()
        .fold(0, |acc, x| acc + x.last().unwrap());
    let part2_sum = part2_extended.iter().fold(0, |acc, x| acc + x[0]);
    println!("\nDay 09\nPart 1 - Sum right value: {}", part1_sum);
    println!("Part 2 - Sum left value: {}", part2_sum);
}
