use crate::{Point2D, Timing};
use hashbrown::{HashMap, HashSet};
use std::time::Instant;

type HeightHash = [i64; 7];

#[derive(Eq, Hash, PartialEq, Debug)]
struct State {
    gust_index: usize,
    rock_index: usize,
    height_hash: HeightHash,
}

#[derive(Clone, Copy, Debug)]
enum Gust {
    Left = -1,
    Right = 1,
}

fn parse(input: &str) -> Vec<Gust> {
    input
        .chars()
        .map(|c| match c {
            '<' => Gust::Left,
            '>' => Gust::Right,
            _ => panic!(),
        })
        .collect()
}

#[derive(Copy, Clone, Debug)]
enum RockShape {
    HBar,
    Plus,
    L,
    VBar,
    Square,
}

impl RockShape {
    fn cells(self) -> impl Iterator<Item = Point2D<i64>> {
        match self {
            RockShape::HBar => [(0, 0), (1, 0), (2, 0), (3, 0)].iter().copied(),
            RockShape::Plus => [(1, 0), (0, 1), (1, 1), (2, 1), (1, 2)].iter().copied(),
            RockShape::L => [(0, 0), (1, 0), (2, 0), (2, 1), (2, 2)].iter().copied(),
            RockShape::VBar => [(0, 0), (0, 1), (0, 2), (0, 3)].iter().copied(),
            RockShape::Square => [(0, 0), (1, 0), (0, 1), (1, 1)].iter().copied(),
        }
    }
}

#[derive(Default, Clone)]
struct Cavern {
    columns: [HashSet<i64>; 7],
    max_height: i64,
}

impl Cavern {
    fn calc_hash(&self) -> HeightHash {
        let mut result = [self.max_height; 7];

        for (idx, column) in self.columns.iter().enumerate() {
            result[idx] -= *column.iter().max().unwrap_or(&-1);
        }
        result
    }

    fn hit_something(&self, coord: Point2D<i64>) -> bool {
        // The rock_part could hit
        // a wall
        !(0..=6).contains(&coord.0)
        // the floor
            || coord.1 <= 0
        // another rock_part
            || self.columns[coord.0 as usize].contains(&coord.1)
    }

    fn rock_can_move(&self, shape: RockShape, coord: Point2D<i64>) -> bool {
        // The rock can move if no part of it hit anything
        !shape
            .cells()
            .any(|rock| self.hit_something((coord.0 + rock.0, coord.1 + rock.1)))
    }

    fn drop_rock<'a>(
        &mut self,
        gusts: &mut impl Iterator<Item = (usize, &'a Gust)>,
        rock: RockShape,
    ) {
        let mut rock_coord = (2i64, self.max_height + 4);

        loop {
            let gust = gusts.next().unwrap().1;

            // apply gust
            let rock_after_gust = (rock_coord.0 + *gust as i64, rock_coord.1);
            if self.rock_can_move(rock, rock_after_gust) {
                rock_coord = rock_after_gust;
            }
            // apply gravity
            let rock_after_gravity = (rock_coord.0, rock_coord.1 - 1);
            if !self.rock_can_move(rock, rock_after_gravity) {
                break;
            }
            rock_coord = rock_after_gravity;
        }
        rock.cells().for_each(|r| {
            // Insert all rock_parts into the correct columns
            self.columns
                .get_mut((rock_coord.0 + r.0) as usize)
                .unwrap()
                .insert(rock_coord.1 + r.1);
            // Update the max_height
            self.max_height = self.max_height.max(rock_coord.1 + r.1);
        });
    }
}

fn simulate(inp: &[Gust], rocks_max: usize) -> i64 {
    // We use the following triple to "hash" the State:
    // gust index, rock index, height_profile (diff between max_height and col_height for each column)
    let mut seen = HashMap::<State, (i64, usize)>::new();

    let rock_order = [
        RockShape::HBar,
        RockShape::Plus,
        RockShape::L,
        RockShape::VBar,
        RockShape::Square,
    ];

    let mut gusts = inp.iter().enumerate().cycle().peekable();
    let mut rocks = rock_order.iter().enumerate().cycle().peekable();
    let mut cavern = Cavern::default();

    let mut rock_index = 0;
    while rock_index < rocks_max {
        rock_index += 1;

        cavern.drop_rock(&mut gusts, *rocks.next().unwrap().1);

        // Store the state so we can look for cycles
        if let Some((cycle_max_h, cycle_r_idx)) = seen.insert(
            State {
                gust_index: gusts.peek().unwrap().0,
                rock_index: rock_index % rock_order.len(),
                height_hash: cavern.calc_hash(),
            },
            (cavern.max_height, rock_index),
        ) {
            // If we could a cycle we need to calculate how cycles we can fit into the remaining rock_index
            let length_cycle = rock_index - cycle_r_idx;
            let cycles_left = (rocks_max - rock_index) / length_cycle;

            // We know skip ahead by the amount of cycles that fit into the remaining rock_index
            rock_index += cycles_left * length_cycle;
            // The cycle adds now_height - cycle_start_height many rocks to the highest column
            let skipped_rocks = cycles_left as i64 * (cavern.max_height - cycle_max_h);

            // drop the rocks which aren't a full cycle
            for _ in rock_index..rocks_max {
                cavern.drop_rock(&mut gusts, *rocks.next().unwrap().1);
            }

            // add the height that would have been added if we wouldn't have skipped over the cycles
            return cavern.max_height + skipped_rocks;
        }
    }

    cavern.max_height
}

fn part1(input: &[Gust]) -> i64 {
    simulate(input, 2022)
}

fn part2(input: &[Gust]) -> i64 {
    simulate(input, 1000000000000)
}

pub fn solve(output: bool) -> Timing {
    let raw_input = include_str!("../aoc_input/2022/day17.txt");
    let start = Instant::now();
    let inp = parse(raw_input);
    let parse_time = start.elapsed();
    let p1 = part1(&inp);
    let p1_time = start.elapsed() - parse_time;
    let p2 = part2(&inp);
    let p2_time = start.elapsed() - p1_time;
    if output {
        println!("Day 17");
        println!("\tPart 1: {}", p1);
        println!("\tPart 2: {}", p2);
    }

    Timing {
        day: 17,
        parse: parse_time,
        p1: p1_time,
        p2: p2_time,
    }
}
