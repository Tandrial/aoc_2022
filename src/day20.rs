use crate::Timing;
use std::time::Instant;

fn parse(input: &str) -> Vec<i64> {
    input.lines().map(|line| line.parse().unwrap()).collect()
}

fn mix(nums: &[i64], iterations: usize, decryption_key: i64) -> i64 {
    let values: Vec<i64> = nums.iter().map(|x| x * decryption_key).collect();
    let initial_zero_index = values.iter().position(|&i| i == 0).unwrap();

    let mut idx_array: Vec<usize> = (0..values.len()).collect();

    for _ in 0..iterations {
        for (i, &x) in values.iter().enumerate() {
            let current_index = idx_array.iter().position(|&y| y == i).unwrap();

            idx_array.remove(current_index);

            let new_index = (current_index as i64 + x).rem_euclid(idx_array.len() as i64) as usize;

            idx_array.insert(new_index, i);
        }
    }
    let zero_index = idx_array
        .iter()
        .position(|&i| i == initial_zero_index)
        .unwrap();

    [1000, 2000, 3000]
        .iter()
        .map(|i| values[idx_array[(zero_index + i) % idx_array.len()]])
        .sum()
}

fn part1(inp: &[i64]) -> i64 {
    mix(inp, 1, 1)
}

fn part2(inp: &[i64]) -> i64 {
    mix(inp, 10, 811589153)
}

pub fn solve(output: bool) -> Timing {
    let raw_input = include_str!("../input/day20.txt");
    let start = Instant::now();
    let inp = parse(raw_input);
    let parse_time = start.elapsed();
    let p1 = part1(&inp);
    let p1_time = start.elapsed() - parse_time;
    let p2 = part2(&inp);
    let p2_time = start.elapsed() - p1_time;
    if output {
        println!("Day 20");
        println!("\tPart 1: {}", p1);
        println!("\tPart 2: {}", p2);
    }
    Timing {
        day: 20,
        parse: parse_time,
        p1: p1_time,
        p2: p2_time,
    }
}
