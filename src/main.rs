mod solutions;

use clap::Parser;
use solutions::*;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(value_parser = clap::value_parser!(u8).range(1..=25))]
    day: u8,
    #[arg(value_parser = clap::value_parser!(u8).range(1..=2))]
    step: u8,
}

fn main() {
    let args = Args::parse();
    match args.day {
        1 => println!("{}", day01(args.step)),
        2 => println!("{}", day02(args.step)),
        3 => println!("{}", day03(args.step)),
        4 => println!("{}", day04(args.step)),
        5 => println!("{}", day05(args.step)),
        6 => println!("{}", day06(args.step)),
        7 => println!("{}", day07(args.step)),
        // 8 => println!("{}", day08(args.step)),
        // 9 => println!("{}", day09(args.step)),
        // 10 => println!("{}", day10(args.step)),
        // 11 => println!("{}", day11(args.step)),
        // 12 => println!("{}", day12(args.step)),
        // 13 => println!("{}", day13(args.step)),
        // 14 => println!("{}", day14(args.step)),
        // 15 => println!("{}", day15(args.step)),
        // 16 => println!("{}", day16(args.step)),
        // 17 => println!("{}", day17(args.step)),
        // 18 => println!("{}", day18(args.step)),
        // 19 => println!("{}", day19(args.step)),
        // 20 => println!("{}", day20(args.step)),
        // 21 => println!("{}", day21(args.step)),
        // 22 => println!("{}", day22(args.step)),
        // 23 => println!("{}", day23(args.step)),
        // 24 => println!("{}", day24(args.step)),
        // 25 => println!("{}", day25(args.step)),
        _ => (),
    }
}
