use crate::Timing;
use std::time::{Duration, Instant};

fn parse(inp: &str) -> Vec<(&str, i32)> {
    inp.lines()
        .map(|line| match line.split_once(' ') {
            Some((lhs, rhs)) => (lhs, rhs.parse().unwrap()),
            None => (line, 0),
        })
        .collect()
}

fn get_pixel(cycle: i32, reg_x: i32) -> String {
    let mut res: String = "".into();
    if cycle % 40 == 0 {
        res += "\n";
    }
    let diff: i32 = (reg_x - (cycle % 40)).abs();
    if diff <= 1 {
        res += "█";
    } else {
        res += " ";
    }
    res
}

fn both(inp: &[(&str, i32)]) -> (i32, String) {
    let mut p1 = 0;
    let mut p2: String = "".into();
    let mut reg_x = 1;
    let mut cycle = 0;
    let checks = [20, 60, 100, 140, 180, 220];
    for (inst, val) in inp {
        p2 += &get_pixel(cycle, reg_x);
        cycle += 1;
        if checks.contains(&cycle) {
            p1 += cycle * reg_x;
        }
        // Just check if the instruction ist addx, since noop does nothing and the cycle += 1 is already handled
        if *inst == "addx" {
            p2 += &get_pixel(cycle, reg_x);
            cycle += 1;

            if checks.contains(&cycle) {
                p1 += cycle * reg_x;
            }
            reg_x += val;
        }
    }
    (p1, p2)
}

pub fn solve(output: bool) -> Timing {
    let raw_input = include_str!("../aoc_input/2022/day10.txt");
    let start = Instant::now();
    let inp = parse(raw_input);
    let parse_time = start.elapsed();
    let p1_time = Duration::new(0, 0);
    let (p1, p2) = both(&inp);
    let p2_time = start.elapsed() - p1_time;

    if output {
        println!("Day 10");
        println!("\tPart 1: {}", p1);
        println!("\tPart 2: {}", p2);
    }

    Timing {
        day: 10,
        parse: parse_time,
        p1: p1_time,
        p2: p2_time,
    }
}
