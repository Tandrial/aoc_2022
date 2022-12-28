use crate::Timing;
use itertools::{EitherOrBoth, Itertools};
use std::time::{Duration, Instant};

fn parse(input: &str) -> Vec<&str> {
    input.lines().collect()
}
// char - 45 == idx
static STODEC: [i8; 17] = [-1i8, 0, 0, 0, 1, 2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -2];
static DECTOS: [char; 5] = ['=', '-', '0', '1', '2'];

fn sum_snafu(a: String, b: &str) -> String {
    let digits = a.bytes().zip_longest(b.bytes().rev());
    let mut result = String::new();
    let mut carry = 0i8;
    for p in digits {
        let mut val = match p {
            EitherOrBoth::Both(lhs, rhs) => {
                STODEC[(lhs - 45) as usize] + STODEC[(rhs - 45) as usize] + carry
            }
            EitherOrBoth::Left(val) | EitherOrBoth::Right(val) => {
                STODEC[(val - 45) as usize] + carry
            }
        };
        if val > 2 {
            carry = 1;
            val -= 5;
        } else if val < -2 {
            carry = -1;
            val += 5;
        } else {
            carry = 0;
        }

        result.push(DECTOS[(val + 2) as usize]);
    }
    if carry != 0 {
        result.push(DECTOS[(carry + 2) as usize]);
    }

    result
}

fn part1(inp: &[&str]) -> String {
    inp.iter()
        .fold("".to_string(), |acc: String, elem: &&str| {
            sum_snafu(acc, elem)
        })
        .chars()
        .rev()
        .collect()
}

pub fn solve(output: bool) -> Timing {
    let raw_input = include_str!("../input/day25.txt");
    let start = Instant::now();
    let inp = parse(raw_input);
    let parse_time = start.elapsed();
    let p1 = part1(&inp);
    let p1_time = start.elapsed() - parse_time;
    let p2_time = Duration::new(0, 0);
    if output {
        println!("Day 25");
        println!("\tPart 1: {}", p1);
    }

    Timing {
        day: 25,
        parse: parse_time,
        p1: p1_time,
        p2: p2_time,
    }
}
