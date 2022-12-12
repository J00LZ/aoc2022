use std::ops::{Deref, DerefMut};

use itertools::Itertools;
use rayon::prelude::{IntoParallelRefIterator, ParallelIterator};

use crate::Day;

struct Matrix {
    matrix: Vec<Vec<char>>,
    rows: usize,
    cols: usize,
}

impl Matrix {
    fn new(input: String) -> Self {
        let matrix = input
            .lines()
            .filter(|l| !l.is_empty())
            .map(|l| l.chars().collect_vec())
            .collect_vec();
        let rows = matrix.len();
        let cols = matrix[0].len();
        Self { matrix, rows, cols }
    }

    fn neighbours(&self, pos: &(usize, usize)) -> Vec<(usize, usize)> {
        let (x, y) = *pos;
        let mut neighbours = Vec::new();
        if x > 0 {
            neighbours.push((x - 1, y));
        }
        if x < self.cols - 1 {
            neighbours.push((x + 1, y));
        }
        if y > 0 {
            neighbours.push((x, y - 1));
        }
        if y < self.rows - 1 {
            neighbours.push((x, y + 1));
        }
        neighbours
            .into_iter()
            .filter(|(x_test, y_test)| {
                (self.matrix[*y_test][*x_test] as i32) <= (self.matrix[y][x] as i32 + 1)
            })
            .collect_vec()
    }
}

impl Deref for Matrix {
    type Target = Vec<Vec<char>>;

    fn deref(&self) -> &Self::Target {
        &self.matrix
    }
}

impl DerefMut for Matrix {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.matrix
    }
}

pub struct Day12 {
    matrix: Matrix,
    start_pos: (usize, usize),
    end_pos: (usize, usize),
}

impl Day for Day12 {
    fn new(input: String) -> Self
    where
        Self: Sized,
    {
        let mut matrix = Matrix::new(input);
        let mut start_pos = (0, 0);
        let mut end_pos = (0, 0);
        for (y, row) in matrix.iter_mut().enumerate() {
            for (x, c) in row.iter_mut().enumerate() {
                if *c == 'S' {
                    start_pos = (x, y);
                    *c = 'a';
                } else if *c == 'E' {
                    end_pos = (x, y);
                    *c = 'z';
                }
            }
        }
        Self {
            matrix,
            start_pos,
            end_pos,
        }
    }

    fn part1(&self) -> String {
        let r = pathfinding::directed::dijkstra::dijkstra(
            &self.start_pos,
            |p| {
                self.matrix
                    .neighbours(p)
                    .into_iter()
                    .map(|p| (p, 1))
                    .collect_vec()
            },
            |&p| p == self.end_pos,
        );
        r.unwrap().1.to_string()
    }

    fn part2(&self) -> String {
        let mut start_positions = Vec::new();
        for (y, row) in self.matrix.iter().enumerate() {
            for (x, c) in row.iter().enumerate() {
                if *c == 'a' {
                    start_positions.push((x, y));
                }
            }
        }

        let z = start_positions
            .par_iter()
            .flat_map(|p| {
                pathfinding::directed::dijkstra::dijkstra(
                    p,
                    |p| {
                        self.matrix
                            .neighbours(p)
                            .into_iter()
                            .map(|p| (p, 1))
                            .collect_vec()
                    },
                    |&p| p == self.end_pos,
                )
                .map(|r| r.1)
            })
            .min()
            .unwrap()
            .to_string();

        z
    }
}
