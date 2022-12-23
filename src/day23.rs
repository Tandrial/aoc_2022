use crate::Timing;
use std::{
    collections::{HashMap, HashSet, VecDeque},
    time::Instant,
};

#[derive(Debug)]
struct Move {
    dir: (i64, i64),
    checks: [(i64, i64); 3],
}

fn parse(input: &str) -> HashSet<(i64, i64)> {
    let mut elves = HashSet::new();
    for (idy, line) in input.lines().enumerate() {
        for (idx, c) in line.chars().enumerate() {
            if c == '#' {
                elves.insert((idx as i64, idy as i64));
            }
        }
    }
    elves
}

fn find_size(grid: &HashSet<(i64, i64)>) -> i64 {
    let x_min = grid.iter().map(|(x, _)| x).min().unwrap();
    let y_min = grid.iter().map(|(_, y)| y).min().unwrap();

    let x_max = grid.iter().map(|(x, _)| x).max().unwrap();
    let y_max = grid.iter().map(|(_, y)| y).max().unwrap();

    (x_max - x_min + 1).abs() * (y_max - y_min + 1).abs() - grid.len() as i64
}

fn both(inp: &HashSet<(i64, i64)>) -> (i64, i64) {
    let neighbours = vec![
        (-1, -1),
        (-1, 0),
        (-1, 1),
        (0, -1),
        (0, 1),
        (1, -1),
        (1, 0),
        (1, 1),
    ];

    let mut grid = inp.clone();

    let mut moves = VecDeque::new();
    //If there is no Elf in the N, NE, or NW adjacent positions, the Elf proposes moving north one step.
    moves.push_back(Move {
        dir: (0, -1),
        checks: [(0, -1), (1, -1), (-1, -1)],
    });
    //If there is no Elf in the S, SE, or SW adjacent positions, the Elf proposes moving south one step.
    moves.push_back(Move {
        dir: (0, 1),
        checks: [(0, 1), (1, 1), (-1, 1)],
    });
    //If there is no Elf in the W, NW, or SW adjacent positions, the Elf proposes moving west one step.
    moves.push_back(Move {
        dir: (-1, 0),
        checks: [(-1, 0), (-1, -1), (-1, 1)],
    });
    //If there is no Elf in the E, NE, or SE adjacent positions, the Elf proposes moving east one step.
    moves.push_back(Move {
        dir: (1, 0),
        checks: [(1, 0), (1, -1), (1, 1)],
    });

    let mut round = 0;
    let mut part1 = 0;
    loop {
        if round == 10 {
            part1 = find_size(&grid);
        }
        let mut next_grid = HashSet::<(i64, i64)>::new();
        let mut prop_moves = HashMap::<(i64, i64), Vec<(i64, i64)>>::new();
        for &(x, y) in grid.iter() {
            //If no other Elves are in one of those eight positions, the Elf does not do anything during this round.
            if !neighbours
                .iter()
                .any(|&(x_off, y_off)| grid.contains(&(x + x_off, y + y_off)))
            {
                next_grid.insert((x, y));
                continue;
            }
            let mut can_move = false;
            for mvn in moves.iter() {
                if !mvn
                    .checks
                    .iter()
                    .any(|&(x_off, y_off)| grid.contains(&(x + x_off, y + y_off)))
                {
                    let (mov_x, mov_y) = mvn.dir;
                    let target = (x + mov_x, y + mov_y);
                    can_move = true;
                    prop_moves.entry(target).or_default().push((x, y));
                    break;
                }
            }
            if !can_move {
                next_grid.insert((x, y));
            }
        }

        if prop_moves.is_empty() {
            return (part1, round + 1);
        }

        for (proposal, mut elves) in prop_moves {
            if elves.len() == 1 {
                next_grid.insert(proposal);
            } else {
                next_grid.extend(elves.drain(..));
            }
        }

        let tmp = moves.pop_front().unwrap();
        moves.push_back(tmp);
        grid = next_grid;
        round += 1;
    }
}

pub fn solve(output: bool) -> Timing {
    let raw_input = include_str!("../input/day23.txt");
    let start = Instant::now();
    let inp = parse(raw_input);
    let parse_time = start.elapsed();
    let p1_time = start.elapsed() - parse_time;
    let (p1, p2) = both(&inp);
    let p2_time = start.elapsed() - p1_time;
    if output {
        println!("Day 23");
        println!("\tPart 1: {}", p1);
        println!("\tPart 2: {}", p2);
    }
    Timing {
        day: 23,
        parse: parse_time,
        p1: p1_time,
        p2: p2_time,
    }
}
