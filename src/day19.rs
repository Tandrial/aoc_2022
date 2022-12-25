use crate::Timing;
use hashbrown::HashSet;
use rayon::prelude::*;
use regex_macro::regex;
use std::collections::VecDeque;
use std::time::Instant;

struct Blueprint {
    ore_bot_ore_cost: i64,
    clay_bot_ore_cost: i64,
    obsidian_bot_ore_cost: i64,
    obsidian_bot_clay_cost: i64,
    geode_bot_ore_cost: i64,
    geode_bot_obsidian_cost: i64,
}

#[derive(Default, Clone, Copy, Hash, PartialEq, Eq)]
struct State {
    ore: i64,
    ore_bots: i64,
    clay: i64,
    clay_bots: i64,
    obsidian: i64,
    obsidian_bots: i64,
    geode: i64,
    geode_bots: i64,
}

impl State {
    pub fn new() -> State {
        State {
            ore: 0,
            ore_bots: 1,
            clay: 0,
            clay_bots: 0,
            obsidian: 0,
            obsidian_bots: 0,
            geode: 0,
            geode_bots: 0,
        }
    }
}

fn mine_resources(mut state: State) -> State {
    state.ore += state.ore_bots;
    state.clay += state.clay_bots;
    state.obsidian += state.obsidian_bots;
    state.geode += state.geode_bots;
    state
}

fn parse(input: &str) -> Vec<(i64, Blueprint)> {
    // Blueprint 1: Each ore robot costs 4 ore. Each clay robot costs 4 ore. Each obsidian robot costs 4 ore and 17 clay. Each geode robot costs 4 ore and 16 obsidian.

    let re = regex!(
        r"Blueprint (?P<num>\d+): Each ore robot costs (?P<oo>\d+) ore. Each clay robot costs (?P<co>\d+) ore. Each obsidian robot costs (?P<Oo>\d+) ore and (?P<Oc>\d+) clay. Each geode robot costs (?P<go>\d+) ore and (?P<gO>\d+) obsidian."
    );
    input
        .lines()
        .map(|line| {
            let caps = re.captures(line).unwrap();
            let num: i64 = caps["num"].parse().unwrap();
            let ore_ore: i64 = caps["oo"].parse().unwrap();
            let clay_ore: i64 = caps["co"].parse().unwrap();
            let obs_ore: i64 = caps["Oo"].parse().unwrap();
            let obs_clay: i64 = caps["Oc"].parse().unwrap();
            let geo_ore: i64 = caps["go"].parse().unwrap();
            let geo_obs: i64 = caps["gO"].parse().unwrap();
            (
                num,
                Blueprint {
                    ore_bot_ore_cost: ore_ore,
                    clay_bot_ore_cost: clay_ore,
                    obsidian_bot_ore_cost: obs_ore,
                    obsidian_bot_clay_cost: obs_clay,
                    geode_bot_ore_cost: geo_ore,
                    geode_bot_obsidian_cost: geo_obs,
                },
            )
        })
        .collect()
}

fn eval_blueprint(b: &Blueprint, max_time: i64) -> i64 {
    let mut q = VecDeque::<(i64, State)>::new();
    q.push_back((0, State::new()));

    let mut seen = HashSet::<State>::new();

    let max_ore_cost = *[
        b.ore_bot_ore_cost,
        b.clay_bot_ore_cost,
        b.obsidian_bot_ore_cost,
        b.geode_bot_ore_cost,
    ]
    .iter()
    .max()
    .unwrap();

    let mut max_quality = 0;
    while !q.is_empty() {
        let (cur_time, s) = q.pop_front().unwrap();
        max_quality = max_quality.max(s.geode);

        if cur_time == max_time || seen.contains(&s) {
            continue;
        }
        seen.insert(s);

        // If we can't catch up discard the branch early
        // Best case: current geodes + geodes/min + [Build a geode each min after]
        // [...] is just the euler_sum: the next geode build gets (n) geodes, then (n-1), n-2, ...
        if (s.geode
            + s.geode_bots * (max_time - cur_time)
            + (max_time - cur_time) * (max_time - cur_time + 1) / 2)
            <= max_quality
        {
            continue;
        }

        // If we can build a geode bot build one
        // There is never a reason NOT to build more geode bots
        if s.ore >= b.geode_bot_ore_cost && s.obsidian >= b.geode_bot_obsidian_cost {
            let mut new_state = mine_resources(s);
            new_state.ore -= b.geode_bot_ore_cost;
            new_state.obsidian -= b.geode_bot_obsidian_cost;
            new_state.geode_bots += 1;
            q.push_back((cur_time + 1, new_state));
            continue;
        }

        // If we can build an obsidian bot

        // AND we have less than the cost of a geode bot build one
        //       Since we can only build one bot per turn there is no reason to get more than the geode bot cost

        if s.obsidian_bots < b.geode_bot_obsidian_cost
            && s.ore >= b.obsidian_bot_ore_cost
            && s.clay >= b.obsidian_bot_clay_cost
        {
            let mut new_state = mine_resources(s);
            new_state.ore -= b.obsidian_bot_ore_cost;
            new_state.clay -= b.obsidian_bot_clay_cost;
            new_state.obsidian_bots += 1;
            q.push_back((cur_time + 1, new_state));
            continue;
        }

        let mut nothing_built = true;

        // If we can build a clay bot

        // AND we have less than the cost of a obsidian bot build one
        //       Since we can only build one bot per turn there is no reason to get more than the obsidian bot cost

        if s.clay_bots < b.obsidian_bot_clay_cost && s.ore >= b.clay_bot_ore_cost {
            let mut new_state = mine_resources(s);
            new_state.ore -= b.clay_bot_ore_cost;
            new_state.clay_bots += 1;
            q.push_back((cur_time + 1, new_state));
            nothing_built = false;
        }

        // If we can build an ore bot AND we have less than the max cost off all bots build one
        // Since we can only build one bot per turn there is no reason to get more the max(ore_costs) many

        if s.ore_bots < max_ore_cost && s.ore >= b.ore_bot_ore_cost {
            let mut new_state = mine_resources(s);
            new_state.ore -= b.ore_bot_ore_cost;
            new_state.ore_bots += 1;
            q.push_back((cur_time + 1, new_state));
            nothing_built = false;
        }

        // If haven't built anything we wait until there is enough resources
        // OR If we have a lot of ore waiting is probably not a good idea
        if nothing_built || s.ore < 2 * max_ore_cost {
            q.push_back((cur_time + 1, mine_resources(s)));
        }
    }
    max_quality
}

fn part1(inp: &[(i64, Blueprint)]) -> i64 {
    inp.par_iter()
        .map(|(num, cost)| num * eval_blueprint(cost, 24))
        .sum()
}

fn part2(inp: &[(i64, Blueprint)]) -> i64 {
    inp.par_iter()
        .take(3)
        .map(|(_, cost)| eval_blueprint(cost, 32))
        .product()
}

pub fn solve(output: bool) -> Timing {
    let raw_input = include_str!("../input/day19.txt");
    let start = Instant::now();
    let inp = parse(raw_input);
    let parse_time = start.elapsed();
    let p1 = part1(&inp);
    let p1_time = start.elapsed() - parse_time;
    let p2 = part2(&inp);
    let p2_time = start.elapsed() - p1_time;
    if output {
        println!("Day 19");
        println!("\tPart 1: {}", p1);
        println!("\tPart 2: {}", p2);
    }
    Timing {
        day: 19,
        parse: parse_time,
        p1: p1_time,
        p2: p2_time,
    }
}
