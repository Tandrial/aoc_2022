use criterion::{criterion_group, criterion_main, Criterion};

fn bench_all() {
    aoc_2022::day01::solve(false);
    aoc_2022::day02::solve(false);
    aoc_2022::day03::solve(false);
    aoc_2022::day04::solve(false);
    aoc_2022::day05::solve(false);
    aoc_2022::day06::solve(false);
    aoc_2022::day07::solve(false);
    aoc_2022::day08::solve(false);
    aoc_2022::day09::solve(false);
    aoc_2022::day10::solve(false);
    aoc_2022::day11::solve(false);
    aoc_2022::day12::solve(false);
    aoc_2022::day13::solve(false);
    aoc_2022::day14::solve(false);
    aoc_2022::day15::solve(false);
    aoc_2022::day16::solve(false);
    aoc_2022::day17::solve(false);
    aoc_2022::day18::solve(false);
    aoc_2022::day19::solve(false);
    aoc_2022::day20::solve(false);
    aoc_2022::day21::solve(false);
    aoc_2022::day22::solve(false);
    aoc_2022::day23::solve(false);
    aoc_2022::day24::solve(false);
    aoc_2022::day25::solve(false);
}

fn criterion_benchmark(c: &mut Criterion) {
    // c.bench_function("AoC 2022 - Day01", |b| {
    //     b.iter(|| aoc_2022::day01::solve(false))
    // });
    // c.bench_function("AoC 2022 - Day02", |b| {
    //     b.iter(|| aoc_2022::day02::solve(false))
    // });
    // c.bench_function("AoC 2022 - Day03", |b| {
    //     b.iter(|| aoc_2022::day03::solve(false))
    // });
    // c.bench_function("AoC 2022 - Day04", |b| {
    //     b.iter(|| aoc_2022::day04::solve(false))
    // });
    // c.bench_function("AoC 2022 - Day05", |b| {
    //     b.iter(|| aoc_2022::day05::solve(false))
    // });
    // c.bench_function("AoC 2022 - Day06", |b| {
    //     b.iter(|| aoc_2022::day06::solve(false))
    // });
    // c.bench_function("AoC 2022 - Day07", |b| {
    //     b.iter(|| aoc_2022::day07::solve(false))
    // });
    // c.bench_function("AoC 2022 - Day08", |b| {
    //     b.iter(|| aoc_2022::day08::solve(false))
    // });
    // c.bench_function("AoC 2022 - Day09", |b| {
    //     b.iter(|| aoc_2022::day09::solve(false))
    // });
    // c.bench_function("AoC 2022 - Day10", |b| {
    //     b.iter(|| aoc_2022::day10::solve(false))
    // });
    // c.bench_function("AoC 2022 - Day11", |b| {
    //     b.iter(|| aoc_2022::day10::solve(false))
    // });
    // c.bench_function("AoC 2022 - Day12", |b| {
    //     b.iter(|| aoc_2022::day12::solve(false))
    // });
    // c.bench_function("AoC 2022 - Day13", |b| {
    //     b.iter(|| aoc_2022::day13::solve(false))
    // });
    // c.bench_function("AoC 2022 - Day14", |b| {
    //     b.iter(|| aoc_2022::day14::solve(false))
    // });
    // c.bench_function("AoC 2022 - Day15", |b| {
    //     b.iter(|| aoc_2022::day15::solve(false))
    // });
    // c.bench_function("AoC 2022 - Day16", |b| {
    //     b.iter(|| aoc_2022::day16::solve(false))
    // });
    c.bench_function("AoC 2022 - Day17", |b| {
        b.iter(|| aoc_2022::day17::solve(false))
    });
    // c.bench_function("AoC 2022 - Day18", |b| {
    //     b.iter(|| aoc_2022::day18::solve(false))
    // });
    // c.bench_function("AoC 2022 - Day19", |b| {
    //     b.iter(|| aoc_2022::day19::solve(false))
    // });
    // c.bench_function("AoC 2022 - Day20", |b| {
    //     b.iter(|| aoc_2022::day20::solve(false))
    // });
    // c.bench_function("AoC 2022 - Day21", |b| {
    //     b.iter(|| aoc_2022::day21::solve(false))
    // });
    // c.bench_function("AoC 2022 - Day22", |b| {
    //     b.iter(|| aoc_2022::day22::solve(false))
    // });
    // c.bench_function("AoC 2022 - Day23", |b| {
    //     b.iter(|| aoc_2022::day23::solve(false))
    // });
    // c.bench_function("AoC 2022 - Day24", |b| {
    //     b.iter(|| aoc_2022::day24::solve(false))
    // });
    // c.bench_function("AoC 2022 - Day25", |b| {
    //     b.iter(|| aoc_2022::day25::solve(false))
    // });
    c.bench_function("AoC 2022 - All", |b| b.iter(bench_all));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
