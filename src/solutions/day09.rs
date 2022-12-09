use std::collections::HashSet;

use super::get_input;

fn update_knots(rope: &mut Vec<(i32, i32)>) {
    for i in 1..rope.len() {
        let dx = rope[i - 1].0 - rope[i].0;
        let dy = rope[i - 1].1 - rope[i].1;
        if dx.abs() > 1 || dy.abs() > 1 {
            rope[i].0 += dx.signum();
            rope[i].1 += dy.signum();
        }
    }
}

pub fn day09(step: u8) -> usize {
    let len = if step == 1 { 2 } else { 10 };
    let mut rope = vec![(0, 0); len];
    let mut tail_visited = HashSet::from([(0, 0)]);
    for (dir, count) in get_input("input/day09.txt")
        .lines()
        .filter_map(|s| s.split_once(' '))
    {
        for _ in 0..count.parse().unwrap_or(0) {
            match dir {
                "U" => rope[0].1 += 1,
                "D" => rope[0].1 -= 1,
                "L" => rope[0].0 -= 1,
                "R" => rope[0].0 += 1,
                _ => (),
            }
            update_knots(&mut rope);
            tail_visited.insert(rope[len - 1]);
        }
    }
    tail_visited.len()
}
