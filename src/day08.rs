#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(dead_code)]

use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead, BufReader};

fn parse_line(text: &str) -> (&str, &str, &str) {
    let mut p = text
        .split(|x: char| [' ', ' ', ')', '(', '=', ','].contains(&x))
        .filter(|x| !x.is_empty());
    let key = p.next().unwrap();
    let l = p.next().unwrap();
    let r = p.next().unwrap();
    (key, l, r)
}

fn walk_zzz(path: &str, map: &HashMap<String, (String, String)>) -> usize {
    let mut steps = 0;
    let mut loc = "AAA";

    for c in path.chars().cycle() {
        let choice = map.get(loc).unwrap();

        loc = if c == 'L' {
            choice.0.as_str()
        } else {
            choice.1.as_str()
        };

        steps += 1;
        if loc == "ZZZ" {
            break;
        }
    }
    steps
}

fn walk_n<'a>(
    path: &'a str,
    map: &'a HashMap<String, (String, String)>,
    start: &'a str,
    n: usize,
) -> &'a str {
    let mut steps = 0;
    let mut loc = start;
    //let mut p = 0;

    for c in path.chars().cycle() {
        //p += 1;

        let choice = map.get(loc).unwrap();

        loc = if c == 'L' {
            choice.0.as_str()
        } else {
            choice.1.as_str()
        };
        /*
        if loc.ends_with('Z') {

            p = 0;
        }
        */
        steps += 1;
        if steps == n {
            break;
        }
    }
    loc
}

//This function works but is too slow on full input.
fn parallel_walk(path: &str, map: &HashMap<String, (String, String)>) -> usize {
    let mut steps = 0;
    let mut loc: Vec<&str> = map
        .keys()
        .filter(|x| x.ends_with('A'))
        .map(|x| x.as_str())
        .collect();

    for c in path.chars().cycle() {
        loc = loc
            .iter()
            .map(|x| {
                let choice = map.get(*x).unwrap();

                if c == 'L' {
                    &choice.0
                } else {
                    &choice.1
                }
            })
            .map(|x| x.as_str())
            .filter(|x| x != &"ZZZ")
            .collect();

        steps += 1;
        if loc.iter().all(|x| x.ends_with('Z')) || loc.len() == 0 {
            break;
        }
        if steps >= 1000000 {
            break;
        }
    }
    steps
}

fn ghost_map<'a>(
    path: &str,
    map: &'a HashMap<String, (String, String)>,
    n: usize,
) -> HashMap<String, String> {
    let mut g_map = HashMap::new();

    map.keys().for_each(|x| {
        g_map.insert(x.to_owned(), walk_n(path, map, x, n).to_owned());
    });
    g_map
}

fn walk_ghost(map: &HashMap<String, String>) -> usize {
    let mut steps = 0;

    let mut loc: Vec<&str> = map
        .keys()
        .filter(|x| x.ends_with('A'))
        .map(|x| x.as_str())
        .collect();
    //loc = vec![loc[0],loc[1],loc[2],loc[3]];
    while !loc.iter().all(|x| x.ends_with('Z')) {
        loc = loc
            .iter()
            .map(|y| map.get(*y).expect("mapped location").as_str())
            .collect();
        steps += 1;
        if steps >= 100000 {
            break;
        }
    }

    steps
}

fn walk_gmap<'a>(map: &'a HashMap<String, String>, start: &'a str, n: usize) -> &'a str {
    let mut loc = start;

    for _ in 0..n {
        loc = map.get(loc).unwrap().as_str();
    }

    loc
}

fn remap_gmap(map: &HashMap<String, String>, n: usize) -> HashMap<String, String> {
    let mut remap = HashMap::new();

    map.keys().for_each(|x| {
        remap.insert(x.to_owned(), walk_gmap(map, x, n).to_owned());
    });

    remap
}

pub fn day08() {
    let input_file = File::open("../input/day08.txt").expect("input file");
    let input_buffer = BufReader::new(input_file);
    let mut lines = input_buffer.lines();

    let path = lines.next().expect("valid_input").expect("valid_input");
    let lines = lines.skip(1);
    let mut map: HashMap<String, (String, String)> = HashMap::new();

    lines.for_each(|x| {
        let x = x.expect("valid input");

        let (key, left, right) = parse_line(&x);
        {
            map.insert(key.to_owned(), (left.to_owned(), right.to_owned()));
        }
    });

    let az_steps = walk_zzz(&path, &map);

    let g_map = ghost_map(&path, &map, az_steps);

    //Magic numbers from testing alignment of two Z locations

    let g_map = remap_gmap(&g_map, 2491);
    let g_map = remap_gmap(&g_map, 71);
    let g_map = remap_gmap(&g_map, 61);
    let g_map = remap_gmap(&g_map, 67);

    let ghost_steps = walk_ghost(&g_map);

    println!("\nDay 08\nPart 1 - Total steps: {}", az_steps);
    println!("Part 2 - Total steps: {}", az_steps * 2491 * 71 * 61 * 67);
}

