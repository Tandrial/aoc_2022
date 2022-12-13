use crate::Timing;
use pest::{iterators::Pair, Parser};
use std::{cmp::Ordering, time::Instant};

#[derive(Parser)]
#[grammar = "day13.pest"]
struct PacketParser;

#[derive(Debug, Eq, PartialEq)]
enum Packet {
    List(Vec<Packet>),
    Num(i32),
}

impl PartialOrd for Packet {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Packet {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match (self, other) {
            // Num vs. Num
            (Packet::Num(lhs), Packet::Num(rhs)) => lhs.cmp(rhs),
            // List vs Num
            (Packet::Num(lhs), Packet::List(_)) => Packet::List(vec![Packet::Num(*lhs)]).cmp(other),
            (Packet::List(_), Packet::Num(rhs)) => self.cmp(&Packet::List(vec![Packet::Num(*rhs)])),
            // List vs List
            (Packet::List(lhs), Packet::List(rhs)) => {
                for (l, r) in lhs.iter().zip(rhs.iter()) {
                    let res = l.cmp(r);
                    if res != Ordering::Equal {
                        return res;
                    }
                }
                lhs.len().cmp(&rhs.len())
            }
        }
    }
}

fn parse_rec(pair: Pair<Rule>) -> Packet {
    match pair.as_rule() {
        Rule::file => unimplemented!(),
        Rule::expression => parse_rec(pair.into_inner().next().unwrap()),
        Rule::list => Packet::List(pair.into_inner().map(parse_rec).collect()),
        Rule::number => Packet::Num(pair.as_str().parse().unwrap()),
    }
}

fn parse(input: &str) -> Vec<(Packet, Packet)> {
    let lines = input.split("\n\n");
    let res: Vec<(Packet, Packet)> = lines
        .map(|line| {
            let pairs = PacketParser::parse(Rule::file, line).unwrap();

            let mut res = pairs
                .into_iter()
                .next()
                .unwrap()
                .into_inner()
                .map(parse_rec);

            (res.next().unwrap(), res.next().unwrap())
        })
        .collect();
    res
}

fn part1(inp: &[(Packet, Packet)]) -> usize {
    let mut p1 = 0;
    for (idx, (l, r)) in inp.iter().enumerate() {
        if l < r {
            p1 += idx + 1;
        }
    }
    p1
}

fn part2(inp: &mut [(Packet, Packet)]) -> usize {
    let mark_1 = Packet::List(vec![Packet::Num(2)]);
    let mark_2 = Packet::List(vec![Packet::Num(6)]);
    let mut packets = vec![&mark_1, &mark_2];
    for (l, r) in inp {
        packets.push(l);
        packets.push(r);
    }
    packets.sort();

    let idx_1 = 1 + packets.iter().position(|p| *p == &mark_1).unwrap_or(0);
    let idx_2 = 1 + packets.iter().position(|p| *p == &mark_2).unwrap_or(0);

    idx_1 * idx_2
}

pub fn solve(output: bool) -> Timing {
    let raw_input = include_str!("../input/day13.txt");
    let start = Instant::now();
    let mut inp = parse(raw_input);
    let parse_time = start.elapsed();
    let p1 = part1(&inp);
    let p1_time = start.elapsed() - parse_time;
    let p2 = part2(&mut inp);
    let p2_time = start.elapsed() - p1_time;

    if output {
        println!("Day 13");
        println!("\tPart 1: {}", p1);
        println!("\tPart 2: {}", p2);
    }
    Timing {
        day: 13,
        parse: parse_time,
        p1: p1_time,
        p2: p2_time,
    }
}
