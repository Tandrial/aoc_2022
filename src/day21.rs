use crate::Timing;
use std::{collections::HashMap, time::Instant};

#[derive(Debug, PartialEq)]
enum Monkey<'a> {
    Eq((&'a str, u8, &'a str)),
    Num(f64),
}

fn parse(input: &str) -> HashMap<&str, Monkey> {
    let mut res = HashMap::new();

    for line in input.lines() {
        let (name, eq) = line.split_once(": ").unwrap();
        if eq.contains(' ') {
            let eqq: Vec<&str> = eq.split(' ').collect();
            res.insert(name, Monkey::Eq((eqq[0], eqq[1].as_bytes()[0], eqq[2])));
        } else {
            res.insert(name, Monkey::Num(eq.parse().unwrap()));
        }
    }
    res
}

fn calc(inp: &HashMap<&str, Monkey>) -> (f64, f64) {
    let mut look_up = HashMap::<&str, f64>::new();

    let mut vec = Vec::from_iter(inp.iter());
    let mut vec2: Vec<(&&str, &Monkey)> = vec![];
    while !vec.is_empty() {
        for (name, eq) in vec {
            match eq {
                Monkey::Num(val) => {
                    look_up.insert(*name, *val);
                }
                Monkey::Eq((lhs, op, rhs)) => {
                    if look_up.contains_key(lhs) && look_up.contains_key(rhs) {
                        let l = *look_up.get(lhs).unwrap();
                        let r = *look_up.get(rhs).unwrap();
                        if *name == "root" {
                            return (l, r);
                        }
                        match &op {
                            b'+' => look_up.insert(name, l + r),
                            b'-' => look_up.insert(name, l - r),
                            b'*' => look_up.insert(name, l * r),
                            b'/' => look_up.insert(name, l / r),
                            _ => unreachable!(),
                        };
                    } else {
                        vec2.push((name, eq));
                    }
                }
            };
        }
        vec = vec2;
        vec2 = vec![];
    }
    unreachable!()
}

fn part1(inp: &HashMap<&str, Monkey>) -> i64 {
    let (l, r) = calc(inp);
    (l + r) as i64
}

fn part2(inp: &mut HashMap<&str, Monkey>) -> i64 {
    // Since the function being built is linear we can simply interpolate between two points
    // and solve for x == 0, pick 2 points very far away so make sure we round to the correct point

    let x1 = 100_000_000_000f64;
    inp.insert("humn", Monkey::Num(x1));
    let (l, r) = calc(inp);
    let r1 = l - r;

    let x2 = -x1;
    inp.insert("humn", Monkey::Num(x2));
    let (l2, r2) = calc(inp);
    let r2 = l2 - r2;

    // Two point form
    let res = (x1 * r2 - x2 * r1) / (r2 - r1);

    res.round() as i64
}

pub fn solve(output: bool) -> Timing {
    let raw_input = include_str!("../aoc_input/2022/day21.txt");
    let start = Instant::now();
    let mut inp = parse(raw_input);
    let parse_time = start.elapsed();
    let p1 = part1(&inp);
    let p1_time = start.elapsed() - parse_time;
    let p2 = part2(&mut inp);
    let p2_time = start.elapsed() - p1_time;
    if output {
        println!("Day 21");
        println!("\tPart 1: {}", p1);
        println!("\tPart 2: {}", p2);
    }

    Timing {
        day: 21,
        parse: parse_time,
        p1: p1_time,
        p2: p2_time,
    }
}
