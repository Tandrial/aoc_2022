use std::{fmt::Display, ops::Add, time::Duration};

mod day01;
mod day02;
mod day03;
mod day04;
mod day05;

pub struct Timing {
    day: u32,
    parse: Duration,
    p1: Duration,
    p2: Duration,
}

impl Display for Timing {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            " | {:02}  | {:>10} | {:>10} | {:>10} | {:>10} |",
            self.day,
            dur_string(&self.parse),
            dur_string(&self.p1),
            dur_string(&self.p2),
            dur_string(&self.parse.add(self.p1).add(self.p2))
        )
    }
}

fn dur_string(d: &Duration) -> String {
    if d.as_secs() != 0 {
        format!("{:.3} s", d.as_secs_f32())
    } else if d.as_millis() != 0 {
        format!("{:.3} ms", d.as_micros() as f32 / 1000.0)
    } else if d.as_micros() != 0 {
        format!("{:.3} us", d.as_nanos() as f32 / 1000.0)
    } else {
        format!("{:.3} ns", d.as_nanos())
    }
}

pub fn print_stats(stats: &[Timing]) {
    let mut total_parse = Duration::new(0, 0);
    let mut total_p1 = Duration::new(0, 0);
    let mut total_p2 = Duration::new(0, 0);
    println!("\n");
    println!(" | Day |      Parse |      Part1 |      Part2 |      Total |");
    println!("-+-----+------------+------------+------------+------------+-");
    for stat in stats {
        total_parse += stat.parse;
        total_p1 += stat.p1;
        total_p2 += stat.p2;
        println!("{}", stat);
    }
    println!("-+-----+------------+------------+------------+------------+-");
    let line = format!(
        " | sum | {:>10} | {:>10} | {:>10} | {:>10} |",
        dur_string(&total_parse),
        dur_string(&total_p1),
        dur_string(&total_p2),
        dur_string(&total_parse.add(total_p1).add(total_p2))
    );
    println!("{}", line);
}

fn main() {
    let stats = vec![
        day01::solve(),
        day02::solve(),
        day03::solve(),
        day04::solve(),
        day05::solve(),
    ];

    print_stats(&stats);
}
