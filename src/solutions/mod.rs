mod day01;
mod day02;
mod day03;
mod day04;
mod day05;
mod day06;
mod day07;
mod day08;
mod day09;
mod day10;
mod day11;
mod day12;
mod day13;
// mod day14;
// mod day15;
// mod day16;
// mod day17;
// mod day18;
// mod day19;
// mod day20;
// mod day21;
// mod day22;
// mod day23;
// mod day24;
// mod day25;

pub use day01::day01;
pub use day02::day02;
pub use day03::day03;
pub use day04::day04;
pub use day05::day05;
pub use day06::day06;
pub use day07::day07;
pub use day08::day08;
pub use day09::day09;
pub use day10::day10;
pub use day11::day11;
pub use day12::day12;
pub use day13::day13;
// pub use day14::day14;
// pub use day15::day15;
// pub use day16::day16;
// pub use day17::day17;
// pub use day18::day18;
// pub use day19::day19;
// pub use day20::day20;
// pub use day21::day21;
// pub use day22::day22;
// pub use day23::day23;
// pub use day24::day24;
// pub use day25::day25;

use std::fs::read_to_string;

pub fn get_input(file_name: &str) -> String {
    read_to_string(file_name).unwrap_or_default()
}
