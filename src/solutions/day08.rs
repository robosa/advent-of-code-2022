use std::collections::HashSet;

use super::get_input;

fn scan(
    line: &[((usize, usize), u8)],
    visibles: &mut HashSet<(usize, usize)>,
    scores: &mut [Vec<usize>],
) {
    let ((x0, y0), mut highest) = line[0];
    visibles.insert((x0, y0));
    let mut line_scores = vec![0; line.len()];
    scores[x0][y0] = 0;

    for (i, ((x, y), h)) in line.iter().enumerate().skip(1) {
        if *h > highest {
            highest = *h;
            visibles.insert((*x, *y));
        }
        let mut j = 1;
        while j < i && *h > line[i - j].1 {
            j += line_scores[i - j];
        }
        line_scores[i] = j;
        scores[*x][*y] *= j;
    }
}

pub fn day08(step: u8) -> usize {
    let data = get_input("input/day08.txt")
        .lines()
        .enumerate()
        .map(|(i, s)| {
            s.chars()
                .enumerate()
                .map(|(j, c)| ((i, j), c.to_digit(10).unwrap_or(0) as u8))
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();
    let len = data.len();
    let mut visibles = HashSet::new();
    let mut scores = vec![vec![1; len]; len];

    for idx in 0..len {
        scan(&data[idx], &mut visibles, &mut scores);
        let row_rev = data[idx].iter().rev().copied().collect::<Vec<_>>();
        scan(&row_rev, &mut visibles, &mut scores);
        let column = data.iter().map(|line| line[idx]).collect::<Vec<_>>();
        scan(&column, &mut visibles, &mut scores);
        let column_rev = column.into_iter().rev().collect::<Vec<_>>();
        scan(&column_rev, &mut visibles, &mut scores);
    }

    if step == 1 {
        visibles.len()
    } else {
        *scores.iter().flatten().max().unwrap()
    }
}
