use crate::Timing;
use std::time::Instant;

fn get_range_iter(start: usize, end: usize) -> impl Iterator<Item = usize> {
    if start < end {
        start..=end
    } else {
        end..=start
    }
}

fn parse(input: &str) -> ([[u8; 675]; 185], usize) {
    // 0 == empty
    // 1 == wall
    // 2 == sand
    let mut grid = [[0u8; 675]; 185];
    let mut max_y = 0;
    for line in input.lines() {
        let points: Vec<(usize, usize)> = line
            .split(" -> ")
            .map(|p| {
                let (x, y) = p.split_once(',').unwrap();
                (x.parse::<usize>().unwrap(), y.parse::<usize>().unwrap())
            })
            .collect();
        for w in points.windows(2) {
            let (p1_x, p1_y) = w[0];
            let (p2_x, p2_y) = w[1];
            get_range_iter(p1_x, p2_x).for_each(|x| {
                get_range_iter(p1_y, p2_y).for_each(|y| {
                    max_y = max_y.max(y);
                    grid[y][x] = 1u8;
                });
            });
        }
    }
    (grid, max_y + 2)
}

fn part1(data: &([[u8; 675]; 185], usize)) -> usize {
    let (inp, _) = data;
    both(&mut inp.clone(), usize::MAX)
}

fn part2(data: &([[u8; 675]; 185], usize)) -> usize {
    let (mut inp, max_y) = data;
    both(&mut inp, *max_y)
}

fn both(inp: &mut [[u8; 675]; 185], max_y: usize) -> usize {
    let mut res = 0;
    loop {
        let mut sand_x = 500;
        let mut sand_y = 0;
        loop {
            if sand_y + 1 >= max_y {
                // Sand hit the floor (part 2)
                inp[sand_y][sand_x] = 2u8;
                res += 1;
                break;
            } else if sand_y >= inp.len() - 1 {
                // Sand fell into the abyss (part 1)
                return res;
            } else if inp[sand_y + 1][sand_x] == 0u8 {
                sand_y += 1;
            } else if inp[sand_y + 1][sand_x - 1] == 0u8 {
                sand_y += 1;
                sand_x -= 1;
            } else if inp[sand_y + 1][sand_x + 1] == 0u8 {
                sand_y += 1;
                sand_x += 1;
            } else if sand_y == 0 {
                // The last possible sand position
                return res + 1;
            } else {
                inp[sand_y][sand_x] = 2u8;
                res += 1;
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
    let p1 = part1(&inp);
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
