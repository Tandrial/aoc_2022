use std::time::Instant;

fn parse(inp: &str) -> Vec<i64> {
    let mut total_count: Vec<i64> = Vec::new();
    let mut current: i64 = 0;
    for line in inp.lines() {
        match line.parse::<i64>() {
            Ok(value) => current += value,
            Err(_) => {
                total_count.push(current);
                current = 0;
            }
        };
    }
    total_count.sort();
    total_count
}

fn part1(inp: &[i64]) -> i64 {
    inp[inp.len() - 1]
}

fn part2(inp: &[i64]) -> i64 {
    inp[inp.len() - 3..].iter().sum()
}

pub fn solve() {
    let raw_input = include_str!("../input/day01.txt");
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
