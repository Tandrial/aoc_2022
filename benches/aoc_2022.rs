use criterion::{criterion_group, criterion_main, Criterion};

fn bench_all() {
    aoc_2022::day01::solve(false);
    aoc_2022::day02::solve(false);
    aoc_2022::day03::solve(false);
    aoc_2022::day04::solve(false);
    aoc_2022::day05::solve(false);
    aoc_2022::day06::solve(false);
}

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("AoC 2022 - Day01", |b| {
        b.iter(|| aoc_2022::day01::solve(false))
    });
    c.bench_function("AoC 2022 - Day02", |b| {
        b.iter(|| aoc_2022::day02::solve(false))
    });
    c.bench_function("AoC 2022 - Day03", |b| {
        b.iter(|| aoc_2022::day03::solve(false))
    });
    c.bench_function("AoC 2022 - Day04", |b| {
        b.iter(|| aoc_2022::day04::solve(false))
    });
    c.bench_function("AoC 2022 - Day05", |b| {
        b.iter(|| aoc_2022::day05::solve(false))
    });
    c.bench_function("AoC 2022 - Day06", |b| {
        b.iter(|| aoc_2022::day06::solve(false))
    });
    c.bench_function("AoC 2022 - All", |b| b.iter(bench_all));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);