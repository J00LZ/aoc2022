use std::collections::HashSet;

use crate::Day;

pub struct Day08 {
    grid: Vec<Vec<(HashSet<VisibleFrom>, i64)>>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum VisibleFrom {
    Top,
    Bottom,
    Left,
    Right,
}

impl Day for Day08 {
    fn new(input: String) -> Self
    where
        Self: Sized,
    {
        let mut grid = Vec::new();
        for l in input.lines() {
            let mut line = Vec::new();
            for c in l.chars() {
                line.push(c);
            }
            grid.push(line);
        }
        let get_char = |x: usize, y: usize| grid[y][x];
        let ys = grid.len();
        let xs = grid[0].len();
        let mut grid = vec![vec![(HashSet::new(), 0); xs]; ys];
        for (y, line) in grid.iter_mut().enumerate().take(ys) {
            for (x, (map, score)) in line.iter_mut().enumerate().take(xs) {
                let current_char = get_char(x, y);
                let top_visible = (0..y).all(|y| get_char(x, y) < current_char);
                let bottom_visible = (y + 1..ys).all(|y| get_char(x, y) < current_char);
                let left_visible = (0..x).all(|x| get_char(x, y) < current_char);
                let right_visible = (x + 1..xs).all(|x| get_char(x, y) < current_char);
                if top_visible {
                    map.insert(VisibleFrom::Top);
                }
                if bottom_visible {
                    map.insert(VisibleFrom::Bottom);
                }
                if left_visible {
                    map.insert(VisibleFrom::Left);
                }
                if right_visible {
                    map.insert(VisibleFrom::Right);
                }
                let mut y_up = 0;
                for y in (0..y).rev() {
                    if get_char(x, y) < current_char {
                        y_up += 1;
                    } else {
                        y_up += 1;
                        break;
                    }
                }

                let mut y_down = 0;
                for y in y + 1..ys {
                    if get_char(x, y) < current_char {
                        y_down += 1;
                    } else {
                        y_down += 1;
                        break;
                    }
                }

                let mut x_left = 0;
                for x in (0..x).rev() {
                    if get_char(x, y) < current_char {
                        x_left += 1;
                    } else {
                        x_left += 1;
                        break;
                    }
                }

                let mut x_right = 0;
                for x in x + 1..xs {
                    if get_char(x, y) < current_char {
                        x_right += 1;
                    } else {
                        x_right += 1;
                        break;
                    }
                }

                *score = y_up * y_down * x_left * x_right;
            }
        }

        Self { grid }
    }

    fn part1(&self) -> String {
        self.grid
            .iter()
            .flatten()
            .map(|(a, _)| a)
            .filter(|x| !x.is_empty())
            .count()
            .to_string()
    }

    fn part2(&self) -> String {
        self.grid
            .iter()
            .flatten()
            .map(|(_, a)| a)
            .max()
            .unwrap()
            .to_string()
    }
}
