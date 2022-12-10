use super::get_input;

struct Cpu {
    cycle: u8,
    acc: i32,
}

impl Cpu {
    fn process(&mut self) -> i32 {
        if ((self.cycle % 40) as i32).abs_diff(self.acc) > 1 {
            print!(".");
        } else {
            print!("#");
        }
        self.cycle += 1;
        if self.cycle % 40 == 0 {
            println!();
        }
        if self.cycle % 40 == 20 {
            self.cycle as i32 * self.acc
        } else {
            0
        }
    }
}

pub fn day10(_: u8) -> i32 {
    let mut result = 0;
    let mut cpu = Cpu { cycle: 0, acc: 1 };
    for cmd in get_input("input/day10.txt").lines() {
        result += cpu.process();
        if let Some(v) = cmd.strip_prefix("addx ").map(|s| s.parse().unwrap_or(0)) {
            result += cpu.process();
            cpu.acc += v;
        }
    }
    result
}
