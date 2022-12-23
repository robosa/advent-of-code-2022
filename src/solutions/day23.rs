use std::collections::HashSet;

use super::get_input;

struct Elf {
    pos: (i32, i32),
    n: bool,
    s: bool,
    w: bool,
    e: bool,
    ne: bool,
    nw: bool,
    se: bool,
    sw: bool,
}

#[derive(Clone, Copy)]
enum Dir {
    North,
    South,
    West,
    East,
}

impl Dir {
    fn next(&self) -> Self {
        match self {
            Dir::North => Dir::South,
            Dir::South => Dir::West,
            Dir::West => Dir::East,
            Dir::East => Dir::North,
        }
    }
}

impl Elf {
    fn new(elves: &HashSet<(i32, i32)>, pos: (i32, i32)) -> Self {
        let (x, y) = pos;
        Self {
            pos,
            n: elves.contains(&(x, y - 1)),
            s: elves.contains(&(x, y + 1)),
            w: elves.contains(&(x - 1, y)),
            e: elves.contains(&(x + 1, y)),
            ne: elves.contains(&(x + 1, y - 1)),
            nw: elves.contains(&(x - 1, y - 1)),
            se: elves.contains(&(x + 1, y + 1)),
            sw: elves.contains(&(x - 1, y + 1)),
        }
    }

    fn is_idle(&self) -> bool {
        !self.n && !self.s && !self.w && !self.e && !self.ne && !self.nw && !self.se && !self.sw
    }

    fn check_dir(&self, dir: Dir) -> Option<(i32, i32)> {
        match dir {
            Dir::North => self.check_north(4),
            Dir::South => self.check_south(4),
            Dir::West => self.check_west(4),
            Dir::East => self.check_east(4),
        }
    }

    fn check_north(&self, to_check: u8) -> Option<(i32, i32)> {
        if !self.n && !self.nw && !self.ne {
            Some((self.pos.0, self.pos.1 - 1))
        } else if to_check > 1 {
            self.check_south(to_check - 1)
        } else {
            None
        }
    }

    fn check_south(&self, to_check: u8) -> Option<(i32, i32)> {
        if !self.s && !self.sw && !self.se {
            Some((self.pos.0, self.pos.1 + 1))
        } else if to_check > 1 {
            self.check_west(to_check - 1)
        } else {
            None
        }
    }

    fn check_west(&self, to_check: u8) -> Option<(i32, i32)> {
        if !self.w && !self.nw && !self.sw {
            Some((self.pos.0 - 1, self.pos.1))
        } else if to_check > 1 {
            self.check_east(to_check - 1)
        } else {
            None
        }
    }

    fn check_east(&self, to_check: u8) -> Option<(i32, i32)> {
        if !self.e && !self.ne && !self.se {
            Some((self.pos.0 + 1, self.pos.1))
        } else if to_check > 1 {
            self.check_north(to_check - 1)
        } else {
            None
        }
    }
}

fn turn(elves: &HashSet<(i32, i32)>, dir: Dir) -> HashSet<(i32, i32)> {
    let mut proposals = Vec::new();
    let mut new_pos = HashSet::new();
    let mut forbidden = HashSet::new();
    for &(x, y) in elves.iter() {
        let elf = Elf::new(elves, (x, y));
        if elf.is_idle() {
            proposals.push(((x, y), (x, y)));
        } else if let Some((nx, ny)) = elf.check_dir(dir) {
            proposals.push(((x, y), (nx, ny)));
            if !new_pos.insert((nx, ny)) {
                forbidden.insert((nx, ny));
            }
        } else {
            proposals.push(((x, y), (x, y)));
        }
    }
    proposals
        .into_iter()
        .map(|(p1, p2)| if forbidden.contains(&p2) { p1 } else { p2 })
        .collect()
}

pub fn day23(step: u8) -> usize {
    let mut elves = HashSet::new();
    let data = get_input("input/day23.txt");
    for (j, line) in data.lines().enumerate() {
        for (i, c) in line.chars().enumerate() {
            if c == '#' {
                elves.insert((i as i32, j as i32));
            }
        }
    }
    let mut dir = Dir::North;
    if step == 1 {
        for _ in 0..10 {
            elves = turn(&elves, dir);
            dir = dir.next();
        }
        let mut min_x = i32::MAX;
        let mut max_x = i32::MIN;
        let mut min_y = i32::MAX;
        let mut max_y = i32::MIN;
        for &(x, y) in &elves {
            min_x = min_x.min(x);
            max_x = max_x.max(x);
            min_y = min_y.min(y);
            max_y = max_y.max(y);
        }
        ((max_x - min_x + 1) * (max_y - min_y + 1)) as usize - elves.len()
    } else {
        for i in 1.. {
            let new_elves = turn(&elves, dir);
            if new_elves == elves {
                return i;
            }
            elves = new_elves;
            dir = dir.next();
        }
        0
    }
}
