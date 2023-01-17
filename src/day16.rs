use crate::Timing;
use hashbrown::HashSet;
use regex_macro::regex;
use std::{
    collections::{hash_map::Entry, BTreeSet, HashMap},
    time::Instant,
};

type DistanceMap = HashMap<(u8, u8), i64>;
type FlowMap = HashMap<u8, i64>;

fn parse(input: &str) -> (DistanceMap, FlowMap) {
    // Valve AA has flow rate=0; tunnels lead to valves DD, II, BB

    let re = regex!(
        r"Valve (?P<cur>[A-Z]{2}) has flow rate=(?P<flow>\d+); tunnels? leads? to valves? (?P<next>.*)"
    );
    let mut pre_parse: Vec<(&str, i64, &str)> = vec![];
    // Remap the Name to an ID: "AA" ==> 1
    let mut lookup_id = HashMap::<&str, u8>::new();
    lookup_id.insert("AA", 0);
    let mut id = 1;
    let mut id_high = 128;

    // Find IDs for each room, rooms with a flow start at 1 and those without flow start at 128
    // this makes it possible to use the ids in a bitmask later
    for line in input.lines() {
        let caps = re.captures(line).unwrap();
        let name = caps.name("cur").unwrap().as_str();
        let flow: i64 = caps["flow"].parse().unwrap();
        let connection = caps.name("next").unwrap().as_str();
        if flow == 0 {
            if name != "AA" {
                lookup_id.insert(name, id_high);
                id_high += 1;
            }
        } else {
            lookup_id.insert(name, id);
            id += 1;
        }
        pre_parse.push((name, flow, connection));
    }

    let mut all_rooms = HashMap::<u8, (i64, HashSet<u8>)>::new();
    for (name, flow, connection) in pre_parse {
        // Store the flowrate and the connected rooms
        let mut room_info = (flow, HashSet::<u8>::new());

        for room in connection.split(", ") {
            room_info.1.insert(*lookup_id.get(room).unwrap());
        }
        all_rooms.insert(*lookup_id.get(name).unwrap(), room_info);
    }
    // We only really care about the rooms that have an active flow, which reduces the
    // amount of rooms we have to check later
    let rooms_with_flow = all_rooms
        .iter()
        .filter_map(|(&name, room)| {
            if room.0 > 0 {
                Some((name, room.0))
            } else {
                None
            }
        })
        .collect();
    // Stupid bfs to fill in the distances between each node
    // https://en.wikipedia.org/wiki/Floyd-Warshall_algorithm might be faster
    // but p1 and p2 take 200x the time so who cares
    let mut distances = HashMap::<(u8, u8), i64>::new();

    for (&name, _) in all_rooms.iter() {
        let mut cur = HashSet::<u8>::new();
        cur.insert(name);
        let mut next = HashSet::<u8>::new();
        let mut dist = 0;

        distances.insert((name, name), 0);

        loop {
            if cur.is_empty() {
                break;
            }
            dist += 1;
            for p in cur {
                for &neigh in &all_rooms.get(&p).unwrap().1 {
                    let pair = (name, neigh);
                    if let Entry::Vacant(e) = distances.entry(pair) {
                        e.insert(dist);
                        next.insert(neigh);
                    }
                }
            }
            cur = next;
            next = HashSet::new();
        }
    }

    (distances, rooms_with_flow)
}

fn part1(inp: &(DistanceMap, FlowMap)) -> i64 {
    let (distances, rooms_to_visit) = inp;
    // This is just normal DFS over the whole reduced graph
    // Start at "AA" which has ID 0
    // seen is a 16bit bit mask
    dfs(0, 30, 0u16, rooms_to_visit, distances)
}

fn dfs(
    cur: u8,
    rem_time: i64,
    seen: u16,
    rooms_to_visit: &FlowMap,
    distances: &DistanceMap,
) -> i64 {
    // Add the current room to the visited rooms
    let seen_new = seen | 1 << cur;

    // And remove the room from the todo list
    let mut rooms_visit_new = rooms_to_visit.clone();
    rooms_visit_new.remove(&cur);

    // Actually to the DFS and keep the best flow around
    let mut best_flow = 0;
    for (&target, &flow) in &rooms_visit_new {
        // Only expand if we can reach the room in time
        let time_left = rem_time - distances[&(cur, target)] - 1;
        if time_left > 0 {
            let mut flow = flow * time_left;
            flow += dfs(target, time_left, seen_new, &rooms_visit_new, distances);
            best_flow = best_flow.max(flow);
        }
    }
    best_flow
}

fn part2(inp: &(DistanceMap, FlowMap)) -> i64 {
    let (distances, rooms_to_visit) = inp;

    // The idea is that you and the elephant each open vales in different rooms
    // So the whole set of rooms can be split into 2 subsets with no common rooms
    // For which the flow can then be calculated with DFS again and the combined
    // best flow rate wins.

    // To generate the subsets we need to calculate the PowerSet(rooms_to_visit)
    // of the rooms reachable in 26 min
    let mut bitmasks = HashMap::<BTreeSet<u8>, i64>::new();
    gen_bitmasks(
        0,
        0,
        26,
        &BTreeSet::<u8>::new(),
        rooms_to_visit,
        distances,
        &mut bitmasks,
    );

    let mut bitmask_all = BTreeSet::<u8>::new();
    for room in rooms_to_visit.keys() {
        bitmask_all.insert(*room);
    }

    // The PowerSet might not be complete because we weren't able to reach all
    // rooms inside of 26 min. This generates the missing ones by removing elements
    // from bitmask_all via dfs until it finds a subset it knows the score for.
    add_missing(&bitmask_all, &mut bitmasks);

    let mut max_flow = 0i64;

    // my_bitmask is the set of rooms I'm checking, the elephant needs to
    // check all the other ones, which is calculated by removing my_bitmask
    // from the bitmask_all. Loop over all possibilities 2^n subsets and find the best
    for (my_bitmask, my_flow) in &bitmasks {
        let elephant_bitmask = &bitmask_all - my_bitmask;
        if let Some(elephant_flow) = bitmasks.get(&elephant_bitmask) {
            max_flow = max_flow.max(my_flow + elephant_flow);
        }
    }

    max_flow
}

fn gen_bitmasks(
    cur: u8,
    cur_flow: i64,
    rem_time: i64,
    seen: &BTreeSet<u8>,
    rooms_to_visit: &FlowMap,
    distances: &DistanceMap,
    bitmasks: &mut HashMap<BTreeSet<u8>, i64>,
) -> i64 {
    let mut seen_new = seen.clone();
    // Add the current room to the seen rooms
    seen_new.insert(cur);
    // remove the seen rooms from the todo list
    let mut rooms_visit_new = rooms_to_visit.clone();
    for s in seen_new.iter() {
        rooms_visit_new.remove(s);
    }

    let mut bitmap = seen_new.clone();
    bitmap.remove(&0u8);
    // Calc the bitmask and compare with the cur_flow to see if we found a new max
    let bit_flow = bitmasks.get(&bitmap).unwrap_or(&0);
    bitmasks.insert(bitmap, *bit_flow.max(&cur_flow));

    // DFS for all the remaining rooms we need to visit and keep the best_flow around
    let mut best_flow = 0;
    for (&target, &flow) in &rooms_visit_new {
        let time_left = rem_time - distances[&(cur, target)] - 1;
        // only expand if we can reach the room in time
        if time_left > 0 {
            let mut flow = flow * time_left;
            flow += gen_bitmasks(
                target,
                cur_flow + flow,
                time_left,
                &seen_new,
                &rooms_visit_new,
                distances,
                bitmasks,
            );
            best_flow = best_flow.max(flow);
        }
    }
    best_flow
}

fn add_missing(cur: &BTreeSet<u8>, bitmasks: &mut HashMap<BTreeSet<u8>, i64>) -> i64 {
    // If we found a bitmask we haven't see, we try to find a value be removing
    // elements one-by-one and checking if the new mask has a known flow
    // If there is more than one only keep the best around
    if !bitmasks.contains_key(cur) {
        let mut best_flow = 0;
        for target in cur {
            let mut cur_new = cur.clone();
            cur_new.remove(target);
            let new_flow = add_missing(&cur_new, bitmasks);
            best_flow = best_flow.max(new_flow);
        }
        bitmasks.insert(cur.clone(), best_flow);
    }
    *bitmasks.get(cur).unwrap()
}

pub fn solve(output: bool) -> Timing {
    let raw_input = include_str!("../aoc_input/2022/day16.txt");
    let start = Instant::now();
    let inp = parse(raw_input);
    let parse_time = start.elapsed();
    let p1 = part1(&inp);
    let p1_time = start.elapsed() - parse_time;
    let p2 = part2(&inp);
    let p2_time = start.elapsed() - p1_time;
    if output {
        println!("Day 16");
        println!("\tPart 1: {}", p1);
        println!("\tPart 2: {}", p2);
    }

    Timing {
        day: 16,
        parse: parse_time,
        p1: p1_time,
        p2: p2_time,
    }
}
