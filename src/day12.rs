use crate::Timing;
use std::{
    collections::{HashSet, VecDeque},
    time::Instant,
};

type Grid = Vec<Vec<u8>>;
type Point = (i32, i32);

fn parse(input: &str) -> (Grid, Point, Point) {
    let height = &input.lines().count();
    let mut start = (0, 0);
    let mut end = (0, 0);
    let mut grid = std::iter::repeat(vec![]).take(*height).collect::<Vec<_>>();
    for (idy, line) in input.lines().enumerate() {
        let mut idx: usize = 0;
        for &num in line.as_bytes() {
            if num == b'S' {
                start = (idx as i32, idy as i32);
                grid.get_mut(idy).unwrap().push(b'a');
            } else if num == b'E' {
                end = (idx as i32, idy as i32);
                grid.get_mut(idy).unwrap().push(b'z');
            } else {
                grid.get_mut(idy).unwrap().push(num);
            }
            idx += 1;
        }
    }
    (grid, start, end)
}

fn both(inp: (Grid, Point, Point)) -> (usize, usize) {
    let (grid, start, end) = inp;
    let mut p1 = 0;
    let mut p2 = 0;
    let mut q = VecDeque::new();
    let mut seen = HashSet::<(i32, i32)>::new();
    q.push_back((start, "".to_string()));
    seen.insert(start);

    while let Some(((next_x, next_y), path)) = q.pop_front() {
        if (next_x, next_y) == end {
            p1 = path.len();
            p2 = p1 - path.chars().take_while(|c| *c == 'a').count();
            break;
        }
        let height = grid
            .get(next_y as usize)
            .and_then(|line| line.get(next_x as usize))
            .unwrap();
        for (dx, dy) in &[(0, -1), (0, 1), (1, 0), (-1, 0)] {
            let (child_x, child_y) = (next_x + dx, next_y + dy);
            if let Some(c_height) = grid
                .get(child_y as usize)
                .and_then(|c| c.get(child_x as usize))
            {
                let diff = *c_height as i32 - *height as i32;
                if diff <= 1 && !seen.contains(&(child_x, child_y)) {
                    seen.insert((child_x, child_y));
                    let mut path_new = path.clone();
                    path_new.push(*c_height as char);
                    q.push_back(((child_x, child_y), path_new));
                }
            }
        }
    }
    (p1, p2)
}

pub fn solve(output: bool) -> Timing {
    let raw_input = include_str!("../input/day12.txt");
    let start = Instant::now();
    let inp = parse(raw_input);
    let parse_time = start.elapsed();
    let p1_time = start.elapsed() - parse_time;
    let (p1, p2) = both(inp);
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
