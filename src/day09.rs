use std::{
    cmp::{max, min},
    collections::HashSet,
};

use itertools::Itertools;

use crate::Day;

pub struct Day09 {
    directions: Vec<Direction>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }

    fn chebyshev_distance(&self, other: &Self) -> i32 {
        max((self.x - other.x).abs(), (self.y - other.y).abs())
    }

    fn move_to(&self, dir: Direction) -> Point {
        match dir {
            Direction::Up => Point::new(self.x, self.y + 1),
            Direction::Down => Point::new(self.x, self.y - 1),
            Direction::Left => Point::new(self.x - 1, self.y),
            Direction::Right => Point::new(self.x + 1, self.y),
        }
    }
}

#[derive(Clone, Copy, Debug)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

fn foo(directions: impl Iterator<Item = Direction>, length: usize) -> usize {
    assert!(length > 1);
    let mut v = vec![Point::new(0, 0); length];
    let mut visited = HashSet::new();
    for d in directions {
        v[0] = v[0].move_to(d);
        for p in 0..length - 1 {
            let head = &v[p];
            let tail = &v[p + 1];
            if head.chebyshev_distance(tail) > 1 {
                v[p + 1] = move_point(head, tail);
            }
        }
        visited.insert(v[length - 1]);
    }

    visited.len()
}

fn move_point(head: &Point, tail: &Point) -> Point {
    Point {
        x: tail.x + max(min(head.x - tail.x, 1), -1),
        y: tail.y + max(min(head.y - tail.y, 1), -1),
    }
}

impl Day for Day09 {
    fn new(input: String) -> Self
    where
        Self: Sized,
    {
        Self {
            directions: input
                .lines()
                .filter(|x| !x.is_empty())
                .flat_map(|line| {
                    let (dir, count) = line.split_whitespace().collect_tuple().unwrap();
                    let count = count.parse::<usize>().unwrap();
                    let d = match dir {
                        "U" => Direction::Up,
                        "D" => Direction::Down,
                        "L" => Direction::Left,
                        "R" => Direction::Right,
                        _ => panic!("Huh?"),
                    };
                    vec![d; count]
                })
                .collect(),
        }
    }

    fn part1(&self) -> String {
        // let (mut head_x, mut head_y) = (0i32, 0i32);
        // let (mut tail_x, mut tail_y) = (0i32, 0i32);
        // let mut seen_positions = HashSet::new();
        // for d in self.directions.iter().copied() {
        //     match d {
        //         Direction::Up => head_y += 1,
        //         Direction::Down => head_y -= 1,
        //         Direction::Left => head_x -= 1,
        //         Direction::Right => head_x += 1,
        //     }
        //     if head_x == tail_x && ((head_y - tail_y).abs() >= 2) {
        //         if head_y < tail_y {
        //             tail_y -= 1;
        //         } else {
        //             tail_y += 1;
        //         }
        //     }
        //     if head_y == tail_y && ((head_x - tail_x).abs() >= 2) {
        //         if head_x < tail_x {
        //             tail_x -= 1;
        //         } else {
        //             tail_x += 1;
        //         }
        //     }
        //     // diagonal movement
        //     if head_x != tail_x
        //         && head_y != tail_y
        //         && (((head_x - tail_x).abs() >= 2) || ((head_y - tail_y).abs() >= 2))
        //     {
        //         if head_x < tail_x {
        //             tail_x -= 1;
        //         } else {
        //             tail_x += 1;
        //         }
        //         if head_y < tail_y {
        //             tail_y -= 1;
        //         } else {
        //             tail_y += 1;
        //         }
        //     }
        //     seen_positions.insert((tail_x, tail_y));
        // }

        // seen_positions.len().to_string()
        foo(self.directions.iter().copied(), 2).to_string()
    }

    fn part2(&self) -> String {
        foo(self.directions.iter().copied(), 10).to_string()
    }
}
