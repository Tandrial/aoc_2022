use std::{fmt::Display, ops::Add, time::Duration};

pub mod day01;
pub mod day02;
pub mod day03;
pub mod day04;
pub mod day05;
pub mod day06;
pub mod day07;
pub mod day08;

pub struct Timing {
    pub day: u32,
    pub parse: Duration,
    pub p1: Duration,
    pub p2: Duration,
}
impl Display for Timing {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "| {:02}  | {:>10} | {:>10} | {:>10} | {:>10} |",
            self.day,
            dur_string(&self.parse),
            dur_string(&self.p1),
            dur_string(&self.p2),
            dur_string(&self.parse.add(self.p1).add(self.p2))
        )
    }
}

pub fn dur_string(d: &Duration) -> String {
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
