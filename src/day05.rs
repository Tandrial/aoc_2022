use crate::{BorrowTwoMut, Timing};
use std::time::Instant;

type CrateStack = Vec<char>;
type Move = (usize, usize, usize);

fn parse(inp: &str) -> (Vec<CrateStack>, Vec<Move>) {
    let (stacks_str, moves_str) = inp.split_once("\n\n").unwrap();
    let mut stack_lines = stacks_str.lines().rev();

    let count = stack_lines
        .next()
        .and_then(|line| line.split(' ').last())
        .and_then(|last| last.parse::<usize>().ok())
        .unwrap();
    let mut stacks = std::iter::repeat(vec![]).take(count).collect::<Vec<_>>();

    for line in stack_lines {
        for num in 0..count {
            match line.as_bytes().get(1 + num * 4) {
                Some(b' ') | None => {}
                Some(e) => stacks.get_mut(num).unwrap().push(*e as char),
            }
        }
    }

    let moves = moves_str
        .split('\n')
        .map(|line| {
            let elem = line.split(' ').collect::<Vec<&str>>();
            (
                elem.get(1).unwrap().parse::<usize>().unwrap(),
                elem.get(3).unwrap().parse::<usize>().unwrap() - 1,
                elem.get(5).unwrap().parse::<usize>().unwrap() - 1,
            )
        })
        .collect();

    (stacks, moves)
}

fn part1(inp: &(Vec<CrateStack>, Vec<Move>)) -> String {
    apply_moves(inp, true)
}

fn part2(inp: &(Vec<CrateStack>, Vec<Move>)) -> String {
    apply_moves(inp, false)
}

fn apply_moves(inp: &(Vec<CrateStack>, Vec<Move>), lifo: bool) -> String {
    let mut state = inp.0.clone();
    for (cnt, from, to) in inp.1.iter() {
        let (s, d) = state.borrow_two_mut(*from, *to);
        if lifo {
            d.extend(s.drain(s.len() - *cnt..).rev());
        } else {
            d.extend(s.drain((s.len() - *cnt)..));
        }
    }
    state.iter().map(|stack| stack.last().unwrap()).collect()
}

pub fn solve(output: bool) -> Timing {
    let raw_input = include_str!("../aoc_input/2022/day05.txt");
    let start = Instant::now();
    let inp = parse(raw_input);
    let parse_time = start.elapsed();
    let p1 = part1(&inp);
    let p1_time = start.elapsed() - parse_time;
    let p2 = part2(&inp);
    let p2_time = start.elapsed() - p1_time;

    if output {
        println!("Day 05");
        println!("\tPart 1: {}", p1);
        println!("\tPart 2: {}", p2);
    }

    Timing {
        day: 5,
        parse: parse_time,
        p1: p1_time,
        p2: p2_time,
    }
}
