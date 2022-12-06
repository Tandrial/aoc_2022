use std::time::Instant;

use crate::Timing;

fn parse(inp: &str) -> Vec<(&str, (&str, &str))> {
    let mut backpacks: Vec<(&str, (&str, &str))> = Vec::new();
    for line in inp.lines() {
        backpacks.push((line, line.split_at(line.len() / 2)));
    }
    backpacks
}

fn str_to_int(s: &str) -> u64 {
    let mut res = 0u64;
    // Extract the upper/lower case bit and work with that
    // 'a' 0b1100001
    // 'A' 0b1000001
    //        ^
    for c in s.as_bytes() {
        res |= (((c >> 5) & 1) as u64) << (c - 96);
        res |= (((!c >> 5) & 1) as u64) << (c - 38);
    }
    // for c in s.chars() {
    //     match c {
    //         'a'..='z' => res |= 1 << (c as u32 - 96),
    //         'A'..='Z' => res |= 1 << (c as u32 - 38),
    //         _ => unreachable!(),
    //     }
    // }
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

pub fn solve(output: bool) -> Timing {
    let raw_input = include_str!("../input/day03.txt");
    let start = Instant::now();
    let inp = parse(raw_input);
    let parse_time = start.elapsed();
    let p1 = part1(&inp);
    let p1_time = start.elapsed() - parse_time;
    let p2 = part2(&inp);
    let p2_time = start.elapsed() - p1_time;

    if output {
        println!("Day 03");
        println!("\tPart 1: {}", p1);
        println!("\tPart 2: {}", p2);
    }

    Timing {
        day: 3,
        parse: parse_time,
        p1: p1_time,
        p2: p2_time,
    }
}
