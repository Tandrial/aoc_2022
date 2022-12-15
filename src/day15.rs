use crate::Timing;
use regex_macro::regex;
use std::{collections::HashSet, time::Instant};

type Point = (i64, i64);

fn parse(input: &str) -> Vec<(Point, i64)> {
    // Sensor at x=2, y=18: closest beacon is at x=-2, y=15
    let re = regex!(
        r"Sensor at x=(?P<sx>-?\d+), y=(?P<sy>-?\d+): closest beacon is at x=(?P<bx>-?\d+), y=(?P<by>-?\d+)"
    );
    input
        .lines()
        .map(|line| {
            let caps = re.captures(line).unwrap();
            let s: Point = (caps["sx"].parse().unwrap(), caps["sy"].parse().unwrap());
            let b: Point = (caps["bx"].parse().unwrap(), caps["by"].parse().unwrap());
            (s, (s.0 - b.0).abs() + (s.1 - b.1).abs())
        })
        .collect()
}

fn part1(inp: &[(Point, i64)]) -> i64 {
    let mut min_x = i64::MAX;
    let mut max_x = i64::MIN;
    // We waste  2000000 - sensor.y of the initial distance going up and use the rest
    // to go left or right to get the min/max x value for each sensor on that line
    for &(s, d) in inp {
        let rem_distance = d - (2000000 - s.1).abs();
        if rem_distance > 0 {
            min_x = min_x.min(s.0 - rem_distance);
            max_x = max_x.max(s.0 + rem_distance);
        }
    }
    max_x - min_x
}

fn part2(inp: &[(Point, i64)]) -> i64 {
    /*
    // Since there is exactly ONE possible position it HAS to be d + 1 distance
    // from a sensor, otherwise if the beacon would be d + 2 away from a sensor
    // there could be more beacons d + 1 away from that sensor

    // Check by quadrant up_right, up_left, down_left, down_right
    for dir in &[(1, 1), (-1, 1), (-1, -1), (1, -1)] {
        for &(s, d) in inp {
            for dist_split in 0..d {
                // split the dist into an x and y part so the points end up just
                // outside of the range of the sensor
                let ring_x = s.0 + dir.0 * dist_split;
                let ring_y = s.1 + dir.1 * (d + 1 - dist_split);
                // The beacon is in Q1 so skip if the point is somewhere else
                if ring_x < 0 || ring_y < 0 || ring_x > 4000000 || ring_y > 4000000 {
                    break;
                }
                // if the point is out of reach for ALL sensors, we found the beacon
                if inp
                    .iter()
                    .all(|&(s, d)| (s.0 - ring_x).abs() + (s.1 - ring_y).abs() >= d)
                {
                    return ring_x as i64 * 4000000 + ring_y as i64;
                }
            }
        }
    } */

    // Building on the above, there are actually just a couple of actually interesting
    // points. We only care about if boundaries intersect at the edges and not the whole line

    // The boundaries of the scanners are formed by 4 lines and all have the same slope
    // 2 with slope 1 and 2 with slope -1. All that differs is the offset of the y axis
    // The missing beacon HAS to be just outside of those line, by either +1 or -1

    // ups:   f(x) =  x + (s_x - s_y +- (d + 1)) =  x + a
    // downs: g(x) = -x + (x_y - s_y +- (d + 1)) = -x + b

    let mut ups = HashSet::<i64>::new();
    let mut downs = HashSet::<i64>::new();

    for &(s, d) in inp {
        ups.insert(s.1 - s.0 + d + 1);
        ups.insert(s.1 - s.0 - d - 1);
        downs.insert(s.0 + s.1 + d + 1);
        downs.insert(s.0 + s.1 - d - 1);
    }

    // The beacon has to be at an intersection of an up and a down line so we just
    // have to check these points and check if they are fare enough way from each sensor

    // The intersection of 2 lines f(x) = x + a and g(x) = -x + b is at ((b - a) / 2, (b + a) / 2)

    for up in ups.iter() {
        for &down in downs.iter() {
            let point = ((down - up) / 2, (down + up) / 2);
            if point.0 < 0 || point.1 < 0 || point.0 > 4000000 || point.1 > 4000000 {
                continue;
            }
            if inp
                .iter()
                .all(|&(s, d)| (s.0 - point.0).abs() + (s.1 - point.1).abs() >= d)
            {
                return point.0 as i64 * 4000000 + point.1 as i64;
            }
        }
    }
    unreachable!()
}

pub fn solve(output: bool) -> Timing {
    let raw_input = include_str!("../input/day15.txt");
    let start = Instant::now();
    let inp = parse(raw_input);
    let parse_time = start.elapsed();
    let p1 = part1(&inp);
    let p1_time = start.elapsed() - parse_time;
    let p2 = part2(&inp);
    let p2_time = start.elapsed() - p1_time;
    if output {
        println!("Day 15");
        println!("\tPart 1: {}", p1);
        println!("\tPart 2: {}", p2);
    }
    Timing {
        day: 15,
        parse: parse_time,
        p1: p1_time,
        p2: p2_time,
    }
}
