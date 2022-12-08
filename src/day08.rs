use crate::Timing;
use std::time::Instant;

fn parse(input: &str) -> Vec<Vec<u8>> {
    let height = &input.lines().count();
    let mut grid = std::iter::repeat(vec![]).take(*height).collect::<Vec<_>>();
    for (idx, line) in input.lines().enumerate() {
        for &num in line.as_bytes() {
            grid.get_mut(idx).unwrap().push(num - b'0');
        }
    }
    grid
}

fn both(inp: &Vec<Vec<u8>>) -> (i64, u64) {
    let (h, w) = (inp.len() as i64, inp.first().unwrap().len() as i64);
    let mut p1 = 2 * h + 2 * (w - 2);
    let mut p2 = 0;

    for y in 1..h - 1 {
        for x in 1..w - 1 {
            let mut score = 1;
            let mut can_see = false;
            for (dx, dy) in &[(0, -1i64), (0, 1), (1, 0), (-1i64, 0)] {
                let mut view_distance = 0;
                let mut blocked = false;
                let (mut curr_x, mut curr_y) = (x as i64 + dx, y as i64 + dy);
                while (0..w).contains(&curr_x) && (0..h).contains(&curr_y) {
                    view_distance += 1;
                    if inp[curr_y as usize][curr_x as usize] >= inp[y as usize][x as usize] {
                        blocked |= true;
                        break;
                    }
                    curr_x += dx;
                    curr_y += dy;
                }
                score *= view_distance;
                can_see |= !blocked;
            }
            p2 = p2.max(score);
            p1 += can_see as i64;
        }
    }
    (p1, p2)
}

pub fn solve(output: bool) -> Timing {
    let raw_input = include_str!("../input/day08.txt");
    let start = Instant::now();
    let inp = parse(raw_input);
    let parse_time = start.elapsed();
    let p1_time = start.elapsed() - parse_time;
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
