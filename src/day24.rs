use crate::Timing;
use std::{
    collections::{HashMap, HashSet, VecDeque},
    time::{Duration, Instant},
};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Blizzard {
    dir: Point,
    pos: Point,
}

impl Blizzard {
    fn mve(&self, max_x: i64, max_y: i64) -> Self {
        let mut new_x = self.pos.0 + self.dir.0;
        let mut new_y = self.pos.1 + self.dir.1;

        if new_x == 0 {
            new_x = max_x - 1;
        } else if new_x == max_x {
            new_x = 1;
        } else if new_y == 0 {
            new_y = max_y - 1;
        } else if new_y == max_y {
            new_y = 1;
        }
        Blizzard {
            dir: self.dir,
            pos: (new_x, new_y),
        }
    }
}

type Point = (i64, i64);

fn parse(input: &str) -> (HashSet<Blizzard>, Point, Point, Point) {
    let mut blizzards = HashSet::<Blizzard>::new();

    let mut start = (0, 0);
    let mut exit = (0, 0);
    let mut max_x = 0;
    let mut max_y = 0;
    for (idy, line) in input.lines().enumerate() {
        for (idx, c) in line.chars().enumerate() {
            let x = idx as i64;
            let y = idy as i64;
            match c {
                '>' => {
                    blizzards.insert(Blizzard {
                        dir: (1, 0),
                        pos: (x, y),
                    });
                }
                '<' => {
                    blizzards.insert(Blizzard {
                        dir: (-1, 0),
                        pos: (x, y),
                    });
                }
                'v' => {
                    blizzards.insert(Blizzard {
                        dir: (0, 1),
                        pos: (x, y),
                    });
                }
                '^' => {
                    blizzards.insert(Blizzard {
                        dir: (0, -1),
                        pos: (x, y),
                    });
                }
                '#' => {
                    max_x = max_x.max(x);
                    max_y = max_y.max(y);
                }
                '.' => {
                    if idy == 0 {
                        start = (x, y);
                    } else {
                        exit = (x, y);
                    }
                }
                _ => unreachable!(),
            }
        }
    }
    (blizzards, (max_x, max_y), start, exit)
}

#[allow(dead_code)]
fn get_lcm(a: i64, b: i64) -> i64 {
    let mut min = a.min(b);
    let mut max = a.max(b);
    let mut rem = min % max;

    while rem != 0 {
        max = min;
        min = rem;
        rem = max % min;
    }
    a * b / min
}

fn simulate(
    movements: &HashSet<Blizzard>,
    max_x: i64,
    max_y: i64,
    entry: &Point,
    exit: &Point,
) -> (i64, i64) {
    let mut blizzard_positions = HashMap::<i64, HashSet<Blizzard>>::new();
    let t1 = Instant::now();
    blizzard_positions.insert(0, movements.clone());

    for t in 1..750 {
        let mut new_blizzards = HashSet::<Blizzard>::new();
        blizzard_positions
            .get(&(t - 1))
            .unwrap()
            .iter()
            .for_each(|b| {
                new_blizzards.insert(b.mve(max_x, max_y));
            });
        blizzard_positions.insert(t, new_blizzards);
    }

    println!("Pre comp done took {:?}", t1.elapsed());

    let mut trip_times = [i64::MAX, i64::MAX, i64::MAX];

    let mut start_time = 0;
    for trip_num in 0..3 {
        let (mut start, mut target) = (entry, exit);
        if trip_num == 1 {
            start = exit;
            target = entry;
        }

        let mut seen = HashSet::<(Point, i64)>::new();
        let mut q = VecDeque::new();
        q.push_back((*start, start_time));

        while let Some(((next_x, next_y), time)) = q.pop_front() {
            if seen.contains(&((next_x, next_y), time)) {
                continue;
            }
            seen.insert(((next_x, next_y), time));

            let child_blizzard = blizzard_positions.get(&(time + 1)).unwrap();

            for (dx, dy) in &[(0, -1), (0, 1), (1, 0), (-1, 0), (0, 0)] {
                let (child_x, child_y) = (next_x + dx, next_y + dy);
                if (child_x, child_y) == *target {
                    trip_times[trip_num] = time + 1;
                    q.clear();
                    break;
                }
                let check_blizz = no_blizzard_target((child_x, child_y), child_blizzard);
                if (*dx == 0 && *dy == 0 && check_blizz)
                    || ((1..max_x).contains(&child_x)
                        && (1..max_y).contains(&child_y)
                        && check_blizz)
                {
                    // println!("  Can move to {child_x}/{child_y}");
                    q.push_back(((child_x, child_y), time + 1));
                }
            }
        }
        if trip_num == 2 {
            return (trip_times[0], trip_times[2]);
        }
        start_time = trip_times[trip_num];
    }
    unreachable!()
}

#[allow(dead_code)]
fn dump_blizzard(blizzard: &HashSet<Blizzard>, max_x: i64, max_y: i64) {
    for y in 0..=max_y {
        for x in 0..=max_x {
            if x == 0 || x == max_x || y == 0 || y == max_y {
                print!("#");
            } else {
                let mut it = blizzard.iter().filter(|b| b.pos == (x, y));

                let cnt = it.clone().count();

                if cnt == 1 {
                    match it.next().unwrap().dir {
                        (1, 0) => print!(">"),
                        (-1, 0) => print!("<"),
                        (0, 1) => print!("v"),
                        (0, -1) => print!("^"),
                        _ => unreachable!(),
                    }
                } else if cnt == 0 {
                    print!(".");
                } else {
                    print!("{cnt}");
                }
            }
        }
        println!();
    }
}

fn no_blizzard_target(child_pos: Point, child_blizzard: &HashSet<Blizzard>) -> bool {
    !child_blizzard
        .iter()
        .map(|b| b.pos)
        .any(|b_pos| b_pos == child_pos)
}

fn both(inp: &(HashSet<Blizzard>, Point, Point, Point)) -> (i64, i64) {
    let (movements, (max_x, max_y), start, exit) = inp;
    simulate(movements, *max_x, *max_y, start, exit)
}

pub fn solve(output: bool) -> Timing {
    let raw_input = include_str!("../input/day24.txt");
    let start = Instant::now();
    let inp = parse(raw_input);
    let parse_time = start.elapsed();
    let p1_time = Duration::new(0, 0);
    let (p1, p2) = both(&inp);
    let p2_time = start.elapsed() - p1_time;
    if output {
        println!("Day 24");
        println!("\tPart 1: {}", p1);
        println!("\tPart 2: {}", p2);
    }
    Timing {
        day: 24,
        parse: parse_time,
        p1: p1_time,
        p2: p2_time,
    }
}
