use crate::{index_twice, Point2D, Timing};
use hashbrown::HashSet;
use std::time::{Duration, Instant};

fn parse(inp: &str) -> Vec<(char, u8)> {
    inp.lines()
        .map(|line| {
            let (lhs, rhs) = line.split_once(' ').unwrap();
            (lhs.as_bytes()[0] as char, rhs.parse().unwrap())
        })
        .collect()
}

fn both(inp: &Vec<(char, u8)>) -> (usize, usize) {
    let mut moved_p1 = HashSet::<Point2D<i32>>::new();
    moved_p1.insert((0, 0));
    let mut moved_p2 = HashSet::<Point2D<i32>>::new();
    moved_p2.insert((0, 0));
    let mut rope = [(0, 0); 10];
    for (dir, l) in inp {
        let offset = match &dir {
            'U' => (-1, 0),
            'D' => (1, 0),
            'R' => (0, 1),
            'L' => (0, -1),
            _ => unreachable!(),
        };
        for _ in 0..*l {
            let head = rope.get_mut(0).unwrap();
            head.0 += offset.0;
            head.1 += offset.1;
            for idx in 0..rope.len() - 1 {
                let (h, t) = index_twice::<Point2D<i32>>(&mut rope, idx, idx + 1).unwrap();
                let d = dist(h, t);
                if d > 1 {
                    t.0 += (h.0 - t.0).signum();
                    t.1 += (h.1 - t.1).signum();
                    if idx == 0 {
                        moved_p1.insert(*t);
                    } else if idx == 8 {
                        moved_p2.insert(*t);
                    }
                }
            }
        }
    }
    (moved_p1.len(), moved_p2.len())
}

#[inline(always)]
fn dist(head: &Point2D<i32>, tail: &Point2D<i32>) -> i32 {
    (tail.0 - head.0).abs().max((tail.1 - head.1).abs())
}

pub fn solve(output: bool) -> Timing {
    let raw_input = include_str!("../input/day09.txt");
    let start = Instant::now();
    let inp = parse(raw_input);
    let parse_time = start.elapsed();
    let p1_time = Duration::new(0, 0);
    let (p1, p2) = both(&inp);
    let p2_time = start.elapsed() - p1_time;

    if output {
        println!("Day 09");
        println!("\tPart 1: {}", p1);
        println!("\tPart 2: {}", p2);
    }

    Timing {
        day: 9,
        parse: parse_time,
        p1: p1_time,
        p2: p2_time,
    }
}
