use super::*;
use std::time::Instant;

fn parse(inp: &str) -> Vec<(&str, (&str, &str))> {
    let mut total_count: Vec<(&str, (&str, &str))> = Vec::new();
    for line in inp.lines() {
        total_count.push((line, line.split_at(line.len() / 2)));
    }
    total_count
}

fn str_to_int(s: &str) -> u64 {
    let mut res = 0u64;
    for c in s.chars() {
        match c {
            'a'..='z' => res |= 1 << (c as u32 - 96),
            'A'..='Z' => res |= 1 << (c as u32 - 38),
            _ => unreachable!(),
        }
    }
    res
}

fn part1(inp: &[(&str, (&str, &str))]) -> i64 {
    let mut score: i64 = 0;
    for (_, (lhs, rhs)) in inp {
        let num_l = str_to_int(lhs);
        let num_r = str_to_int(rhs);
        let res = num_l & num_r;
        score += res.trailing_zeros() as i64;
    }
    score
}

fn part2(inp: &[(&str, (&str, &str))]) -> i64 {
    let mut score: i64 = 0;
    for e in inp.chunks(3) {
        let s1 = str_to_int(e.get(0).unwrap().0);
        let s2 = str_to_int(e.get(1).unwrap().0);
        let s3 = str_to_int(e.get(2).unwrap().0);

        let res = s1 & s2 & s3;
        score += res.trailing_zeros() as i64
    }
    score
}

pub fn solve() {
    let raw_input = get_input(3);
    let start = Instant::now();
    let inp = parse(&raw_input);
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
