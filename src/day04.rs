use std::{ops::RangeInclusive, time::Instant};

use crate::Timing;

fn get_range(line: &str) -> Option<RangeInclusive<u32>> {
    let mut iter = line.split('-');
    let start: u32 = iter.next()?.parse().ok()?;
    let end = iter.next()?.parse().ok()?;
    Some(start..=end)
}
fn parse(inp: &str) -> Vec<(RangeInclusive<u32>, RangeInclusive<u32>)> {
    inp.lines()
        .map(|line| {
            let mut pair = line.split(',');
            let lhs = get_range(pair.next().unwrap()).unwrap();
            let rhs = get_range(pair.next().unwrap()).unwrap();
            (lhs, rhs)
        })
        .collect::<Vec<(RangeInclusive<u32>, RangeInclusive<u32>)>>()
}

fn part1(inp: &[(RangeInclusive<u32>, RangeInclusive<u32>)]) -> i64 {
    inp.iter()
        .map(|(l, r)| {
            i64::from(
                l.contains(r.start()) && l.contains(r.end())
                    || r.contains(l.start()) && r.contains(l.end()),
            )
        })
        .sum()
}

fn part2(inp: &[(RangeInclusive<u32>, RangeInclusive<u32>)]) -> i64 {
    inp.iter()
        .map(|(l, r)| i64::from(!(l.end() < r.start() || r.end() < l.start())))
        .sum()
}

pub fn solve(output: bool) -> Timing {
    let raw_input = include_str!("../input/day04.txt");
    let start = Instant::now();
    let inp = parse(raw_input);
    let parse_time = start.elapsed();
    let p1 = part1(&inp);
    let p1_time = start.elapsed() - parse_time;
    let p2 = part2(&inp);
    let p2_time = start.elapsed() - p1_time;

    if output {
        println!("Day 04");
        println!("\tPart 1: {}", p1);
        println!("\tPart 2: {}", p2);
    }

    Timing {
        day: 4,
        parse: parse_time,
        p1: p1_time,
        p2: p2_time,
    }
}
