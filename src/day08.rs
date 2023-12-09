use std::cmp::Ordering;
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn parse_line(text: &str) -> (&str, &str, &str) {
    let mut p = text
        .split(|x: char| [' ', ' ', ')', '(', '=', ','].contains(&x))
        .filter(|x| !x.is_empty());
    let key = p.next().unwrap();
    let l = p.next().unwrap();
    let r = p.next().unwrap();
    (key, l, r)
}

fn walk_z(path: &str, map: &HashMap<String, (String, String)>, start: &str) -> usize {
    let mut steps = 0;
    let mut loc = start;

    for c in path.chars().cycle() {
        let choice = map.get(loc).unwrap();

        loc = if c == 'L' {
            choice.0.as_str()
        } else {
            choice.1.as_str()
        };

        steps += 1;
        if loc.ends_with('Z') {
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

fn ghost_map(
    path: &str,
    map: &HashMap<String, (String, String)>,
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

fn gcd(a: usize, b: usize) -> usize {
    match a.cmp(&b) {
        Ordering::Equal => a,
        Ordering::Less => {
            if a == 0 {
                b
            } else {
                gcd(a, b % a)
            }
        }
        Ordering::Greater => {
            if b == 0 {
                a
            } else {
                gcd(a % b, b)
            }
        }
    }
}

fn lcm(a: usize, b: usize) -> usize {
    a * b / gcd(a, b)
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

    let az_steps = walk_z(&path, &map, "AAA");

    let g_map = ghost_map(&path, &map, az_steps);

    //Magic numbers from testing alignment of two Z locations

    let g_map = remap_gmap(&g_map, 2491);
    let g_map = remap_gmap(&g_map, 71);
    let g_map = remap_gmap(&g_map, 61);
    let g_map = remap_gmap(&g_map, 67);

    let ghost_steps = walk_ghost(&g_map);

    println!("\nDay 08\nPart 1 - Total steps: {}", az_steps);
    println!(
        "Part 2 - Total steps: {}",
        az_steps * 2491 * 71 * 61 * 67 * ghost_steps
    );

    //after looking at reddit
    let z_period: Vec<usize> = map
        .keys()
        .filter(|x| x.ends_with('A'))
        .map(|x| walk_z(&path, &map, x.as_str()))
        .collect();

    let better_part2 = z_period.iter().fold(z_period[0], |acc, x| lcm(acc, *x));
    println!("Better Part 2: {}", better_part2);
}

