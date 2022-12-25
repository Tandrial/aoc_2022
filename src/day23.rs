use crate::Timing;
use hashbrown::HashSet;
use std::{
    collections::{HashMap, VecDeque},
    time::{Duration, Instant},
};

#[derive(Debug)]
struct Move {
    dir: (i16, i16),
    checks: [(i16, i16); 3],
}

// The coordinates of each elf are encoded into a single number:
// bit 31..16 == x_pos
// bit 15..0  == y_pos
fn build_hash(x: i16, y: i16) -> i32 {
    ((x as i32) << 16) + y as i32
}

fn parse(input: &str) -> HashSet<i32> {
    // We shift by 100 to make sure that the numbers are positive
    let mut elves = HashSet::<i32>::new();
    for (idy, line) in input.lines().enumerate() {
        for (idx, c) in line.chars().enumerate() {
            if c == '#' {
                elves.insert(build_hash((idx + 100) as i16, (idy + 100) as i16));
            }
        }
    }
    elves
}

fn find_size(grid: &HashSet<i32>) -> i64 {
    let mut x_min = i16::MAX;
    let mut x_max = 0;
    let mut y_min = i16::MAX;
    let mut y_max = 0;
    for elf in grid {
        x_min = x_min.min((elf >> 16) as i16);
        x_max = x_max.max((elf >> 16) as i16);

        y_min = y_min.min((elf & 0xFF) as i16);
        y_max = y_max.max((elf & 0xFF) as i16);
    }

    ((x_max - x_min + 1).abs() * (y_max - y_min + 1).abs() - grid.len() as i16).into()
}

static NEIGHBORS: &[(i16, i16)] = &[
    (-1, -1),
    (-1, 0),
    (-1, 1),
    (0, -1),
    (0, 1),
    (1, -1),
    (1, 0),
    (1, 1),
];

fn do_round(grid: &HashSet<i32>, moves: &mut VecDeque<Move>) -> (HashSet<i32>, bool) {
    let mut next_grid = HashSet::<i32>::new();
    let mut prop_moves = HashMap::<i32, Vec<i32>>::new();
    for &p in grid.iter() {
        let x = (p >> 16) as i16;
        let y = (p & 0xFF) as i16;
        //If no other Elves are in one of those eight positions, the Elf does not do anything during this round.
        if !NEIGHBORS
            .iter()
            .any(|&(x_off, y_off)| grid.contains(&build_hash(x + x_off, y + y_off)))
        {
            next_grid.insert(p);
            continue;
        }
        let mut can_move = false;
        for mvn in moves.iter() {
            if !mvn
                .checks
                .iter()
                .any(|&(x_off, y_off)| grid.contains(&build_hash(x + x_off, y + y_off)))
            {
                let (mov_x, mov_y) = mvn.dir;
                can_move = true;
                prop_moves
                    .entry(build_hash(x + mov_x, y + mov_y))
                    .or_default()
                    .push(build_hash(x, y));
                break;
            }
        }
        if !can_move {
            next_grid.insert(build_hash(x, y));
        }
    }
    moves.rotate_left(1);

    let moved = prop_moves.is_empty();
    for (proposal, mut elves) in prop_moves {
        if elves.len() == 1 {
            next_grid.insert(proposal);
        } else {
            next_grid.extend(elves.drain(..));
        }
    }

    (next_grid, moved)
}

fn both(inp: &HashSet<i32>) -> (i64, i64) {
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

    let mut grid = inp.clone();
    (0..10).for_each(|_| (grid, _) = do_round(&grid, &mut moves));
    let part1 = find_size(&grid);

    let mut round = 10;
    let mut done = false;
    while !done {
        (grid, done) = do_round(&grid, &mut moves);
        round += 1;
    }

    (part1, round)
}

pub fn solve(output: bool) -> Timing {
    let raw_input = include_str!("../input/day23.txt");
    let start = Instant::now();
    let inp = parse(raw_input);
    let parse_time = start.elapsed();
    let p1_time = Duration::new(0, 0);
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
