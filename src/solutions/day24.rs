use super::get_input;

#[derive(Clone)]
enum Cell {
    Empty,
    Blizzard,
    Reachable,
}

impl Cell {
    fn is_empty(&self) -> bool {
        matches!(self, Self::Empty)
    }

    fn is_reachable(&self) -> bool {
        matches!(self, Self::Reachable)
    }
}

struct Blizzard {
    p: (usize, usize),
    d: char,
}

struct BlizzMap {
    blizzards: Vec<Blizzard>,
    map: Vec<Vec<Cell>>,
    height: usize,
    width: usize,
    reverse: bool,
}

impl BlizzMap {
    fn init(data: Vec<Vec<char>>, width: usize, height: usize) -> Self {
        let mut blizzards = Vec::new();
        for (j, line) in data.iter().enumerate() {
            for (i, &d) in line.iter().enumerate() {
                if matches!(d, '^' | 'v' | '<' | '>') {
                    blizzards.push(Blizzard { p: (i, j), d });
                }
            }
        }
        let mut map = vec![vec![Cell::Empty; width]; height];
        map[0][1] = Cell::Reachable;
        Self {
            blizzards,
            map,
            height,
            width,
            reverse: false,
        }
    }

    fn reverse(&mut self) {
        self.reverse = !self.reverse;
        self.map = vec![vec![Cell::Empty; self.width]; self.height];
        if self.reverse {
            self.map[self.height - 1][self.width - 2] = Cell::Reachable;
        } else {
            self.map[0][1] = Cell::Reachable;
        }
    }

    fn update(&mut self) {
        let mut new_map = vec![vec![Cell::Empty; self.width]; self.height];
        if self.reverse {
            new_map[self.height - 1][self.width - 2] = Cell::Reachable;
        } else {
            new_map[0][1] = Cell::Reachable;
        }
        for blizzard in self.blizzards.iter_mut() {
            let (x, y) = blizzard.p;
            blizzard.p = match blizzard.d {
                '^' => (x, if y > 1 { y - 1 } else { self.height - 2 }),
                'v' => (x, if y < self.height - 2 { y + 1 } else { 1 }),
                '<' => (if x > 1 { x - 1 } else { self.width - 2 }, y),
                '>' => (if x < self.width - 2 { x + 1 } else { 1 }, y),
                _ => unreachable!(),
            };
            new_map[blizzard.p.1][blizzard.p.0] = Cell::Blizzard;
        }
        for (j, line) in new_map.iter_mut().enumerate().take(self.height - 1).skip(1) {
            for (i, c) in line.iter_mut().enumerate().take(self.width - 1).skip(1) {
                if c.is_empty()
                    && (self.map[j][i].is_reachable()
                        || self.map[j - 1][i].is_reachable()
                        || self.map[j + 1][i].is_reachable()
                        || self.map[j][i - 1].is_reachable()
                        || self.map[j][i + 1].is_reachable())
                {
                    *c = Cell::Reachable;
                }
            }
        }
        if self.reverse && self.map[1][1].is_reachable() {
            new_map[0][1] = Cell::Reachable;
        }
        if !self.reverse && self.map[self.height - 2][self.width - 2].is_reachable() {
            new_map[self.height - 1][self.width - 2] = Cell::Reachable;
        }
        self.map = new_map;
    }
}

pub fn day24(step: u8) -> usize {
    let data = get_input("input/day24.txt")
        .lines()
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();
    let width = data[0].len();
    let height = data.len();
    let mut blizz_map = BlizzMap::init(data, width, height);
    let mut count = 0;
    while !blizz_map.map[height - 1][width - 2].is_reachable() {
        blizz_map.update();
        count += 1;
    }
    if step == 2 {
        blizz_map.reverse();
        while !blizz_map.map[0][1].is_reachable() {
            blizz_map.update();
            count += 1;
        }
        blizz_map.reverse();
        while !blizz_map.map[height - 1][width - 2].is_reachable() {
            blizz_map.update();
            count += 1;
        }
    }
    count
}
