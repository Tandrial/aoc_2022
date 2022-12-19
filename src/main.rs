use aoc_2022::Timing;
use std::{ops::Add, time::Duration};

pub fn print_stats(stats: &[Timing]) {
    let mut total_parse = Duration::new(0, 0);
    let mut total_p1 = Duration::new(0, 0);
    let mut total_p2 = Duration::new(0, 0);
    println!("\n");
    println!("| Day |      Parse |      Part1 |      Part2 |      Total |");
    println!("| :-: | ---------: | ---------: | ---------: | ---------: |");
    for stat in stats {
        total_parse += stat.parse;
        total_p1 += stat.p1;
        total_p2 += stat.p2;
        println!("{}", stat);
    }
    let line = format!(
        "| sum | {:>10} | {:>10} | {:>10} | {:>10} |",
        aoc_2022::dur_string(&total_parse),
        aoc_2022::dur_string(&total_p1),
        aoc_2022::dur_string(&total_p2),
        aoc_2022::dur_string(&total_parse.add(total_p1).add(total_p2))
    );
    println!("{}", line);
    println!(
        "\n`{}s` remaining",
        (Duration::new(1, 0) - total_parse.add(total_p1).add(total_p2)).as_secs_f32()
    )
}

fn main() {
    let stats = vec![
        aoc_2022::day01::solve(true),
        aoc_2022::day02::solve(true),
        aoc_2022::day03::solve(true),
        aoc_2022::day04::solve(true),
        aoc_2022::day05::solve(true),
        aoc_2022::day06::solve(true),
        aoc_2022::day07::solve(true),
        aoc_2022::day08::solve(true),
        aoc_2022::day09::solve(true),
        aoc_2022::day10::solve(true),
        // aoc_2022::day11::solve(true),
        aoc_2022::day12::solve(true),
        aoc_2022::day13::solve(true),
        aoc_2022::day14::solve(true),
        aoc_2022::day15::solve(true),
        aoc_2022::day16::solve(true),
        // aoc_2022::day17::solve(true),
        aoc_2022::day18::solve(true),
        aoc_2022::day19::solve(true),
        // aoc_2022::day20::solve(true),
        // aoc_2022::day21::solve(true),
        // aoc_2022::day22::solve(true),
        // aoc_2022::day23::solve(true),
        // aoc_2022::day24::solve(true),
        // aoc_2022::day25::solve(true),
    ];
    print_stats(&stats);
}
