use std::collections::{HashSet, VecDeque};

use super::get_input;

struct Grid {
    height: usize,
    width: usize,
    elevations: Vec<Vec<u8>>,
    start: (usize, usize),
    end: (usize, usize),
}

impl Grid {
    fn new(str: &str) -> Self {
        let mut start = (0, 0);
        let mut end = (0, 0);
        let elevations = str
            .lines()
            .enumerate()
            .map(|(i, s)| {
                s.chars()
                    .enumerate()
                    .map(|(j, c)| match c {
                        'S' => {
                            start = (i, j);
                            b'a'
                        }
                        'E' => {
                            end = (i, j);
                            b'z'
                        }
                        c => c as u8,
                    })
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>();
        Self {
            height: elevations.len(),
            width: elevations[0].len(),
            elevations,
            start,
            end,
        }
    }

    fn can_walk(&self, from: (usize, usize), to: (usize, usize), reverse: bool) -> bool {
        if reverse {
            self.can_walk(to, from, false)
        } else {
            self.elevations[to.0][to.1] <= self.elevations[from.0][from.1] + 1
        }
    }

    fn goal_reached(&self, p: (usize, usize), reverse: bool) -> bool {
        if reverse {
            self.elevations[p.0][p.1] == b'a'
        } else {
            !reverse && p == self.end
        }
    }

    fn walk(&self, reverse: bool) -> Option<usize> {
        let start = if reverse { self.end } else { self.start };
        let mut queue = VecDeque::from([(start, 0)]);
        let mut visited = HashSet::new();
        while let Some((p, cost)) = queue.pop_front() {
            if !visited.insert(p) {
                continue;
            }
            if self.goal_reached(p, reverse) {
                return Some(cost);
            }
            if p.0 != 0 && self.can_walk(p, (p.0 - 1, p.1), reverse) {
                queue.push_back(((p.0 - 1, p.1), cost + 1))
            }
            if p.1 != 0 && self.can_walk(p, (p.0, p.1 - 1), reverse) {
                queue.push_back(((p.0, p.1 - 1), cost + 1))
            }
            if p.0 + 1 != self.height && self.can_walk(p, (p.0 + 1, p.1), reverse) {
                queue.push_back(((p.0 + 1, p.1), cost + 1))
            }
            if p.1 + 1 != self.width && self.can_walk(p, (p.0, p.1 + 1), reverse) {
                queue.push_back(((p.0, p.1 + 1), cost + 1))
            }
        }
        None
    }
}

pub fn day12(step: u8) -> usize {
    let grid = Grid::new(&get_input("input/day12.txt"));
    grid.walk(step == 2).expect("no path")
}
