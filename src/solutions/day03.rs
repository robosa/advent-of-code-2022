use std::collections::HashSet;

use itertools::Itertools;

use super::get_input;

pub fn day03(step: u8) -> usize {
    let data = get_input("input/day03.txt");
    let objects: Vec<_> = match step {
        1 => data
            .lines()
            .map(|s| s.split_at(s.len() / 2))
            .filter_map(|(s1, s2)| {
                let set = HashSet::<char>::from_iter(s2.chars());
                s1.chars().find(|c| set.contains(c))
            })
            .collect(),
        _ => data
            .lines()
            .tuples()
            .filter_map(|(s1, s2, s3)| {
                let set: HashSet<char> = HashSet::<char>::from_iter(s2.chars())
                    .intersection(&HashSet::from_iter(s3.chars()))
                    .copied()
                    .collect();
                s1.chars().find(|c| set.contains(c))
            })
            .collect(),
    };
    objects
        .iter()
        .map(|c| match c {
            c if c.is_ascii_lowercase() => *c as usize - 96,
            c if c.is_ascii_uppercase() => *c as usize - 38,
            _ => 0,
        })
        .sum()
}
