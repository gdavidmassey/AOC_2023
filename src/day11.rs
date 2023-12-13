use std::cmp::Ordering;
use std::io::prelude::*;

struct XY {
    x: usize,
    y: usize,
}

fn between(x: usize, a: usize, b: usize) -> bool {
    match a.cmp(&b) {
        Ordering::Less => x > a && x < b,
        Ordering::Greater => x > b && x < a,
        Ordering::Equal => false,
    }
}

impl XY {
    fn distance(&self, xy: &XY) -> usize {
        self.x.abs_diff(xy.x) + self.y.abs_diff(xy.y)
    }

    fn x_dis(&self, xy: &XY) -> usize {
        self.x.abs_diff(xy.x)
    }

    fn y_dis(&self, xy: &XY) -> usize {
        self.y.abs_diff(xy.y)
    }

    fn a_milli(&self, xy: &XY, g_row: &Vec<usize>, g_col: &Vec<usize>, n: usize) -> usize {
        let mut e_row = 0;
        let mut e_col = 0;
        for i in g_row.iter() {
            if between(*i, self.y, xy.y) {
                e_row += 1;
            }
        }

        for i in g_col.iter() {
            if between(*i, self.x, xy.x) {
                e_col += 1;
            }
        }

        self.distance(xy) + (n - 1) * e_row + (n - 1) * e_col
    }
}

fn vec_ipairs(length: usize) -> Vec<(usize, usize)> {
    let mut ipairs = Vec::new();
    for x in 0..length {
        for y in (x + 1)..length {
            ipairs.push((x, y));
        }
    }
    ipairs
}

pub fn day11() {
    let test_str = "...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#.....";

    let test_str = test_str
        .lines()
        .map(|x| x.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();

    let input_file = std::fs::File::open("../input/day11.txt").unwrap();
    let input = std::io::BufReader::new(input_file)
        .lines()
        .map(|x| x.unwrap().chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();

    let empty_row = input
        .iter()
        .enumerate()
        .filter_map(|(x, row)| match row.iter().all(|c| *c == '.') {
            true => Some(x),
            false => None,
        })
        .collect::<Vec<usize>>();

    let empty_col = input[0]
        .iter()
        .enumerate()
        .filter_map(
            |(x, _)| match input.iter().map(|y| y[x]).all(|c| c == '.') {
                true => Some(x),
                false => None,
            },
        )
        .collect::<Vec<usize>>();
    /*
        empty_col.iter().for_each(|x| {
            input
                .iter()
                .enumerate()
                .for_each(|(y, _)| println!("{}", input[y][*x]))
        });
    */
    let tempty_row = test_str
        .iter()
        .enumerate()
        .filter_map(|(x, row)| match row.iter().all(|c| *c == '.') {
            true => Some(x),
            false => None,
        })
        .collect::<Vec<usize>>();

    let tempty_col = test_str[0]
        .iter()
        .enumerate()
        .filter_map(
            |(x, _)| match test_str.iter().map(|y| y[x]).all(|c| c == '.') {
                true => Some(x),
                false => None,
            },
        )
        .collect::<Vec<usize>>();

    let mut new_grid = Vec::new();

    for (y, row) in input.iter().enumerate() {
        let mut new_row = Vec::new();
        for (i, c) in row.iter().enumerate() {
            new_row.push(*c);
            if empty_col.contains(&i) {
                new_row.push('.');
            }
        }
        new_grid.push(new_row);
        if empty_row.contains(&y) {
            new_grid.push(vec!['.'; row.len() + empty_col.len()]);
        }
    }

    let mut test_galaxies = Vec::new();

    for (y, row) in test_str.iter().enumerate() {
        for (x, char) in row.iter().enumerate() {
            if *char == '#' {
                test_galaxies.push(XY { x, y })
            }
        }
    }

    println!("{}", test_galaxies.len());

    let mut og_galaxies = Vec::new();

    for (y, row) in input.iter().enumerate() {
        for (x, char) in row.iter().enumerate() {
            if *char == '#' {
                og_galaxies.push(XY { x, y })
            }
        }
    }

    let mut galaxies = Vec::new();

    for (y, row) in new_grid.iter().enumerate() {
        for (x, char) in row.iter().enumerate() {
            if *char == '#' {
                galaxies.push(XY { x, y })
            }
        }
    }

    println!("{:?}", galaxies.len());

    let pairs = vec_ipairs(galaxies.len());
    let ogpairs = vec_ipairs(og_galaxies.len());
    let tpairs = vec_ipairs(test_galaxies.len());

    let sum = pairs
        .iter()
        .fold(0, |acc, (a, b)| acc + galaxies[*a].distance(&galaxies[*b]));

    let mill_sum = ogpairs.iter().fold(0, |acc, (a, b)| {
        acc + og_galaxies[*a].a_milli(&og_galaxies[*b], &empty_row, &empty_col, 1000000)
    });

    let test_sum = tpairs.iter().fold(0, |acc, (a, b)| {
        acc + test_galaxies[*a].a_milli(&test_galaxies[*b], &tempty_row, &tempty_col, 100)
    });

    println!("{}", sum);
    println!("test {}", test_sum);

    println!("{}", mill_sum);
}
