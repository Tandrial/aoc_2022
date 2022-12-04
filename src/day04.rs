use std::{ops::RangeInclusive, time::Instant};

fn get_range(line: &str) -> Option<RangeInclusive<u32>> {
    let mut iter = line.split('-');
    let start: u32 = iter.next()?.parse().ok()?;
    let end = iter.next()?.parse().ok()?;
    Some(start..=end)
}
fn parse(inp: &str) -> Vec<(RangeInclusive<u32>, RangeInclusive<u32>)> {
    let mut res: Vec<(RangeInclusive<u32>, RangeInclusive<u32>)> = Vec::new();
    for line in inp.lines() {
        let mut pair = line.split(',');
        let lhs = get_range(pair.next().unwrap()).unwrap();
        let rhs = get_range(pair.next().unwrap()).unwrap();
        res.push((lhs, rhs));
    }
    res
}

fn part1(inp: &[(RangeInclusive<u32>, RangeInclusive<u32>)]) -> i64 {
    let mut score: i64 = 0;
    for (lhs, rhs) in inp {
        if lhs.contains(rhs.start()) && lhs.contains(rhs.end())
            || rhs.contains(lhs.start()) && rhs.contains(lhs.end())
        {
            score += 1;
        }
    }
    score
}

fn part2(inp: &[(RangeInclusive<u32>, RangeInclusive<u32>)]) -> i64 {
    let mut score: i64 = 0;
    for (lhs, rhs) in inp {
        if !(lhs.end() < rhs.start() || rhs.end() < lhs.start()) {
            score += 1
        }
    }
    score
}

pub fn solve() {
    let raw_input = include_str!("../input/day04.txt");
    let start = Instant::now();
    let inp = parse(raw_input);
    let parse_time = start.elapsed();
    println!("Part 1: {}", part1(&inp));
    let p1_time = start.elapsed() - parse_time;
    println!("Part 2: {}", part2(&inp));
    let p2_time = start.elapsed() - p1_time;

    println!(
        "Parsing={} µs, p1={} µs, p2={} µs",
        parse_time.as_micros(),
        p1_time.as_micros(),
        p2_time.as_micros()
    );
}
