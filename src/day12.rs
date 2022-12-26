use crate::{Point2D, Timing};
use grid::Grid;
use hashbrown::HashSet;
use std::{collections::VecDeque, time::Instant};

fn parse(input: &str) -> (Grid<u8>, Point2D, Point2D) {
    let (mut h, mut w) = (0, 0);
    for line in input.lines() {
        h += 1;
        w = w.max(line.len());
    }

    let mut start = (0, 0);
    let mut end = (0, 0);
    let mut grid = Grid::<u8>::new(h, w);
    for (idy, line) in input.lines().enumerate() {
        for (idx, &num) in line.as_bytes().iter().enumerate() {
            if num == b'S' {
                start = (idx as i32, idy as i32);
                grid[idy][idx] = b'a';
            } else if num == b'E' {
                end = (idx as i32, idy as i32);
                grid[idy][idx] = b'z';
            } else {
                grid[idy][idx] = num;
            }
        }
    }
    (grid, start, end)
}

fn part1_bfs(inp: &(Grid<u8>, Point2D, Point2D)) -> usize {
    let (grid, start, end) = inp;

    let end_check = |p1: &Point2D, p2: &Point2D| p1 == p2;
    let calc_diff = |cur: u8, neighbor: u8| neighbor as i32 - cur as i32;

    bfs(grid, start, end, end_check, calc_diff)
}

fn part2_bfs(inp: &(Grid<u8>, Point2D, Point2D)) -> usize {
    let (grid, end, start) = inp;

    let end_check = |p1: &Point2D, _: &Point2D| p1.0 == 0;
    let calc_diff = |cur: u8, neighbor: u8| cur as i32 - neighbor as i32;

    bfs(grid, start, end, end_check, calc_diff)
}

fn bfs<T, F>(grid: &Grid<u8>, start: &Point2D, end: &Point2D, end_check: T, calc_diff: F) -> usize
where
    T: Fn(&Point2D, &Point2D) -> bool,
    F: Fn(u8, u8) -> i32,
{
    let (h, w) = grid.size();

    let mut result = 0;
    let mut q = VecDeque::new();
    let mut seen = HashSet::<Point2D>::new();
    q.push_back((*start, "".to_string()));
    seen.insert(*start);

    while let Some(((next_x, next_y), path)) = q.pop_front() {
        if end_check(&(next_x, next_y), end) {
            result = path.len();
            break;
        }
        let height = grid[next_y as usize][next_x as usize];
        for (dx, dy) in &[(0, -1), (0, 1), (1, 0), (-1, 0)] {
            let (child_x, child_y) = (next_x + dx, next_y + dy);
            if (0..w).contains(&(child_x as usize)) && (0..h).contains(&(child_y as usize)) {
                let c_height = grid[child_y as usize][child_x as usize];
                let diff = calc_diff(height, c_height);
                if diff <= 1 && !seen.contains(&(child_x, child_y)) {
                    seen.insert((child_x, child_y));
                    let mut path_new = path.clone();
                    path_new.push(c_height as char);
                    q.push_back(((child_x, child_y), path_new));
                }
            }
        }
    }
    result
}

pub fn solve(output: bool) -> Timing {
    let raw_input = include_str!("../input/day12.txt");
    let start = Instant::now();
    let inp = parse(raw_input);
    let parse_time = start.elapsed();
    let p1 = part1_bfs(&inp);
    let p1_time = start.elapsed() - parse_time;
    let p2 = part2_bfs(&inp);
    let p2_time = start.elapsed() - p1_time;

    if output {
        println!("Day 12");
        println!("\tPart 1: {}", p1);
        println!("\tPart 2: {}", p2);
    }

    Timing {
        day: 12,
        parse: parse_time,
        p1: p1_time,
        p2: p2_time,
    }
}
