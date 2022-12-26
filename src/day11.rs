use crate::Timing;
use std::time::Instant;

struct Monkey {
    id: usize,
    bag: Vec<usize>,
    op: Box<dyn Fn(usize) -> usize>,
    test: Box<dyn Fn(usize, usize) -> usize>,
    div: usize,
    ins: u64,
}

fn parse(inp: &str) -> Vec<Monkey> {
    let mut id = -1;
    inp.split("\n\n")
        .map(|m| {
            let l: Vec<_> = m.lines().map(|l| l.split(": ").last().unwrap()).collect();
            id += 1;
            let div: usize = l[3].rsplit_once(' ').unwrap().1.parse().unwrap();
            let test_true: usize = l[4].rsplit_once(' ').unwrap().1.parse().unwrap();
            let test_false: usize = l[5].rsplit_once(' ').unwrap().1.parse().unwrap();
            Monkey {
                id: id as usize,
                bag: l[1].split(", ").map(|n| n.parse().unwrap()).collect(),
                op: {
                    let op: Vec<_> = l[2].rsplit_once("= ").unwrap().1.split(' ').collect();
                    match op[2] {
                        "old" => Box::new(|old| old * old),
                        b => match (op[1], b.parse::<usize>().unwrap()) {
                            ("+", n) => Box::new(move |old| old + n),
                            ("*", n) => Box::new(move |old| old * n),
                            _ => unreachable!(),
                        },
                    }
                },
                div,
                test: Box::new(move |elem, div| {
                    if elem % div == 0 {
                        test_true
                    } else {
                        test_false
                    }
                }),
                ins: 0,
            }
        })
        .collect()
}

fn calc(monkeys: &mut [Monkey], count: usize, calm_down: usize) -> u64 {
    let (wrap_worry, mut bags): (usize, _) = (
        monkeys.iter().map(|m| m.div).product(),
        vec![vec![]; monkeys.len()],
    );

    (0..count).for_each(|_| {
        monkeys.iter_mut().for_each(|monkey| {
            monkey.bag.append(&mut bags[monkey.id]);
            monkey.bag.drain(0..).for_each(|worry_level| {
                let new_worry_level = (monkey.op)(worry_level) / calm_down % wrap_worry;
                let target_monkey = (monkey.test)(new_worry_level, monkey.div);
                bags[target_monkey].push(new_worry_level);
                monkey.ins += 1;
            });
        });
    });
    let mut fst = 0;
    let mut snd = 0;
    for m in monkeys {
        if m.ins > fst {
            snd = fst;
            fst = m.ins;
        } else if m.ins > snd {
            snd = m.ins;
        }
    }
    fst * snd
}

fn part1(monkeys: &mut [Monkey]) -> u64 {
    calc(monkeys, 20, 3)
}

fn part2(monkeys: &mut [Monkey]) -> u64 {
    calc(monkeys, 10000, 1)
}

pub fn solve(output: bool) -> Timing {
    let raw_input = include_str!("../input/day11.txt");
    let start = Instant::now();
    let mut inp = parse(raw_input);
    let mut inp2 = parse(raw_input);
    let parse_time = start.elapsed();
    let p1 = part1(&mut inp);
    let p1_time = start.elapsed() - parse_time;
    let p2 = part2(&mut inp2);
    let p2_time = start.elapsed() - p1_time;

    if output {
        println!("Day 11");
        println!("\tPart 1: {}", p1);
        println!("\tPart 2: {}", p2);
    }

    Timing {
        day: 11,
        parse: parse_time,
        p1: p1_time,
        p2: p2_time,
    }
}
