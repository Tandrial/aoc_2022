use crate::Timing;
use std::{cmp::Ordering, collections::HashMap, time::Instant};

#[derive(Debug, PartialEq)]
enum Monkey {
    Eq((String, u8, String)),
    Num(i64),
}

fn parse(input: &str) -> HashMap<&str, Monkey> {
    let mut res = HashMap::new();

    for line in input.lines() {
        let (name, eq) = line.split_once(": ").unwrap();
        if eq.contains(' ') {
            let eqq: Vec<&str> = eq.split(' ').collect();
            res.insert(
                name,
                Monkey::Eq((eqq[0].to_string(), eqq[1].as_bytes()[0], eqq[2].to_string())),
            );
        } else {
            res.insert(name, Monkey::Num(eq.parse().unwrap()));
        }
    }
    res
}

fn calc(inp: &HashMap<&str, Monkey>) -> (i64, i64) {
    let mut look_up = HashMap::<&str, i64>::new();

    let mut vec = Vec::from_iter(inp.iter());
    let mut vec2: Vec<(&&str, &Monkey)> = vec![];
    while !vec.is_empty() {
        for (name, eq) in vec {
            match eq {
                Monkey::Num(val) => {
                    look_up.insert(*name, *val);
                }
                Monkey::Eq((lhs, op, rhs)) => {
                    if look_up.contains_key(lhs.as_str()) && look_up.contains_key(rhs.as_str()) {
                        let l = look_up.get(lhs.as_str()).unwrap();
                        let r = look_up.get(rhs.as_str()).unwrap();
                        if *name == "root" {
                            return (*l, *r);
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
    l + r
}

fn part2(inp: &mut HashMap<&str, Monkey>) -> i64 {
    let (mut l, mut r) = calc(inp);
    while l - r > 0 {
        let hum = inp.get("humn").unwrap();
        match hum {
            Monkey::Num(val) => inp.insert("humn", Monkey::Num(val * 1000000)),
            _ => unreachable!(),
        };
        (l, r) = calc(inp);
    }
    let mut low = 0;
    let mut high = match inp.get("humn").unwrap() {
        Monkey::Num(val) => *val,
        _ => unreachable!(),
    };

    while l - r != 0 {
        let human = low + ((high - low) / 2);
        inp.insert("humn", Monkey::Num(human));

        (l, r) = calc(inp);
        match l.cmp(&r) {
            Ordering::Greater => {
                low = human;
            }
            Ordering::Less => {
                high = human;
            }
            Ordering::Equal => {
                return human;
            }
        }
    }
    unreachable!()
}

pub fn solve(output: bool) -> Timing {
    let raw_input = include_str!("../input/day21.txt");
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
