use std::collections::{HashMap, HashSet};
use std::fs;
use std::io::{prelude::*, BufReader};

struct Number {
    num: Vec<char>,
    is_part: bool,
    gears: HashSet<String>,
}

impl Number {
    fn new() -> Self {
        Self {
            num: Vec::new(),
            is_part: false,
            gears: HashSet::new(),
        }
    }

    fn clear(&mut self) {
        self.num = Vec::new();
        self.is_part = false;
        self.gears = HashSet::new()
    }

    fn get_part(&self) -> Option<usize> {
        match self.is_part {
            true => match self.num.iter().collect::<String>().parse::<usize>() {
                Ok(x) => Some(x),
                Err(_) => None,
            },
            false => None,
        }
    }
}

struct Map {
    map: Vec<Vec<char>>,
    w: usize,
    h: usize,
}

#[derive(Debug)]
struct Gears(HashMap<String, Vec<usize>>);

impl Gears {
    fn new() -> Self {
        Self(HashMap::new())
    }

    fn push(&mut self, k: &str, v: usize) {
        match self.0.get_mut(k) {
            Some(x) => (*x).push(v),
            None => {
                self.0.insert(k.to_owned(), vec![v]);
            }
        }
    }

    fn gear_ratio(&self) -> usize {
        self.0
            .values()
            .filter(|x| x.len() == 2)
            //.for_each(|x|x + 1)
            .fold(0, |acc, x| acc + x[0] * x[1])
    }
}

impl Map {
    fn new(map: Vec<Vec<char>>) -> Map {
        let w = map[0].len();
        let h = map.len();
        Map { map, w, h }
    }

    fn get(&self, x: usize, y: usize) -> Option<char> {
        if x >= self.w || y >= self.h {
            return None;
        }
        Some(self.map[y][x])
    }

    fn check_neighbour(&self, x: usize, y: usize) -> bool {
        let mut result = false;
        let x = x as isize;
        let y = y as isize;
        let x_offset = if x == 0 { vec![0, 1] } else { vec![-1, 0, 1] };
        let y_offset = if y == 0 { vec![0, 1] } else { vec![-1, 0, 1] };
        for xo in x_offset.iter() {
            for yo in y_offset.iter() {
                if *xo == 0 && *yo == 0 {
                    continue;
                }
                let nx = x + *xo;
                let ny = y + *yo;
                result = result || self.is_symbol(nx as usize, ny as usize);
            }
        }

        result
    }

    fn check_gears(&self, x: usize, y: usize) -> Vec<String> {
        let mut result = Vec::new();
        let x = x as isize;
        let y = y as isize;
        let x_offset = if x == 0 { vec![0, 1] } else { vec![-1, 0, 1] };
        let y_offset = if y == 0 { vec![0, 1] } else { vec![-1, 0, 1] };
        for xo in x_offset.iter() {
            for yo in y_offset.iter() {
                if *xo == 0 && *yo == 0 {
                    continue;
                }
                let nx = x + *xo;
                let ny = y + *yo;
                match self.gear_index(nx as usize, ny as usize) {
                    Some(x) => result.push(x),
                    None => (),
                }
            }
        }

        result
    }

    fn is_symbol(&self, x: usize, y: usize) -> bool {
        let c = self.get(x, y);
        match c {
            None => false,
            Some('.') => false,
            Some(c) => match c.to_digit(10) {
                Some(_) => false,
                None => true,
            },
        }
    }

    fn gear_index(&self, x: usize, y: usize) -> Option<String> {
        let c = self.get(x, y);
        match c {
            None => None,
            Some('*') => Some((self.w * y + x).to_string()),
            Some(_) => None,
        }
    }
}

pub fn day03() {
    let input_file = fs::File::open("../input/day03.txt").expect("input file");
    let input_buffer = BufReader::new(input_file);
    let lines = Map::new(
        input_buffer
            .lines()
            .map(|x| x.unwrap().chars().collect::<Vec<_>>())
            .collect::<Vec<_>>(),
    );

    let mut parts: Vec<usize> = Vec::new();
    let mut gears = Gears::new();
    let mut number = Number::new();
    let mut last_numeric = false;
    for y in 0..lines.h {
        for x in 0..lines.w {
            let c = lines.get(x, y).unwrap();
            //println!("{}",c);
            let is_numeric = match c.to_digit(10) {
                Some(_) => true,
                None => false,
            };

            match is_numeric {
                true => {
                    number.num.push(c);
                    number.is_part |= lines.check_neighbour(x, y);
                    for g in lines.check_gears(x, y).iter() {
                        number.gears.insert(g.to_owned());
                    }
                }
                false => match last_numeric {
                    true => match number.get_part() {
                        Some(x) => {
                            parts.push(x);
                            for g in number.gears.iter() {
                                gears.push(g, x);
                            }
                            number.clear();
                        }
                        None => number.clear(),
                    },
                    false => continue,
                },
            }
            last_numeric = is_numeric;
        }
        match number.get_part() {
            Some(x) => {
                parts.push(x);
                for g in number.gears.iter() {
                    gears.push(g, x);
                }
                number.clear();
            }
            None => number.clear(),
        }
    }
    let total = parts.iter().fold(0, |x, acc| acc + x);
    println!("\nDay 03\nPart 1 - Sum of parts: {}", total);
    println!("Part 2 - Sum of gear ratios: {}", gears.gear_ratio());
}
