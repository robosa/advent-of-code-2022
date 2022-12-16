use std::collections::{HashMap, HashSet};

use nom::{
    branch::alt,
    bytes::complete::tag,
    bytes::complete::take,
    character::complete::u32,
    combinator::{all_consuming, map},
    multi::separated_list1,
    sequence::tuple,
    IResult,
};

use super::get_input;

#[derive(Debug)]
struct Cave {
    flow: u32,
    tunnels: Vec<usize>,
}

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

fn test_paths(
    paths: &[Vec<u32>],
    curr_idx: usize,
    curr_flow: u32,
    remaining: HashSet<(usize, u32)>,
    time: u32,
) -> Vec<(u32, HashSet<(usize, u32)>)> {
    let mut result = vec![(curr_flow, remaining.clone())];
    for (i, flow) in remaining.clone() {
        let cost = paths[curr_idx][i];
        if time > cost + 1 {
            let new_time = time - cost - 1;
            let mut remaining = remaining.clone();
            remaining.remove(&(i, flow));
            result.append(&mut test_paths(
                paths,
                i,
                curr_flow + new_time * flow,
                remaining,
                new_time,
            ));
        }
    }
    result
}

fn mat_mult(left: &[Vec<u32>], right: &[Vec<u32>]) -> Vec<Vec<u32>> {
    let n = left.len();
    let mut res = vec![vec![0; n]; n];
    for i in 0..n {
        for j in 0..n {
            for (k, right) in right.iter().enumerate() {
                res[i][j] += left[i][k] * right[j];
            }
        }
    }
    res
}

fn build_shortest_paths(caves: &[Cave]) -> Vec<Vec<u32>> {
    let n = caves.len();
    let mut adj_mat = vec![vec![0; n]; n];
    for i in 0..n {
        for j in &caves[i].tunnels {
            adj_mat[i][*j] = 1;
        }
    }
    let mut shortest_mat = vec![vec![u32::MAX; n]; n];
    for i in 0..n {
        shortest_mat[i][i] = 0;
        for j in 0..n {
            if adj_mat[i][j] == 1 {
                shortest_mat[i][j] = 1;
            }
        }
    }
    let mut pow_mat = adj_mat.clone();
    for k in 2..30 {
        pow_mat = mat_mult(&pow_mat, &adj_mat);
        for i in 0..n {
            for j in 0..n {
                if pow_mat[i][j] != 0 && k < shortest_mat[i][j] {
                    shortest_mat[i][j] = k;
                }
            }
        }
    }
    shortest_mat
}

pub fn day16(step: u8) -> u32 {
    let input = get_input("input/day16.txt");
    let mut indexes = HashMap::new();
    let mut caves = Vec::new();
    let cave_data = input
        .lines()
        .flat_map(parse_cave)
        .enumerate()
        .map(|(i, (_, (id, flow, tunnels)))| {
            indexes.insert(id.clone(), i);
            (id, flow, tunnels)
        })
        .collect::<Vec<_>>();
    for (_, flow, tunnels) in cave_data {
        let tunnels = tunnels
            .iter()
            .map(|s| indexes.get(s).unwrap())
            .copied()
            .collect();
        caves.push(Cave { flow, tunnels })
    }
    let n = caves.len();
    let paths = build_shortest_paths(&caves);
    let caves_with_flow = (0..n)
        .filter(|&i| caves[i].flow > 0)
        .map(|i| (i, caves[i].flow))
        .collect::<HashSet<_>>();
    let start = indexes.get("AA").unwrap();

    let mut max_flow = 0;
    if step == 1 {
        for (flow, _) in test_paths(&paths, *start, 0, caves_with_flow, 30) {
            max_flow = max_flow.max(flow);
        }
    } else {
        for (flow1, other) in test_paths(&paths, *start, 0, caves_with_flow, 26) {
            for (flow2, _) in test_paths(&paths, *start, 0, other, 26) {
                max_flow = max_flow.max(flow1 + flow2)
            }
        }
    }
    max_flow
}
