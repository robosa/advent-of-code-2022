use std::collections::{hash_map::Entry, HashMap, HashSet};

use super::get_input;

#[derive(PartialEq, Eq, Hash, Clone, Copy)]
enum Block {
    HBar,
    Plus,
    Corner,
    VBar,
    Square,
}

impl Block {
    fn get_shape(&self, p: (usize, usize)) -> Vec<(usize, usize)> {
        let (x, y) = p;
        match self {
            Self::HBar => vec![(x, y), (x + 1, y), (x + 2, y), (x + 3, y)],
            Self::Plus => vec![
                (x, y + 1),
                (x + 1, y),
                (x + 1, y + 1),
                (x + 1, y + 2),
                (x + 2, y + 1),
            ],
            Self::Corner => vec![
                (x, y),
                (x + 1, y),
                (x + 2, y),
                (x + 2, y + 1),
                (x + 2, y + 2),
            ],
            Self::VBar => vec![(x, y), (x, y + 1), (x, y + 2), (x, y + 3)],
            Self::Square => vec![(x, y), (x, y + 1), (x + 1, y), (x + 1, y + 1)],
        }
    }

    fn next(&self) -> Self {
        match self {
            Block::HBar => Block::Plus,
            Block::Plus => Block::Corner,
            Block::Corner => Block::VBar,
            Block::VBar => Block::Square,
            Block::Square => Block::HBar,
        }
    }
}

#[derive(PartialEq, Eq, Hash, Clone, Copy)]
struct State {
    block: Block,
    column_offset: [usize; 7],
    wind_idx: usize,
    last_drop: usize,
}

struct Chamber {
    filled: HashSet<(usize, usize)>,
    max_height: usize,
    wind: Vec<char>,
    state: State,
}

impl Chamber {
    fn rest_block(&mut self, p: (usize, usize)) {
        for p in self.state.block.get_shape(p) {
            self.filled.insert(p);
            self.max_height = self.max_height.max(p.1);
        }
        self.state.block = self.state.block.next();
        for i in 0..7 {
            let mut y = self.max_height;
            while !self.filled.contains(&(i + 1, y)) && y > 0 {
                y -= 1;
            }
            self.state.column_offset[i] = self.max_height - y;
        }
    }

    fn drop_block(&mut self) {
        let mut x = 3;
        let mut y = self.max_height + 4;
        loop {
            let temp_x = match self.wind[self.state.wind_idx] {
                '<' => x - 1,
                '>' => x + 1,
                _ => panic!(),
            };
            self.state.wind_idx = (self.state.wind_idx + 1) % self.wind.len();
            if !self.check_collision((temp_x, y)) {
                x = temp_x;
            }
            if self.check_collision((x, y - 1)) {
                break;
            }
            y -= 1;
        }
        self.state.last_drop = self.max_height + 4 - y;
        self.rest_block((x, y));
    }

    fn check_collision(&self, p: (usize, usize)) -> bool {
        self.state
            .block
            .get_shape(p)
            .iter()
            .any(|&(x, y)| x == 0 || x == 8 || y == 0 || self.filled.contains(&(x, y)))
    }
}

pub fn day17(step: u8) -> usize {
    let mut chamber = Chamber {
        filled: HashSet::new(),
        max_height: 0,
        wind: get_input("input/day17.txt").chars().collect(),
        state: State {
            block: Block::HBar,
            column_offset: [0; 7],
            wind_idx: 0,
            last_drop: 0,
        },
    };
    if step == 1 {
        for _ in 0..2022 {
            chamber.drop_block();
        }
        chamber.max_height
    } else {
        let mut state_map = HashMap::new();
        let mut heights = Vec::new();
        let mut count = 0;
        while let Entry::Vacant(e) = state_map.entry(chamber.state) {
            e.insert(count);
            heights.push(chamber.max_height);
            chamber.drop_block();
            count += 1;
        }
        let cycle_start = state_map.get(&chamber.state).copied().unwrap();
        let nb_cycles = 1000000000000 / (count - cycle_start);
        let remainder = 1000000000000 % (count - cycle_start);
        heights[remainder] + nb_cycles * (chamber.max_height - heights[cycle_start])
    }
}
