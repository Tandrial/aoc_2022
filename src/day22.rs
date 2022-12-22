use crate::Timing;
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

    fn get_move(&self) -> (i32, i32) {
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

    let mut moves: Vec<Move> = vec![];
    let mut move_pos = 0usize;

    while move_pos < inst_arr.len() {
        let mut num = 0;
        while move_pos < inst.len() && inst_arr[move_pos].is_ascii_digit() {
            num *= 10;
            num += inst_arr[move_pos] - b'0';
            move_pos += 1;
        }
        moves.push(Move::Walk(num));
        if move_pos >= inst.len() {
            break;
        }
        match inst_arr[move_pos] {
            b'R' => moves.push(Move::TurnRight),
            b'L' => moves.push(Move::TurnLeft),
            _ => unreachable!(),
        }
        move_pos += 1;
    }

    (grid, moves)
}

fn part1(inp: &(Grid<char>, Vec<Move>)) -> usize {
    let (maze, moves) = inp;
    let (height, width) = maze.size();

    let (mut y_pos, mut x_pos) = (0usize, maze[0].iter().position(|c| *c == '.').unwrap());
    let mut facing = Direction::Right;

    for mve in moves {
        match mve {
            Move::Walk(steps) => {
                for _ in 0..*steps {
                    let (x_off, y_off) = facing.get_move();
                    let (mut x_new, mut y_new) = (x_pos as i32 + x_off, y_pos as i32 + y_off);

                    if !(0..width).contains(&(x_new as usize))
                        || !(0..height).contains(&(y_new as usize))
                        || !".#".contains(maze[y_new as usize][x_new as usize])
                    {
                        match facing {
                            Direction::Right => x_new = 0,
                            Direction::Down => y_new = 0,
                            Direction::Left => x_new = width as i32 - 1,
                            Direction::Up => y_new = height as i32 - 1,
                        }
                        while maze[y_new as usize][x_new as usize] == ' ' {
                            x_new += x_off;
                            y_new += y_off;
                        }
                    }
                    if maze[y_new as usize][x_new as usize] == '#' {
                        break;
                    }
                    x_pos = x_new as usize;
                    y_pos = y_new as usize;
                }
            }
            Move::TurnLeft => facing = facing.left(),
            Move::TurnRight => facing = facing.right(),
        }
    }
    1000 * (y_pos + 1) + 4 * (x_pos + 1) + facing as usize
}

fn part2(inp: &(Grid<char>, Vec<Move>)) -> usize {
    let (maze, moves) = inp;
    let (height, width) = maze.size();

    let (mut y_pos, mut x_pos) = (0usize, maze[0].iter().position(|c| *c == '.').unwrap());
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
                        || !".#".contains(maze[y_new as usize][x_new as usize])
                    {
                        match get_face(x_pos, y_pos) {
                            1 => match facing {
                                Direction::Left => {
                                    // End up at face 4
                                    x_new = 0;
                                    y_new = 149 - y_pos as i32;
                                    facing_new = Direction::Right;
                                }
                                Direction::Up => {
                                    // End up at face 6
                                    x_new = 0;
                                    y_new = 100 + x_pos as i32;
                                    facing_new = Direction::Right;
                                }
                                _ => panic!(),
                            },
                            2 => match facing {
                                Direction::Up => {
                                    // End up at face 6
                                    x_new = x_pos as i32 - 100;
                                    y_new = 199;
                                    facing_new = Direction::Up;
                                }
                                Direction::Right => {
                                    // End up at face 5
                                    x_new = 99;
                                    y_new = 149 - y_pos as i32;
                                    facing_new = Direction::Left;
                                }
                                Direction::Down => {
                                    // End up at face 3
                                    x_new = 99;
                                    y_new = x_pos as i32 - 50;
                                    facing_new = Direction::Left;
                                }
                                _ => panic!(),
                            },
                            3 => match facing {
                                Direction::Right => {
                                    // Ends up at face 2
                                    y_new = 49;
                                    x_new = 50 + y_pos as i32;
                                    facing_new = Direction::Up;
                                }
                                Direction::Left => {
                                    // End up at face 4
                                    x_new = y_pos as i32 - 50;
                                    y_new = 100;
                                    facing_new = Direction::Down;
                                }
                                _ => panic!(),
                            },
                            4 => match facing {
                                Direction::Left => {
                                    // End up at face 1
                                    x_new = 50;
                                    y_new = 149 - y_pos as i32;
                                    facing_new = Direction::Right;
                                }
                                Direction::Up => {
                                    // End up at face 3
                                    x_new = 50;
                                    y_new = 50 + x_pos as i32;
                                    facing_new = Direction::Right;
                                }
                                _ => panic!(),
                            },
                            5 => match facing {
                                Direction::Right => {
                                    // End up at face 2
                                    x_new = 149;
                                    y_new = 149 - y_pos as i32;
                                    facing_new = Direction::Left;
                                }
                                Direction::Down => {
                                    // End up at face 6
                                    x_new = 49;
                                    y_new = 100 + x_pos as i32;
                                    facing_new = Direction::Left;
                                }
                                _ => panic!(),
                            },
                            6 => match facing {
                                Direction::Right => {
                                    // End up at face 5
                                    y_new = 149;
                                    x_new = y_pos as i32 - 100;
                                    facing_new = Direction::Up;
                                }
                                Direction::Down => {
                                    // End up at face 2
                                    y_new = 0;
                                    x_new = x_pos as i32 + 100;
                                    facing_new = Direction::Down;
                                }
                                Direction::Left => {
                                    // End up at face 1
                                    x_new = y_pos as i32 - 100;
                                    y_new = 0;
                                    facing_new = Direction::Down;
                                }
                                _ => panic!(),
                            },
                            _ => panic!(),
                        }
                    }
                    if maze[y_new as usize][x_new as usize] == '#' {
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
    let raw_input = include_str!("../input/day22.txt");
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
