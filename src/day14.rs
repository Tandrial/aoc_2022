use crate::Timing;
use grid::Grid;
use std::time::Instant;

type Path = Vec<(usize, usize)>;

fn get_range_iter(start: usize, end: usize) -> impl Iterator<Item = usize> {
    if start < end {
        start..=end
    } else {
        end..=start
    }
}

fn parse(input: &str) -> (Grid<u8>, usize) {
    // 0 == empty,  1 == wall, 2 == sand
    let mut height = 0;
    let mut width = 0;
    // Parse the input into a Vec of Paths
    let paths: Vec<Path> = input
        .lines()
        .map(|line| {
            let points: Vec<(usize, usize)> = line
                .split(" -> ")
                .map(|p| {
                    let (x, y) = p.split_once(',').unwrap();
                    (x.parse::<usize>().unwrap(), y.parse::<usize>().unwrap())
                })
                .collect();
            points
        })
        .collect();
    // Find the size of the input
    for path in &paths {
        for w in path.windows(2) {
            let (p1_x, p1_y) = w[0];
            let (p2_x, p2_y) = w[1];
            width = width.max(p1_x.max(p2_x));
            height = height.max(p1_y.max(p2_y));
        }
    }

    // Sand falls at (500, 0) with a max height of 175 (highest + 2 [part2]) the
    // max width of the forming pyramid 350 (45° slope) one down ==> one to the
    // left/right so to save memory we can shift the left most point to (0, 175)
    // which means the drop point is at (175, 0): 500 - 175 = 325, so shift everything
    // by 500 - (height + 2)

    let shift = 500 - (height + 2);
    let mut grid: Grid<u8> = Grid::new(height + 2, width + (width - height) - shift);
    // Insert the points along the Paths into the grid
    for path in &paths {
        for w in path.windows(2) {
            let (p1_x, p1_y) = w[0];
            let (p2_x, p2_y) = w[1];
            get_range_iter(p1_x, p2_x).for_each(|x| {
                get_range_iter(p1_y, p2_y).for_each(|y| {
                    grid[y][x - shift] = 1u8;
                });
            });
        }
    }
    (grid, height)
}

// Part 1 gets a clone of the grid
// fn part1(data: &([[u8; 350]; 175], usize)) -> usize {
fn part1(data: &(Grid<u8>, usize)) -> usize {
    let (inp, max_y) = data;
    // initial sand_drop:
    // old - shift
    // 500 - (500 - (max_y + 2)
    // 500 - 500 + max_y + 2
    both(&mut inp.clone(), usize::MAX, max_y + 2)
}

// Part 2 can work with the original grid
// fn part2(data: &([[u8; 350]; 175], usize)) -> usize {
fn part2(data: &(Grid<u8>, usize)) -> usize {
    let (inp, max_y) = data;
    both(&mut inp.clone(), *max_y, max_y + 2)
}

fn both(inp: &mut Grid<u8>, max_y: usize, initial_x: usize) -> usize {
    let mut res = 0;
    loop {
        let mut sand_x = initial_x;
        let mut sand_y = 0;
        loop {
            if sand_y > max_y {
                // Sand hit the floor (part 2)
                inp[sand_y][sand_x] = 2u8;
                res += 1;
                break;
            } else if sand_y >= inp.size().0 - 1 {
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
