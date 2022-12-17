use std::cmp::Reverse;
use std::collections::{HashMap, HashSet};

use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::{alpha1, u32};
use nom::combinator::{all_consuming, map};
use nom::multi::separated_list1;
use nom::sequence::tuple;
use nom::IResult;

use super::get_input;

fn parse_cave(input: &str) -> IResult<&str, (&str, u32, Vec<&str>)> {
    map(
        all_consuming(tuple((
            tag("Valve "),
            alpha1,
            tag(" has flow rate="),
            u32,
            alt((
                tag("; tunnels lead to valves "),
                tag("; tunnel leads to valve "),
            )),
            separated_list1(tag(", "), alpha1),
        ))),
        |(_, id, _, flow, _, tunnels)| (id, flow, tunnels),
    )(input)
}

fn shortest_paths(adj_list: &[Vec<usize>]) -> Vec<Vec<u32>> {
    let n = adj_list.len();
    let mut shortest = vec![vec![u32::MAX; n]; n];
    for i in 0..n {
        shortest[i][i] = 0;
        for &j in adj_list[i].iter() {
            shortest[i][j] = 1;
        }
    }
    for k in 0..n {
        for i in 0..n {
            for j in 0..n {
                if shortest[i][k] < u32::MAX
                    && shortest[k][j] < u32::MAX
                    && shortest[i][j] > shortest[i][k] + shortest[k][j]
                {
                    shortest[i][j] = shortest[i][k] + shortest[k][j];
                }
            }
        }
    }
    shortest
}

fn check_all_paths(
    caves: &[(usize, u32)],
    paths: &[Vec<u32>],
    curr_idx: usize,
    curr_flow: u32,
    visited: HashSet<usize>,
    time: u32,
) -> Vec<(u32, HashSet<usize>)> {
    let mut result = vec![(curr_flow, visited.clone())];
    for &(i, flow) in caves {
        let cost = paths[curr_idx][i];
        let mut visited = visited.clone();
        if time > cost + 1 && visited.insert(i) {
            let new_time = time - cost - 1;
            let new_flow = curr_flow + new_time * flow;
            result.append(&mut check_all_paths(
                caves, paths, i, new_flow, visited, new_time,
            ));
        }
    }
    result
}

pub fn day16(step: u8) -> u32 {
    let input = get_input("input/day16.txt");
    let mut idx = HashMap::new();
    let mut caves = Vec::new();
    let mut adj_list = Vec::new();
    let data = input
        .lines()
        .flat_map(parse_cave)
        .enumerate()
        .map(|(i, (_, (id, flow, tunnels)))| {
            idx.insert(id, i);
            (i, flow, tunnels)
        })
        .collect::<Vec<_>>();
    for (i, flow, tunnels) in data {
        adj_list.push(tunnels.iter().filter_map(|s| idx.get(s).copied()).collect());
        if flow > 0 {
            caves.push((i, flow));
        }
    }
    let paths = shortest_paths(&adj_list);
    let start = idx.get("AA").copied().unwrap();
    let time = if step == 1 { 30 } else { 26 };

    let mut result = check_all_paths(&caves, &paths, start, 0, HashSet::new(), time);
    if step == 1 {
        result.iter().max_by_key(|&(flow, _)| flow).unwrap().0
    } else {
        let mut max_flow = 0;
        result.sort_unstable_by_key(|&(flow, _)| Reverse(flow));
        for (i, (flow_1, visited_1)) in result[..result.len() - 1].iter().enumerate() {
            let (flow_2, _) = result[i + 1..]
                .iter()
                .find(|(_, visited_2)| visited_1.is_disjoint(visited_2))
                .unwrap();
            max_flow = max_flow.max(flow_1 + flow_2);
        }
        max_flow
    }
}
