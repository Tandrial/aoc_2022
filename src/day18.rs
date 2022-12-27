use crate::{Point3D, Timing};
use hashbrown::HashSet;
use std::{collections::VecDeque, time::Instant};

fn parse(input: &str) -> HashSet<Point3D<i32>> {
    let mut res = HashSet::<Point3D<i32>>::new();
    for line in input.lines() {
        let coords: Vec<i32> = line.split(',').map(|num| num.parse().unwrap()).collect();
        res.insert((coords[0], coords[1], coords[2]));
    }
    res
}

fn part1(inp: &HashSet<Point3D<i32>>) -> i64 {
    let mut result = 0;
    for (x, y, z) in inp.iter() {
        let mut visible_surface: i64 = 6;
        for (dx, dy, dz) in &[
            (1, 0, 0),
            (-1, 0, 0),
            (0, 1, 0),
            (0, -1, 0),
            (0, 0, 1),
            (0, 0, -1),
        ] {
            let next = (x + dx, y + dy, z + dz);
            if inp.contains(&next) {
                visible_surface -= 1;
            }
        }
        result += visible_surface;
    }
    result
}

fn part2(inp: &HashSet<Point3D<i32>>) -> i64 {
    let x_min = inp.iter().map(|(x, _, _)| x).min().unwrap() - 1;
    let y_min = inp.iter().map(|(_, y, _)| y).min().unwrap() - 1;
    let z_min = inp.iter().map(|(_, _, z)| z).min().unwrap() - 1;

    let x_max = inp.iter().map(|(x, _, _)| x).max().unwrap() + 1;
    let y_max = inp.iter().map(|(_, y, _)| y).max().unwrap() + 1;
    let z_max = inp.iter().map(|(_, _, z)| z).max().unwrap() + 1;

    let mut q = VecDeque::new();
    let mut seen = HashSet::<Point3D<i32>>::new();
    let start = (x_min, y_min, z_min);
    q.push_back(start);
    seen.insert(start);
    let mut total_surfaces = 0;

    while let Some((x, y, z)) = q.pop_front() {
        for (dx, dy, dz) in &[
            (1, 0, 0),
            (-1, 0, 0),
            (0, 1, 0),
            (0, -1, 0),
            (0, 0, 1),
            (0, 0, -1),
        ] {
            let next = (x + dx, y + dy, z + dz);
            if (x_min..=x_max).contains(&next.0)
                && (y_min..=y_max).contains(&next.1)
                && (z_min..=z_max).contains(&next.2)
            {
                if inp.contains(&next) {
                    total_surfaces += 1
                } else if !seen.contains(&next) {
                    seen.insert(next);
                    q.push_back(next);
                }
            }
        }
    }
    total_surfaces
}

pub fn solve(output: bool) -> Timing {
    let raw_input = include_str!("../input/day18.txt");
    let start = Instant::now();
    let inp = parse(raw_input);
    let parse_time = start.elapsed();
    let p1 = part1(&inp);
    let p1_time = start.elapsed() - parse_time;
    let p2 = part2(&inp);
    let p2_time = start.elapsed() - p1_time;
    if output {
        println!("Day 18");
        println!("\tPart 1: {}", p1);
        println!("\tPart 2: {}", p2);
    }

    Timing {
        day: 18,
        parse: parse_time,
        p1: p1_time,
        p2: p2_time,
    }
}
