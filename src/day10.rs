use std::io::prelude::*;

#[derive(Debug, Clone, Copy, PartialEq)]
struct XY {
    x: usize,
    y: usize,
}

struct PipeMap {
    part_map: Vec<Vec<Part>>,
}

impl PipeMap {
    fn new(part_map: Vec<Vec<Part>>) -> Self {
        Self { part_map }
    }

    fn get_s(&self) -> Option<XY> {
        self.part_map.iter().enumerate().find_map(|(y, line)| {
            match line.iter().enumerate().find_map(|(x, part)| match part {
                Part::PS => Some(x),
                _ => None,
            }) {
                Some(x) => Some(XY { x, y }),
                None => None,
            }
        })
    }

    fn walk_pipe(&self) -> Vec<XY> {
        let mut pos = self.get_s().unwrap();
        let mut visited = Vec::new();
        let mut i = 0;

        while i < 100000 {
            let look = [
                self.look_u(&pos),
                self.look_r(&pos),
                self.look_d(&pos),
                self.look_l(&pos),
            ];

            let next_pos = look.iter().find_map(|x| match x {
                Some(x) => match Some(x) == visited.last() {
                    true => None,
                    false => Some(x),
                },
                None => None,
            });

            i += 1;

            visited.push(pos);

            if *self.check_part(next_pos.unwrap()) == Part::PS {
                break;
            }

            pos = *next_pos.unwrap();
        }

        visited
    }

    fn check_part(&self, xy: &XY) -> &Part {
        &self.part_map[xy.y][xy.x]
    }

    fn look_u(&self, xy: &XY) -> Option<XY> {
        let can_connect = [Part::PS, Part::PJ, Part::PL, Part::Pv];
        if !can_connect.contains(self.check_part(xy)) || xy.y == 0 {
            return None;
        }
        let xy = XY {
            x: xy.x,
            y: xy.y - 1,
        };

        match self.check_part(&xy) {
            Part::PF => Some(xy),
            Part::P7 => Some(xy),
            Part::Pv => Some(xy),
            Part::PS => Some(xy),
            _ => None,
        }
    }

    fn look_d(&self, xy: &XY) -> Option<XY> {
        let can_connect = [Part::PS, Part::PF, Part::P7, Part::Pv];
        if !can_connect.contains(self.check_part(xy)) {
            return None;
        }
        let xy = XY {
            x: xy.x,
            y: xy.y + 1,
        };

        match self.check_part(&xy) {
            Part::PJ => Some(xy),
            Part::PL => Some(xy),
            Part::Pv => Some(xy),
            Part::PS => Some(xy),
            _ => None,
        }
    }

    fn look_l(&self, xy: &XY) -> Option<XY> {
        let can_connect = [Part::PS, Part::PJ, Part::P7, Part::Ph];
        if !can_connect.contains(self.check_part(xy)) || xy.x == 0 {
            return None;
        }
        let xy = XY {
            x: xy.x - 1,
            y: xy.y,
        };

        match self.check_part(&xy) {
            Part::PF => Some(xy),
            Part::PL => Some(xy),
            Part::Ph => Some(xy),
            Part::PS => Some(xy),
            _ => None,
        }
    }

    fn look_r(&self, xy: &XY) -> Option<XY> {
        let can_connect = [Part::PS, Part::PF, Part::PL, Part::Ph];
        if !can_connect.contains(self.check_part(xy)) {
            return None;
        }
        let xy = XY {
            x: xy.x + 1,
            y: xy.y,
        };

        match self.check_part(&xy) {
            Part::PJ => Some(xy),
            Part::P7 => Some(xy),
            Part::Ph => Some(xy),
            Part::PS => Some(xy),
            _ => None,
        }
    }

    fn only_pipe(&self) -> Vec<Vec<&Part>> {
        let pipe = self.walk_pipe();
        self.part_map
            .iter()
            .enumerate()
            .map(|(y, line)| {
                line.iter()
                    .enumerate()
                    .map(|(x, part)| match pipe.contains(&XY { x, y }) {
                        true => part,
                        false => &Part::P_,
                    })
                    .collect()
            })
            .collect()
    }
}

#[derive(Debug, PartialEq, Clone, Copy)]
enum Part {
    PF,
    PJ,
    PL,
    P7,
    Ph,
    Pv,
    P_,
    PS,
}

fn parse_input() -> Vec<Vec<Part>> {
    let input_file = std::fs::File::open("../input/day10.txt").expect("input file");
    let input_buffer = std::io::BufReader::new(input_file);
    input_buffer
        .lines()
        .map(|x| {
            x.expect("valid input")
                .chars()
                .filter_map(|x| match x {
                    'L' => Some(Part::PL),
                    'F' => Some(Part::PF),
                    '7' => Some(Part::P7),
                    'J' => Some(Part::PJ),
                    '|' => Some(Part::Pv),
                    '-' => Some(Part::Ph),
                    '.' => Some(Part::P_),
                    'S' => Some(Part::PS),
                    'I' => Some(Part::P_),
                    _ => None,
                })
                .collect::<Vec<Part>>()
        })
        .collect()
}

fn check_in_x(map_row: &[&Part], xy: &XY) -> bool {
    if map_row[xy.x] != &Part::P_ {
        return false;
    }

    let p = map_row[0..xy.x].iter().fold(0, |acc, part| match part {
        Part::Pv | Part::PF | Part::P7 => acc + 1,
        _ => acc,
    });

    p % 2 == 1
}

fn check_in_y(map: &[Vec<&Part>], xy: &XY) -> bool {
    if map[xy.y][xy.x] != &Part::P_ {
        return false;
    }
    let col = map.iter().map(|row| row[xy.x]).collect::<Vec<&Part>>();
    let p = col[0..xy.y].iter().fold(0, |acc, part| match part {
        Part::Ph | Part::PF | Part::PL => acc + 1,
        _ => acc,
    });

    p % 2 == 1
}

pub fn day10() {
    let p = PipeMap::new(parse_input());
    let pipe_coords = p.walk_pipe();
    let po = p.only_pipe();

    println!(
        "\nDay 10\nPart 1 - Furthest point from S: {}",
        pipe_coords.len() / 2
    );

    let p2 = po.iter().enumerate().fold(0, |acc, (y, line)| {
        acc + line.iter().enumerate().fold(0, |acc, (x, _)| {
            match check_in_x(&line[..], &XY { x, y }) && check_in_y(&po, &XY { x, y }) {
                true => acc + 1,
                false => acc,
            }
        })
    });

    println!("Part 2 - Tiles inside pipe: {}", p2);
}
