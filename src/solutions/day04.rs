use super::get_input;

fn parse_pair(str: &str) -> Option<(u8, u8)> {
    str.split_once('-')
        .and_then(|(a, b)| a.parse().ok().zip(b.parse().ok()))
}

fn parse_line(str: &str) -> Option<((u8, u8), (u8, u8))> {
    str.split_once(',')
        .and_then(|(a, b)| parse_pair(a).zip(parse_pair(b)))
}

fn range_included(r1: &(u8, u8), r2: &(u8, u8)) -> bool {
    r1.0 >= r2.0 && r1.1 <= r2.1 || r1.0 <= r2.0 && r1.1 >= r2.1
}

fn range_overlap(r1: &(u8, u8), r2: &(u8, u8)) -> bool {
    r1.0 <= r2.1 && r2.0 <= r1.1
}

pub fn day04(step: u8) -> usize {
    get_input("input/day04.txt")
        .lines()
        .filter_map(parse_line)
        .filter(|(r1, r2)| match step {
            1 => range_included(r1, r2),
            _ => range_overlap(r1, r2),
        })
        .count()
}
