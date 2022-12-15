use std::collections::HashSet;

use itertools::Itertools;

use super::get_input;

type SensorData = ((i64, i64), (i64, i64));

fn line_cover(line: i64, data: &[SensorData], min_x: i64, max_x: i64) -> Vec<(i64, i64)> {
    let mut covers: Vec<(i64, i64)> = Vec::new();
    for ((sx, sy), (bx, by)) in data {
        let sensor_range = (sx.abs_diff(*bx) + sy.abs_diff(*by)) as i64;
        let line_dist = line.abs_diff(*sy) as i64;
        if line_dist <= sensor_range {
            let start = min_x.max(sx - (sensor_range - line_dist));
            let end = max_x.min(sx + (sensor_range - line_dist)) + 1;
            if start < end {
                covers.push((start, end));
            }
        }
    }
    covers.sort_unstable();
    covers
        .into_iter()
        .coalesce(|(a, b), (c, d)| {
            if c <= b {
                Ok((a, b.max(d)))
            } else {
                Err(((a, b), (c, d)))
            }
        })
        .collect()
}

fn parse_coord(s: &str) -> Option<(i64, i64)> {
    s.split_once(", y=")
        .and_then(|(x, y)| x.parse().ok().zip(y.parse().ok()))
}

fn parse_sensor_data(s: &str) -> Option<SensorData> {
    s.strip_prefix("Sensor at x=")
        .and_then(|s| s.split_once(": closest beacon is at x="))
        .and_then(|(s1, s2)| parse_coord(s1).zip(parse_coord(s2)))
}

pub fn day15(step: u8) -> i64 {
    let param = 2000000 * step as i64;
    let data = get_input("input/day15.txt")
        .lines()
        .filter_map(parse_sensor_data)
        .collect::<Vec<_>>();
    if step == 1 {
        let mut s_b = HashSet::new();
        for ((sx, sy), (bx, by)) in &data {
            if *sy == param {
                s_b.insert(*sx);
            };
            if *by == param {
                s_b.insert(*bx);
            };
        }
        line_cover(param, &data, i64::MIN, i64::MAX)
            .iter()
            .fold(0, |sum, (a, b)| sum + b - a)
            - s_b.len() as i64
    } else {
        for line in 0..=param {
            match line_cover(line, &data, 0, param)[0] {
                (a, _) if a > 0 => return line,
                (_, b) if b <= param => return b * param + line,
                _ => (),
            }
        }
        panic!()
    }
}
