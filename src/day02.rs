use byteorder::{BigEndian, ReadBytesExt};
use std::io::Cursor;
use std::time::Instant;

fn parse(inp: &str) -> Vec<(char, char)> {
    let mut games: Vec<(char, char)> = Vec::new();
    let mut rdr = Cursor::new(inp.as_bytes());
    while let Ok(val) = rdr.read_u32::<BigEndian>() {
        let [a, _, c, _] = val.to_be_bytes();
        games.push((a as char, c as char));
    }
    games
}

fn part1(inp: &[(char, char)]) -> i64 {
    let mut score: i64 = 0;
    for (opp, me) in inp {
        score += *me as i64 - 'X' as i64 + 1;
        match (*opp, *me) {
            // winnings gives 6
            ('A', 'Y') => score += 6,
            ('B', 'Z') => score += 6,
            ('C', 'X') => score += 6,
            // drawing gives 3
            (o, m) if (o as u32) + 23 == m as u32 => score += 3,
            _ => {}
        }
    }
    score
}

fn part2(inp: &[(char, char)]) -> i64 {
    let mut score: i64 = 0;
    for (opp, res) in inp {
        match (*res, *opp) {
            // needs to loose
            ('X', 'A') => score += 3,
            ('X', 'B') => score += 1,
            ('X', 'C') => score += 2,

            // needs to draw
            ('Y', 'A') => score += 4,
            ('Y', 'B') => score += 5,
            ('Y', 'C') => score += 6,

            // needs to win
            ('Z', 'A') => score += 8,
            ('Z', 'B') => score += 9,
            ('Z', 'C') => score += 7,
            _ => unreachable!(),
        }
    }
    score
}

pub fn solve() {
    let raw_input = include_str!("../input/day02.txt");
    let start = Instant::now();
    let inp = parse(raw_input);
    let parse_time = start.elapsed();
    println!("Part 1: {}", part1(&inp));
    let p1_time = start.elapsed() - parse_time;
    println!("Part 2: {}", part2(&inp));
    let p2_time = start.elapsed() - p1_time;

    println!(
        "Parsing={} µs, p1={} µs, p2={} µs",
        parse_time.as_micros(),
        p1_time.as_micros(),
        p2_time.as_micros()
    );
}
