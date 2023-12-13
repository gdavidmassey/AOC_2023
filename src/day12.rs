//.??..??...?##. 1,1,3

use std::io::prelude::*;

fn check(s: &str, n: usize) -> bool {
    if s.len() < n {
        return false;
    };

    s[..n].chars().all(|c| c == '#' || c == '?')
}

fn valid(s: &str, pat: &[usize]) -> usize {
    //println!("{}",s);
    //println!("{:?}",pat);
    if s.len() == 0 {
        match pat.len() >= 1 {
            true => return 0,
            false => return 1,
        }
    };

    if pat.len() == 0 {
        match s.contains('#') {
            true => return 0,
            false => return 1,
        }
    };

    if s.len() < pat[0] {
        return 0;
    };

    if s.len() == pat[0] && pat.len() > 1 {
        return 0;
    };

    if s.len() == pat[0] && pat.len() == 1 {
        if check(s, pat[0]) {
            return 1;
        } else {
            return 0;
        }
    };

    let cur_char = if s.len() > pat[0] && s.chars().nth(pat[0]).unwrap() == '#' {
        0
    } else if check(s, pat[0]) {
        valid(&s[pat[0] + 1..], &pat[1..])
    } else {
        0
    };

    let next_char = if s.starts_with('#') {
        0
    } else {
        valid(&s[1..], &pat)
    };

    cur_char + next_char
}

pub fn day12() {
    //let tsum = vec![".??..??...?##. 1,1,3"];
    let input_file = std::fs::File::open("../input/day12.txt").unwrap();
    let tot = std::io::BufReader::new(input_file)
        .lines()
        .fold(0, |acc, x| {
            let line = x.unwrap();
            let mut spl = line.split(' ');
            let s = spl.next().unwrap();
            let s = vec![s, s, s, s, s].join("?");
            let pat = spl.next().unwrap();
            let pat = vec![pat, pat, pat, pat, pat].join(",");
            let pat: Vec<usize> = pat.split(',').map(|x| x.parse().unwrap()).collect();
            let val = valid(&s, &pat[..]);
            println!("{} {:?} - {}", s, pat, val);
            acc + val
        });

    println!("{}", tot);
}
