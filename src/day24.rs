use crate::Timing;
use hashbrown::HashSet;
use std::{
    collections::{HashMap, VecDeque},
    time::{Duration, Instant},
};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct BlizzardMap {
    // Blizzards are encoded as an u128 number
    // for the right and left moving blizzards the LSB is the Right edge
    // for the up and down moving blizzards the LSB is the Upper edge
    right: Vec<u128>,
    left: Vec<u128>,
    up: Vec<u128>,
    down: Vec<u128>,
}
impl BlizzardMap {
    fn next(&self, max_x: usize, max_y: usize) -> BlizzardMap {
        let right = self
            .right
            .iter()
            .map(|&val| {
                let idx = (val << 1).leading_zeros();
                if 128 - idx > max_x as u32 && val & (1 << (max_x - 1)) != 0 {
                    val << 1 | 1
                } else {
                    val << 1
                }
            })
            .collect();
        let down = self
            .down
            .iter()
            .map(|&val| {
                let idx = (val << 1).leading_zeros();
                if 128 - idx > max_y as u32 && val & (1 << (max_y - 1)) != 0 {
                    val << 1 | 1
                } else {
                    val << 1
                }
            })
            .collect();

        let left = self
            .left
            .iter()
            .map(|&val| val >> 1 | (val & 1) << (max_x - 1))
            .collect();
        let up = self
            .up
            .iter()
            .map(|&val| val >> 1 | (val & 1) << (max_y - 1))
            .collect();
        BlizzardMap {
            right,
            left,
            up,
            down,
        }
    }
}

type Point = (i64, i64);

fn parse(input: &str) -> (BlizzardMap, usize, usize, Point, Point) {
    let mut start = (-1, -1);
    let mut exit = (-1, -1);
    let max_x = input.lines().next().unwrap().len() - 2;
    let max_y = input.lines().count() - 2;

    let mut right = vec![0u128; max_y];
    let mut left = vec![0u128; max_y];
    let mut up = vec![0u128; max_x];
    let mut down = vec![0u128; max_x];

    for (idy, line) in input.lines().enumerate() {
        for (idx, c) in line.chars().enumerate() {
            let x = idx as i64;
            match c {
                '>' => *right.get_mut(idy - 1).unwrap() |= 1 << (idx - 1),
                '<' => *left.get_mut(idy - 1).unwrap() |= 1 << (idx - 1),
                'v' => *down.get_mut(idx - 1).unwrap() |= 1 << (idy - 1),
                '^' => *up.get_mut(idx - 1).unwrap() |= 1 << (idy - 1),
                '#' => {}
                '.' => {
                    if idy == 0 {
                        start = (x, idy as i64);
                    } else if idy == max_y + 1 {
                        exit = (x, idy as i64);
                    }
                }
                _ => unreachable!(),
            }
        }
    }
    (
        BlizzardMap {
            right,
            left,
            up,
            down,
        },
        max_x,
        max_y,
        start,
        exit,
    )
}

fn simulate(
    movements: &BlizzardMap,
    max_x: usize,
    max_y: usize,
    entry: &Point,
    exit: &Point,
) -> (i64, i64) {
    let mut blizzard_maps = HashMap::<i64, BlizzardMap>::new();
    let mut next_map = movements.clone();

    for t in 0..750 {
        blizzard_maps.insert(t, next_map.clone());
        next_map = next_map.next(max_x, max_y);
    }

    let mut trip_times = [0, 0, 0, 0];

    // Start reversed so that flipping start/target at the start makes it correct
    let mut start = exit;
    let mut target = entry;

    for trip_num in 1..4 {
        // Swap start and target
        (start, target) = (target, start);

        let mut seen = HashSet::<(Point, i64)>::new();
        let mut q = VecDeque::new();
        q.push_back((*start, trip_times[trip_num - 1]));

        while let Some(((cur_x, cur_y), time)) = q.pop_front() {
            if seen.contains(&((cur_x, cur_y), time)) {
                continue;
            }
            seen.insert(((cur_x, cur_y), time));
            let next_blizzard_map = blizzard_maps.get(&(time + 1)).unwrap();
            for (dx, dy) in &[(1, 0), (0, 1), (-1, 0), (0, -1), (0, 0)] {
                let (next_x, next_y) = (cur_x + dx, cur_y + dy);
                // Trip is finished, record time and clear the queue
                if (next_x, next_y) == *target {
                    trip_times[trip_num] = time + 1;
                    q.clear();
                    break;
                }
                // Can't move outside of the blizzard area
                if (1..=max_x).contains(&(next_x as usize))
                    && (1..=max_y).contains(&(next_y as usize))
                // and only if there isn't a blizzard on the square
                // the offset is to shift real world coords to blizzard coords
                    && can_move(next_x - 1, next_y - 1, next_blizzard_map)
                {
                    q.push_back(((next_x, next_y), time + 1));
                }
            }
            // We can wait at the start, which doesn't have blizzards so its always possible
            if (cur_x, cur_y) == *start {
                q.push_back(((cur_x, cur_y), time + 1));
            }
        }
        if trip_num == 3 {
            return (trip_times[1], trip_times[3]);
        }
    }
    unreachable!()
}

#[allow(dead_code)]
fn dump_blizzard(b: &BlizzardMap, max_x: usize, max_y: usize) {
    for y in 0..max_y {
        for x in 0..max_x {
            let right = *b.right.get(y).unwrap() & (1 << x);
            let left = *b.left.get(y).unwrap() & (1 << x);
            let up = *b.up.get(x).unwrap() & (1 << y);
            let down = *b.down.get(x).unwrap() & (1 << y);

            let cnt = [right, left, up, down]
                .iter()
                .filter(|&val| *val > 0)
                .count();

            if cnt > 1 {
                print!("{cnt}")
            } else if right >= 1 {
                print!(">");
            } else if left >= 1 {
                print!("<");
            } else if down >= 1 {
                print!("v");
            } else if up >= 1 {
                print!("^");
            } else {
                print!(".");
            }
        }
        println!();
    }
    println!("\n");
}

fn can_move(x: i64, y: i64, b: &BlizzardMap) -> bool {
    let idx = x as usize;
    let idy = y as usize;

    let right = *b.right.get(idy).unwrap() & (1 << (x));
    let left = *b.left.get(idy).unwrap() & (1 << (x));
    let up = *b.up.get(idx).unwrap() & (1 << (y));
    let down = *b.down.get(idx).unwrap() & (1 << (y));

    right + left + up + down == 0
}

fn both(inp: &(BlizzardMap, usize, usize, Point, Point)) -> (i64, i64) {
    let (movements, max_x, max_y, start, exit) = inp;
    simulate(movements, *max_x, *max_y, start, exit)
}

pub fn solve(output: bool) -> Timing {
    let raw_input = include_str!("../input/day24.txt");
    let start = Instant::now();
    let inp = parse(raw_input);
    let parse_time = start.elapsed();
    let p1_time = Duration::new(0, 0);
    let (p1, p2) = both(&inp);
    let p2_time = start.elapsed() - p1_time;
    if output {
        println!("Day 24");
        println!("\tPart 1: {}", p1);
        println!("\tPart 2: {}", p2);
    }

    Timing {
        day: 24,
        parse: parse_time,
        p1: p1_time,
        p2: p2_time,
    }
}
