use crate::Timing;

use std::time::Instant;

fn draw_path(grid: &mut [[u8; 675]; 178], path: &str) -> usize {
    let points: Vec<(usize, usize)> = path
        .split(" -> ")
        .map(|p| {
            let (x, y) = p.split_once(',').unwrap();
            (x.parse::<usize>().unwrap(), y.parse::<usize>().unwrap())
        })
        .collect();
    let mut max_y = 0;
    for w in points.windows(2) {
        let (p1_x, p1_y) = w[0];
        let (p2_x, p2_y) = w[1];
        let x_start = p1_x.min(p2_x);
        let y_start = p1_y.min(p2_y);
        let x_end = p1_x.max(p2_x);
        let y_end = p1_y.max(p2_y);

        (x_start..=x_end).for_each(|x| {
            (y_start..=y_end).for_each(|y| {
                max_y = max_y.max(y);
                grid[y][x] = 1u8;
            });
        });
    }
    max_y
}

fn parse(input: &str) -> ([[u8; 675]; 178], usize) {
    // 0 == empty
    // 1 == wall
    // 2 == sand
    let mut grid = [[0u8; 675]; 178];
    let mut max_y = 0;
    for line in input.lines() {
        max_y = max_y.max(draw_path(&mut grid, line));
    }

    (grid, max_y + 2)
}

fn part1(mut inp: [[u8; 675]; 178]) -> usize {
    let mut p1 = 0;
    loop {
        let mut sand_x = 500;
        let mut sand_y = 0;
        loop {
            if sand_y >= inp.len() - 1 {
                return p1;
            } else if inp[sand_y + 1][sand_x] == 0u8 {
                sand_y += 1;
            } else if inp[sand_y + 1][sand_x - 1] == 0u8 {
                sand_y += 1;
                sand_x -= 1;
            } else if inp[sand_y + 1][sand_x + 1] == 0u8 {
                sand_y += 1;
                sand_x += 1;
            } else {
                inp[sand_y][sand_x] = 2u8;
                p1 += 1;
                break;
            }
        }
    }
}

fn part2(data: &([[u8; 675]; 178], usize)) -> usize {
    let (mut inp, max_y) = data;
    let mut p2 = 0;
    loop {
        let mut sand_x = 500;
        let mut sand_y = 0;
        loop {
            if sand_y + 1 >= *max_y {
                inp[sand_y][sand_x] = 2u8;
                p2 += 1;
                break;
            } else if inp[sand_y + 1][sand_x] == 0u8 {
                sand_y += 1;
            } else if inp[sand_y + 1][sand_x - 1] == 0u8 {
                sand_y += 1;
                sand_x -= 1;
            } else if inp[sand_y + 1][sand_x + 1] == 0u8 {
                sand_y += 1;
                sand_x += 1;
            } else if sand_y == 0 {
                return p2 + 1;
            } else {
                inp[sand_y][sand_x] = 2u8;
                p2 += 1;
                break;
            }
        }
    }
}

pub fn solve(output: bool) -> Timing {
    let raw_input = include_str!("../input/day14.txt");
    let start = Instant::now();
    let inp = parse(raw_input);
    let parse_time = start.elapsed();
    let p1 = part1(inp.0);
    let p1_time = start.elapsed() - parse_time;
    let p2 = part2(&inp);
    let p2_time = start.elapsed() - p1_time;
    if output {
        println!("Day 14");
        println!("\tPart 1: {}", p1);
        println!("\tPart 2: {}", p2);
    }
    Timing {
        day: 14,
        parse: parse_time,
        p1: p1_time,
        p2: p2_time,
    }
}
