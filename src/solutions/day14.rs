use std::collections::HashSet;

use super::get_input;

struct Map {
    filled: HashSet<(i16, i16)>,
    max_y: i16,
}

impl Map {
    fn new(data: &str) -> Self {
        let mut filled = HashSet::<(i16, i16)>::new();
        let mut max_y = 0;
        for line in data.lines() {
            let mut iter = line.split(" -> ").filter_map(|s| {
                s.split_once(',')
                    .and_then(|(x, y)| x.parse().ok().zip(y.parse().ok()))
            });
            let mut start = iter.next().unwrap();
            filled.insert(start);
            max_y = max_y.max(start.1);
            for next in iter {
                for x in start.0.min(next.0) + 1..start.0.max(next.0) {
                    filled.insert((x, start.1));
                }
                for y in start.1.min(next.1) + 1..start.1.max(next.1) {
                    filled.insert((start.0, y));
                }
                filled.insert(next);
                max_y = max_y.max(next.1);
                start = next;
            }
        }
        Self { filled, max_y }
    }

    fn next_drop(&self, p: (i16, i16)) -> Option<(i16, i16)> {
        if !self.filled.contains(&(p.0, p.1 + 1)) {
            Some((p.0, p.1 + 1))
        } else if !self.filled.contains(&(p.0 - 1, p.1 + 1)) {
            Some((p.0 - 1, p.1 + 1))
        } else if !self.filled.contains(&(p.0 + 1, p.1 + 1)) {
            Some((p.0 + 1, p.1 + 1))
        } else {
            None
        }
    }

    fn drop_sand(&mut self) -> bool {
        let mut sand = (500, 0);
        while let Some(next) = self.next_drop(sand) {
            if next.1 == self.max_y {
                return false;
            }
            sand = next;
        }
        self.filled.insert(sand)
    }

    fn fill(&mut self) -> usize {
        self.filled.insert((500, 0));
        let mut sand = HashSet::from([(500, 0)]);
        let mut half_width = 1;
        for y in 1..=self.max_y + 1 {
            for x in 500 - half_width..=500 + half_width {
                if (sand.contains(&(x, y - 1))
                    || sand.contains(&(x - 1, y - 1))
                    || sand.contains(&(x + 1, y - 1)))
                    && self.filled.insert((x, y))
                {
                    sand.insert((x, y));
                }
            }
            half_width += 1;
        }
        sand.len()
    }
}

pub fn day14(step: u8) -> usize {
    let mut map = Map::new(&get_input("input/day14.txt"));
    if step == 1 {
        let mut count = 0;
        while map.drop_sand() {
            count += 1;
        }
        count
    } else {
        map.fill()
    }
}
