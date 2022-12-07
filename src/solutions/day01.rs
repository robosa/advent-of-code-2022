use std::cmp::Reverse;

use itertools::Itertools;

use super::get_input;

pub fn day01(step: u8) -> usize {
    let mut calories: Vec<_> = get_input("input/day01.txt")
        .lines()
        .map(|s| s.parse::<usize>().ok())
        .coalesce(|x, y| match (x, y) {
            (Some(x), Some(y)) => Ok(Some(x + y)),
            _ => Err((x, y)),
        })
        .flatten()
        .collect();
    calories.sort_unstable_by_key(|x| Reverse(*x));
    match step {
        1 => calories[0],
        _ => calories[0..3].iter().sum(),
    }
}
