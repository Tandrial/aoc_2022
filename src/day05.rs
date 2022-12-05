use std::time::Instant;

type CrateStack = Vec<char>;
type Move = (usize, usize, usize);

fn parse(inp: &str) -> (Vec<CrateStack>, Vec<Move>) {
    let inp_split: Vec<&str> = inp.split("\n\n").collect();

    let mut stacks_raw = inp_split.first().unwrap().lines().rev();
    let stack_count = stacks_raw.next().unwrap().split("   ").count();
    let mut stacks = std::iter::repeat(vec![])
        .take(stack_count)
        .collect::<Vec<_>>();

    for line in stacks_raw {
        for num in 0..stack_count {
            match line.as_bytes().get(1 + num * 4) {
                Some(b' ') => {}
                Some(e) => stacks.get_mut(num).unwrap().push(*e as char),
                None => {}
            }
        }
    }

    let moves = inp_split
        .get(1)
        .unwrap()
        .split('\n')
        .map(|line| {
            let elem = line.split(' ').collect::<Vec<&str>>();
            (
                elem.get(1).unwrap().parse::<usize>().unwrap(),
                elem.get(3).unwrap().parse::<usize>().unwrap(),
                elem.get(5).unwrap().parse::<usize>().unwrap(),
            )
        })
        .collect();

    (stacks, moves)
}

fn part1(inp: &(Vec<CrateStack>, Vec<Move>)) -> String {
    let mut state = inp.0.clone();
    for (cnt, from, to) in inp.1.iter() {
        for _ in 0..*cnt {
            let f_stack = state.get_mut(*from - 1).unwrap();
            let c = f_stack.pop().unwrap();
            let t_stack = state.get_mut(*to - 1).unwrap();
            t_stack.push(c);
        }
    }
    state.iter().map(|stack| stack.last().unwrap()).collect()
}

fn part2(inp: &(Vec<CrateStack>, Vec<Move>)) -> String {
    let mut state = inp.0.clone();
    for (cnt, from, to) in inp.1.iter() {
        let source = state.get_mut(*from - 1).unwrap();
        let tmp: Vec<char> = source.drain((source.len() - *cnt)..).collect();
        let dest = state.get_mut(*to - 1).unwrap();
        dest.extend(tmp);
    }
    state.iter().map(|stack| stack.last().unwrap()).collect()
}

pub fn solve() {
    let raw_input = include_str!("../input/day05.txt");
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
