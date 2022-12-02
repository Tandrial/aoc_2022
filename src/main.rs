use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

mod day01;
mod day02;
mod day03;

fn main() {
    println!("Day 01");
    day01::solve();
    println!("Day 02");
    day02::solve();
    println!("Day 03");
    day03::solve();
}

fn get_input(day: i64) -> String {
    std::fs::read_to_string(format!("input/day{:02}.txt", day)).expect("File not found")
}

// copied from here https://doc.rust-lang.org/rust-by-example/std_misc/file/read_lines.html
#[allow(dead_code)]
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
