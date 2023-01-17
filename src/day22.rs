use crate::{Point2D, Timing};
use grid::Grid;
use std::time::Instant;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum Direction {
    Right,
    Down,
    Left,
    Up,
}

impl Direction {
    fn right(&self) -> Self {
        match *self {
            Direction::Right => Direction::Down,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
            Direction::Up => Direction::Right,
        }
    }

    fn left(&self) -> Self {
        match *self {
            Direction::Right => Direction::Up,
            Direction::Down => Direction::Right,
            Direction::Left => Direction::Down,
            Direction::Up => Direction::Left,
        }
    }

    fn get_move(&self) -> Point2D<i32> {
        match *self {
            Direction::Right => (1, 0),
            Direction::Down => (0, 1),
            Direction::Left => (-1, 0),
            Direction::Up => (0, -1),
        }
    }
}

#[derive(Debug)]
enum Move {
    Walk(u8),
    TurnLeft,
    TurnRight,
}

fn parse(input: &str) -> (Grid<char>, Vec<Move>) {
    let (maze, inst) = input.split_once("\n\n").unwrap();

    let (mut h, mut w) = (0, 0);
    for line in maze.lines() {
        h += 1;
        w = w.max(line.len());
    }

    let mut grid = Grid::<char>::init(h, w, ' ');
    for (idy, line) in maze.lines().enumerate() {
        for (idx, c) in line.chars().enumerate() {
            grid[idy][idx] = c;
        }
    }

    let inst_arr = inst.as_bytes();
    let mut inst_pos = 0usize;

    let mut moves: Vec<Move> = vec![];

    while inst_pos < inst_arr.len() {
        let mut num = 0;
        while inst_pos < inst.len() && inst_arr[inst_pos].is_ascii_digit() {
            num *= 10;
            num += inst_arr[inst_pos] - b'0';
            inst_pos += 1;
        }
        moves.push(Move::Walk(num));
        if inst_pos >= inst.len() {
            break;
        }
        match inst_arr[inst_pos] {
            b'R' => moves.push(Move::TurnRight),
            b'L' => moves.push(Move::TurnLeft),
            _ => unreachable!(),
        }
        inst_pos += 1;
    }

    (grid, moves)
}

fn find_combination(
    grid: &Grid<char>,
    moves: &Vec<Move>,
    teleport: Box<dyn Fn(&Grid<char>, i32, Direction, Point2D<i32>, Point2D<i32>) -> (Point2D<i32>, Direction)>,
) -> usize {
    let (height, width) = grid.size();

    let (mut y_pos, mut x_pos) = (0usize, grid[0].iter().position(|c| *c == '.').unwrap());
    let mut facing = Direction::Right;

    for mve in moves {
        match mve {
            Move::Walk(steps) => {
                for _ in 0..*steps {
                    let (x_off, y_off) = facing.get_move();
                    let (mut x_new, mut y_new) = (x_pos as i32 + x_off, y_pos as i32 + y_off);
                    let mut facing_new = facing;

                    if !(0..width).contains(&(x_new as usize))
                        || !(0..height).contains(&(y_new as usize))
                        || !".#".contains(grid[y_new as usize][x_new as usize])
                    {
                        ((x_new, y_new), facing_new) = teleport(
                            grid,
                            get_face(x_pos, y_pos),
                            facing,
                            (x_new, y_new),
                            (x_pos as i32, y_pos as i32),
                        );
                    }
                    if grid[y_new as usize][x_new as usize] == '#' {
                        break;
                    }
                    x_pos = x_new as usize;
                    y_pos = y_new as usize;
                    facing = facing_new;
                }
            }
            Move::TurnLeft => facing = facing.left(),
            Move::TurnRight => facing = facing.right(),
        }
    }
    1000 * (y_pos + 1) + 4 * (x_pos + 1) + facing as usize
}

fn part1(inp: &(Grid<char>, Vec<Move>)) -> usize {
    let (grid, moves) = inp;
    find_combination(
        grid,
        moves,
        Box::new(|maze, _, facing, (mut x, mut y), (_, _)| {
            let (h, w) = maze.size();
            match facing {
                Direction::Right => x = 0,
                Direction::Down => y = 0,
                Direction::Left => x = w as i32 - 1,
                Direction::Up => y = h as i32 - 1,
            }
            while maze[y as usize][x as usize] == ' ' {
                x += facing.get_move().0;
                y += facing.get_move().1;
            }
            ((x, y), facing)
        }),
    )
}

fn part2(inp: &(Grid<char>, Vec<Move>)) -> usize {
    let (grid, moves) = inp;
    find_combination(
        grid,
        moves,
        Box::new(|_, face, facing, (_, _), (x, y)| match face {
            1 => match facing {
                Direction::Left => ((0, 149 - y), Direction::Right),
                Direction::Up => ((0, 100 + x), Direction::Right),
                _ => panic!(),
            },
            2 => match facing {
                Direction::Up => ((x - 100, 199), Direction::Up),
                Direction::Right => ((99, 149 - y), Direction::Left),
                Direction::Down => ((99, x - 50), Direction::Left),
                _ => panic!(),
            },
            3 => match facing {
                Direction::Right => ((50 + y, 49), Direction::Up),
                Direction::Left => ((y - 50, 100), Direction::Down),
                _ => panic!(),
            },
            4 => match facing {
                Direction::Left => ((50, 149 - y), Direction::Right),
                Direction::Up => ((50, 50 + x), Direction::Right),
                _ => panic!(),
            },
            5 => match facing {
                Direction::Right => ((149, 149 - y), Direction::Left),
                Direction::Down => ((49, 100 + x), Direction::Left),
                _ => panic!(),
            },
            6 => match facing {
                Direction::Right => ((y - 100, 149), Direction::Up),
                Direction::Down => ((x + 100, 0), Direction::Down),
                Direction::Left => ((y - 100, 0), Direction::Down),
                _ => panic!(),
            },
            _ => panic!(),
        }),
    )
}

fn get_face(x: usize, y: usize) -> i32 {
    /*
      x12
      x3x
      45x
      6xx
    */
    match y / 50 {
        0 => match x / 50 {
            1 => 1,
            2 => 2,
            _ => panic!(),
        },
        1 => 3,
        2 => match x / 50 {
            0 => 4,
            1 => 5,
            _ => panic!(),
        },
        3 => 6,
        _ => panic!(),
    }
}

pub fn solve(output: bool) -> Timing {
    let raw_input = include_str!("../aoc_input/2022/day22.txt");
    let start = Instant::now();
    let inp = parse(raw_input);
    let parse_time = start.elapsed();
    let p1 = part1(&inp);
    let p1_time = start.elapsed() - parse_time;
    let p2 = part2(&inp);
    let p2_time = start.elapsed() - p1_time;
    if output {
        println!("Day 22");
        println!("\tPart 1: {}", p1);
        println!("\tPart 2: {}", p2);
    }

    Timing {
        day: 22,
        parse: parse_time,
        p1: p1_time,
        p2: p2_time,
    }
}
