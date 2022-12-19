use std::collections::HashSet;

use super::get_input;

type Cube = (u8, u8, u8);

fn parse_cube(s: &str) -> Option<Cube> {
    let v = s.split(',').map(|s| s.parse::<u8>()).collect::<Vec<_>>();
    match &v[..] {
        [Ok(x), Ok(y), Ok(z)] => Some((x + 1, y + 1, z + 1)),
        _ => None,
    }
}

fn count_connections(droplet: &HashSet<Cube>) -> usize {
    let mut count = 0;
    for &(x, y, z) in droplet {
        if droplet.contains(&(x, y, z + 1)) {
            count += 1;
        }
        if droplet.contains(&(x, y + 1, z)) {
            count += 1;
        }
        if droplet.contains(&(x + 1, y, z)) {
            count += 1;
        }
    }
    count
}

fn get_neighbors(cube: Cube, x_max: u8, y_max: u8, z_max: u8) -> Vec<Cube> {
    let mut res = Vec::new();
    let (x, y, z) = cube;
    if x != 0 {
        res.push((x - 1, y, z));
    }
    if y != 0 {
        res.push((x, y - 1, z));
    }
    if z != 0 {
        res.push((x, y, z - 1));
    }
    if x != x_max {
        res.push((x + 1, y, z));
    }
    if y != y_max {
        res.push((x, y + 1, z));
    }
    if z != z_max {
        res.push((x, y, z + 1));
    }
    res
}

fn get_ext_surface_area(droplet: &HashSet<Cube>, x_max: u8, y_max: u8, z_max: u8) -> usize {
    let mut visited = HashSet::new();
    let mut stack = Vec::new();
    stack.push((0, 0, 0));
    visited.insert((0, 0, 0));
    let mut area = 0;
    while let Some(p) = stack.pop() {
        for n in get_neighbors(p, x_max, y_max, z_max) {
            if droplet.contains(&n) {
                area += 1;
            } else if visited.insert(n) {
                stack.push(n)
            }
        }
    }
    area
}

pub fn day18(step: u8) -> usize {
    let mut x_max = 0;
    let mut y_max = 0;
    let mut z_max = 0;
    let droplet = get_input("input/day18.txt")
        .lines()
        .filter_map(parse_cube)
        .inspect(|&(x, y, z)| {
            x_max = x_max.max(x);
            y_max = y_max.max(y);
            z_max = z_max.max(z);
        })
        .collect::<HashSet<_>>();
    if step == 1 {
        droplet.len() * 6 - count_connections(&droplet) * 2
    } else {
        get_ext_surface_area(&droplet, x_max + 1, y_max + 1, z_max + 1)
    }
}
