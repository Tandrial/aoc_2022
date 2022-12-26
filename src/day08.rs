use grid::Grid;

use crate::{Point2D, Timing};
use std::time::{Duration, Instant};

fn parse(input: &str) -> Grid<u8> {
    let (mut h, mut w) = (0, 0);
    for line in input.lines() {
        h += 1;
        w = w.max(line.len());
    }

    let mut grid = Grid::<u8>::new(h, w);
    for (idy, line) in input.lines().enumerate() {
        for (idx, num) in line.as_bytes().iter().enumerate() {
            grid[idy][idx] = num - b'0';
        }
    }
    grid
}

fn both(inp: &Grid<u8>) -> Point2D {
    let (h, w) = inp.size();
    let mut p1 = 2 * h + 2 * (w - 2);
    let mut p2 = 0;

    for y in 1..h - 1 {
        for x in 1..w - 1 {
            let mut score = 1;
            let mut can_see = false;
            for (dx, dy) in &[(0, -1i64), (0, 1), (1, 0), (-1i64, 0)] {
                let mut view_distance = 0;
                let mut blocked = false;
                let (mut cur_x, mut cur_y) = (x as i64 + dx, y as i64 + dy);
                while (0..w).contains(&(cur_x as usize)) && (0..h).contains(&(cur_y as usize)) {
                    view_distance += 1;
                    if inp[cur_y as usize][cur_x as usize] >= inp[y][x] {
                        blocked |= true;
                        break;
                    }
                    cur_x += dx;
                    cur_y += dy;
                }
                score *= view_distance;
                can_see |= !blocked;
            }
            p2 = p2.max(score);
            p1 += can_see as usize;
        }
    }
    (p1 as i32, p2)
}

pub fn solve(output: bool) -> Timing {
    let raw_input = include_str!("../input/day08.txt");
    let start = Instant::now();
    let inp = parse(raw_input);
    let parse_time = start.elapsed();
    let p1_time = Duration::new(0, 0);
    let (p1, p2) = both(&inp);
    let p2_time = start.elapsed() - p1_time;

    if output {
        println!("Day 08");
        println!("\tPart 1: {}", p1);
        println!("\tPart 2: {}", p2);
    }

    Timing {
        day: 8,
        parse: parse_time,
        p1: p1_time,
        p2: p2_time,
    }
}
