use std::cmp::Reverse;
use std::collections::{HashMap, HashSet};

use nom::branch::alt;
use nom::bytes::complete::{tag, take};
use nom::character::complete::u32;
use nom::combinator::{all_consuming, map};
use nom::multi::separated_list1;
use nom::sequence::tuple;
use nom::IResult;

use super::get_input;

fn parse_id(input: &str) -> IResult<&str, String> {
    map(take(2usize), |s: &str| s.to_string())(input)
}

fn parse_cave(input: &str) -> IResult<&str, (String, u32, Vec<String>)> {
    map(
        all_consuming(tuple((
            tag("Valve "),
            parse_id,
            tag(" has flow rate="),
            u32,
            alt((
                tag("; tunnels lead to valves "),
                tag("; tunnel leads to valve "),
            )),
            separated_list1(tag(", "), parse_id),
        ))),
        |(_, id, _, flow, _, tunnels)| (id, flow, tunnels),
    )(input)
}

fn build_shortest_paths(adj_list: &[Vec<usize>]) -> Vec<Vec<u32>> {
    let n = adj_list.len();
    let mut shortest_mat = vec![vec![u32::MAX; n]; n];
    for i in 0..n {
        shortest_mat[i][i] = 0;
        for j in adj_list[i].iter() {
            shortest_mat[i][*j] = 1;
        }
    }
    for k in 0..n {
        for i in 0..n {
            for j in 0..n {
                if shortest_mat[i][k] < u32::MAX
                    && shortest_mat[k][j] < u32::MAX
                    && shortest_mat[i][j] > shortest_mat[i][k] + shortest_mat[k][j]
                {
                    shortest_mat[i][j] = shortest_mat[i][k] + shortest_mat[k][j];
                }
            }
        }
    }
    shortest_mat
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
    for (i, flow) in caves {
        let cost = paths[curr_idx][*i];
        let mut visited = visited.clone();
        if time > cost + 1 && visited.insert(*i) {
            let new_time = time - cost - 1;
            result.append(&mut check_all_paths(
                caves,
                paths,
                *i,
                curr_flow + new_time * flow,
                visited,
                new_time,
            ));
        }
    }
    result
}

pub fn day16(step: u8) -> u32 {
    let input = get_input("input/day16.txt");
    let mut indexes = HashMap::new();
    let mut caves = Vec::new();
    let mut adj_list = Vec::new();
    let cave_data = input
        .lines()
        .flat_map(parse_cave)
        .enumerate()
        .map(|(i, (_, (id, flow, tunnels)))| {
            indexes.insert(id, i);
            (i, flow, tunnels)
        })
        .collect::<Vec<_>>();
    for (i, flow, tunnels) in cave_data {
        adj_list.push(tunnels.iter().map(|s| *indexes.get(s).unwrap()).collect());
        if flow > 0 {
            caves.push((i, flow));
        }
    }
    let paths = build_shortest_paths(&adj_list);
    let start = indexes.get("AA").unwrap();
    let time = if step == 1 { 30 } else { 26 };

    let mut result = check_all_paths(&caves, &paths, *start, 0, HashSet::new(), time);
    if step == 1 {
        result.iter().max_by_key(|(flow, _)| *flow).unwrap().0
    } else {
        let mut max_flow = 0;
        result.sort_unstable_by_key(|(flow, _)| Reverse(*flow));
        for (i, (flow1, visited1)) in result[..result.len() - 1].iter().enumerate() {
            let (flow2, _) = result[i + 1..]
                .iter()
                .find(|(_, visited2)| visited1.is_disjoint(visited2))
                .unwrap();
            max_flow = max_flow.max(flow1 + flow2);
        }
        max_flow
    }
}
