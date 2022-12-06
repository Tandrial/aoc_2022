use crate::Timing;
use std::time::Instant;

#[inline(always)]
fn to_shift(c: u8) -> u32 {
    // to make the lowercase chars a..z fit into an u32 we can use mod 32 to have to number wrap around
    1 << (c % 32)
}

fn find_offset(inp: &str, size: usize) -> usize {
    let mut mask = 0u32;
    let data = inp.as_bytes();

    (0..size).for_each(|idx| mask ^= to_shift(data[idx]));
    for idx in size..data.len() {
        if mask.count_ones() as usize == size {
            return idx;
        }
        mask ^= to_shift(data[idx - size]);
        mask ^= to_shift(data[idx]);
    }
    unreachable!()
}

fn part1(inp: &str) -> usize {
    find_offset(inp, 4)
}

fn part2(inp: &str) -> usize {
    find_offset(inp, 14)
}

pub fn solve(output: bool) -> Timing {
    let raw_input = include_str!("../input/day06.txt");
    let start = Instant::now();
    let parse_time = start.elapsed();
    let p1 = part1(raw_input);
    let p1_time = start.elapsed() - parse_time;
    let p2 = part2(raw_input);
    let p2_time = start.elapsed() - p1_time;

    if output {
        println!("Day 06");
        println!("\tPart 1: {}", p1);
        println!("\tPart 2: {}", p2);
    }

    Timing {
        day: 6,
        parse: parse_time,
        p1: p1_time,
        p2: p2_time,
    }
}
