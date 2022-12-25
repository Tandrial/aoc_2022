use crate::Timing;
use itertools::{EitherOrBoth, Itertools};
use std::{
    collections::HashMap,
    time::{Duration, Instant},
};

fn parse(input: &str) -> Vec<&str> {
    input.lines().collect()
}

fn sum_snafu(a: String, b: &str) -> String {
    let mut look_up = HashMap::<char, i8>::new();
    look_up.insert('2', 2);
    look_up.insert('1', 1);
    look_up.insert('0', 0);
    look_up.insert('-', -1);
    look_up.insert('=', -2);
    let mut rev_look_up = HashMap::<i8, char>::new();
    for (k, v) in look_up.iter() {
        rev_look_up.insert(*v, *k);
    }

    let digits = a.chars().zip_longest(b.chars().rev());
    let mut result = String::new();
    let mut carry = 0i8;
    for p in digits {
        let mut val = match p {
            EitherOrBoth::Both(lhs, rhs) => {
                look_up.get(&lhs).unwrap() + look_up.get(&rhs).unwrap() + carry
            }
            EitherOrBoth::Left(val) | EitherOrBoth::Right(val) => {
                look_up.get(&val).unwrap() + carry
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

        result.push(*rev_look_up.get(&val).unwrap());
    }
    if carry != 0 {
        result.push(*rev_look_up.get(&carry).unwrap());
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
