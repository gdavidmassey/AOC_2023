//AOC 2023 Day 05

use std::fs;
use std::time::Instant;

#[derive(Debug)]
struct Map {
    lo: isize,
    hi: isize,
    tx: isize,
}

impl Map {
    fn new(text: &str) -> Self {
        let mut vals = text
            .split_whitespace()
            .filter_map(|x| x.parse::<isize>().ok());
        let dst = vals.next().expect("valid input");
        let src = vals.next().expect("valid input");
        let rng = vals.next().expect("valid input");

        Self {
            lo: src,
            hi: src + rng,
            tx: dst - src,
        }
    }

    fn tx(&self, x: isize) -> Option<isize> {
        if x >= self.lo && x < self.hi {
            Some(x + self.tx)
        } else {
            None
        }
    }

    fn txs(&self, seed: Seed) -> SeedResult {
        let mut seed = seed;
        let mut out = SeedResult::new();
        if seed.hi <= self.lo {
            out.lo = Some(seed);
            return out;
        }
        if seed.lo >= self.hi {
            out.hi = Some(seed);
            return out;
        }
        if seed.lo < self.lo && self.lo < seed.hi {
            let split_seed = seed.split(self.lo);
            out.lo = Some(split_seed.0);
            seed = split_seed.1;
        }
        if seed.hi > self.hi && self.hi > seed.lo {
            let split_seed = seed.split(self.hi);
            out.hi = Some(split_seed.1);
            seed = split_seed.0;
        }
        seed.lo += self.tx;
        seed.hi += self.tx;
        out.tx = Some(seed);

        out
    }
}

struct Maps(Vec<Map>);

impl Maps {
    fn new() -> Self {
        Self(Vec::new())
    }

    fn push(&mut self, map: Map) {
        self.0.push(map);
    }

    fn tx(&self, x: isize) -> isize {
        self.0
            .iter()
            .map(|m| m.tx(x))
            .fold(Some(x), |acc, y| match y {
                Some(n) => Some(n),
                None => acc,
            })
            .expect("isize mapped to isize")
    }

    fn txs(&self, seeds: Vec<Seed>) -> Vec<Seed> {
        let mut seeds = seeds;
        let mut mapped = Vec::new();
        for map in self.0.iter() {
            let mut outside = Vec::new();
            for seed in seeds.into_iter() {
                let res = map.txs(seed);
                if let Some(s) = res.lo {
                    outside.push(s)
                };
                if let Some(s) = res.tx {
                    mapped.push(s)
                };
                if let Some(s) = res.hi {
                    outside.push(s)
                };
            }
            seeds = outside;
        }
        mapped.append(&mut seeds);
        mapped
    }

    fn import(&mut self, text: &str) {
        text.trim().lines().for_each(|x| self.push(Map::new(x)))
    }
}

struct SeedResult {
    tx: Option<Seed>,
    lo: Option<Seed>,
    hi: Option<Seed>,
}

impl SeedResult {
    fn new() -> Self {
        Self {
            tx: None,
            lo: None,
            hi: None,
        }
    }
}

#[derive(Debug)]
struct Seed {
    lo: isize,
    hi: isize,
}

impl Seed {
    fn split(&self, index: isize) -> (Seed, Seed) {
        if index <= self.lo || index >= self.hi {
            panic!("split out of range")
        }
        (
            Seed {
                lo: self.lo,
                hi: index,
            },
            Seed {
                lo: index,
                hi: self.hi,
            },
        )
    }
}

pub fn day05() {
    let parse_start = Instant::now();
    let input_raw = fs::read_to_string("./input/day05_input.txt").expect("input file");
    let mut input_split = input_raw.split("\n\n");

    let seeds = input_split.next().unwrap();

    let p1_seeds = seeds
        .split_whitespace()
        .filter_map(|x| x.parse::<isize>().ok())
        .collect::<Vec<_>>();

    let mut seed_iter = seeds
        .split_whitespace()
        .filter_map(|x| x.parse::<isize>().ok());

    let mut p2_seeds = Vec::new();

    while let Some(seed) = seed_iter.next() {
        p2_seeds.push(Seed {
            lo: seed,
            hi: seed + seed_iter.next().expect("seed range"),
        });
    }

    let maps: Vec<Maps> = input_split
        .map(|x| {
            let map_input = x.split(":").nth(1).expect("map input &str");

            let mut m = Maps::new();
            m.import(map_input);
            m
        })
        .collect();

    println!("Parsing input - {:?}", parse_start.elapsed());

    let p1_start = Instant::now();

    let min_loc = p1_seeds
        .iter()
        .map(|seed| maps.iter().fold(*seed, |acc, map| map.tx(acc)))
        .min()
        .expect("minimum value");

    println!(
        "Part 01 - {:?}\n  Lowest numbered location: {}",
        p1_start.elapsed(),
        min_loc
    );

    let p2_start = Instant::now();

    let p2_min_loc = maps
        .iter()
        .fold(p2_seeds, |acc, map| map.txs(acc))
        .iter()
        .map(|x| x.lo)
        .min()
        .expect("minimum value");

    println!(
        "Part 02 - {:?}\n  Lowest numbered location: {}",
        p2_start.elapsed(),
        p2_min_loc
    );
}
