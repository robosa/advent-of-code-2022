use std::collections::HashMap;

use super::get_input;

pub fn day02(step: u8) -> u32 {
    let combinations = HashMap::from([
        (("A X", 1), 4),
        (("A Y", 1), 8),
        (("A Z", 1), 3),
        (("B X", 1), 1),
        (("B Y", 1), 5),
        (("B Z", 1), 9),
        (("C X", 1), 7),
        (("C Y", 1), 2),
        (("C Z", 1), 6),
        (("A X", 2), 3),
        (("A Y", 2), 4),
        (("A Z", 2), 8),
        (("B X", 2), 1),
        (("B Y", 2), 5),
        (("B Z", 2), 9),
        (("C X", 2), 2),
        (("C Y", 2), 6),
        (("C Z", 2), 7),
    ]);
    get_input("input/day02.txt")
        .lines()
        .map(|s| combinations[&(s, step)])
        .sum()
}
