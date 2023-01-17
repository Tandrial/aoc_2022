use std::{fmt::Display, ops::Add, time::Duration};
extern crate pest;
#[macro_use]
extern crate pest_derive;

extern crate regex_macro;

pub mod day01;
pub mod day02;
pub mod day03;
pub mod day04;
pub mod day05;
pub mod day06;
pub mod day07;
pub mod day08;
pub mod day09;
pub mod day10;
pub mod day11;
pub mod day12;
pub mod day13;
pub mod day14;
pub mod day15;
pub mod day16;
pub mod day17;
pub mod day18;
pub mod day19;
pub mod day20;
pub mod day21;
pub mod day22;
pub mod day23;
pub mod day24;
pub mod day25;

type Point2D<T> = (T, T);
type Point3D<T> = (T, T, T);

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

pub trait BorrowTwoMut<T> {
    fn borrow_two_mut(&mut self, a: usize, b: usize) -> (&mut T, &mut T);
}

impl<T> BorrowTwoMut<T> for [T] {
    fn borrow_two_mut(&mut self, a: usize, b: usize) -> (&mut T, &mut T) {
        assert!(a != b);
        assert!(a < self.len());
        assert!(b < self.len());
        unsafe {
            let ptr = self.as_mut_ptr();
            let a_ptr = ptr.add(a);
            let b_ptr = ptr.add(b);
            (&mut *a_ptr, &mut *b_ptr)
        }
    }
}
