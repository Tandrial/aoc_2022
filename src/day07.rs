use crate::Timing;
use relative_path::{RelativePath as Path, RelativePathBuf as PathBuf};
use std::{collections::HashMap, time::Instant};

fn parse(input: &str) -> HashMap<PathBuf, u64> {
    let mut path = PathBuf::new();

    let mut folders = HashMap::<_, u64>::new();
    let mut total_size = 0;

    for line in input.lines() {
        match line.chars().next() {
            Some('$') => {
                let (_, command) = line.split_once(' ').unwrap();
                if command.starts_with("cd") {
                    let (_, arg) = command.split_once(' ').unwrap();
                    if arg != "/" {
                        path = path.join_normalized(Path::new(arg));
                    }
                }
            }
            Some('d') => {} // nothing to do on dir
            Some(_) => {
                let (num, _) = line.split_once(' ').unwrap();
                let size = num.parse::<u64>().unwrap();
                total_size += size;

                let mut curr_folder = Some(path.clone());

                while let Some(folder) = curr_folder.take() {
                    *folders.entry(folder.clone()).or_default() += size;
                    // Move one directory up and take over ownership of the object
                    curr_folder = folder.parent().map(|p| p.to_owned());
                }
            }
            None => unreachable!(),
        }
    }
    folders.insert(PathBuf::from("total"), total_size);
    folders
}

fn part1(inp: &HashMap<PathBuf, u64>) -> u64 {
    inp.values().filter(|&size| *size < 100_000).sum::<u64>()
}

fn part2(inp: &HashMap<PathBuf, u64>) -> u64 {
    let free_space = 70_000_000 - inp.get(&PathBuf::from("total")).unwrap();
    let needed_space = 30_000_000 - free_space;
    *inp.values()
        .filter(|&size| *size >= needed_space)
        .min()
        .unwrap()
}

pub fn solve(output: bool) -> Timing {
    let raw_input = include_str!("../input/day07.txt");
    let start = Instant::now();
    let inp = parse(raw_input);
    let parse_time = start.elapsed();
    let p1 = part1(&inp);
    let p1_time = start.elapsed() - parse_time;
    let p2 = part2(&inp);
    let p2_time = start.elapsed() - p1_time;

    if output {
        println!("Day 07");
        println!("\tPart 1: {}", p1);
        println!("\tPart 2: {}", p2);
    }
    Timing {
        day: 7,
        parse: parse_time,
        p1: p1_time,
        p2: p2_time,
    }
}
