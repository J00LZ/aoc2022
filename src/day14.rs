use std::{collections::HashMap, ops::Deref};

use itertools::Itertools;

use crate::{
    parser::{Parse, Parser},
    Day,
};

pub struct Day14 {
    world: HashMap<(i64, i64), State>,
}

pub struct Point {
    x: i64,
    y: i64,
}

impl Point {
    fn all_points_from(&self, other: &Point) -> Vec<(i64, i64)> {
        let mut result = vec![];
        for x in self.x.min(other.x)..=self.x.max(other.x) {
            for y in self.y.min(other.y)..=self.y.max(other.y) {
                result.push((x, y));
            }
        }
        result
    }
}

impl<'c> Parse<'c> for Point {
    fn parse(parser: &mut crate::parser::Parser<'c>) -> Self {
        let x = parser.parse_int().unwrap();
        parser.one(',').unwrap();
        let y = parser.parse_int().unwrap();
        Self { x, y }
    }
}

pub struct PointList(Vec<Point>);
impl Deref for PointList {
    type Target = Vec<Point>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<'c> Parse<'c> for PointList {
    fn parse(parser: &mut Parser<'c>) -> Self {
        let mut points = vec![Point::parse(parser)];
        parser.one_with_fn(|c| c.is_ascii_whitespace());
        while parser.parse_str("->").is_some() {
            parser.one_with_fn(|c| c.is_ascii_whitespace());
            points.push(Point::parse(parser));
            parser.one_with_fn(|c| c.is_ascii_whitespace());
        }
        Self(points)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum State {
    Rock,
    Sand,
}

impl Day for Day14 {
    fn new(input: String) -> Self
    where
        Self: Sized,
    {
        let mut world = HashMap::new();
        let points = input
            .lines()
            .map(|line| PointList::parse(&mut Parser::new(line)))
            .collect_vec();
        for p in points {
            for (start, end) in p.iter().tuple_windows() {
                for (x, y) in start.all_points_from(end) {
                    world.insert((x, y), State::Rock);
                }
            }
        }
        Self { world }
    }

    fn part1(&self) -> String {
        let mut m = self.world.clone();
        let mut sand_count = 0;
        'outer: loop {
            let mut location = (500, 0);
            let mut loop_count = 0;
            loop {
                loop_count += 1;
                if loop_count > 1000 {
                    break 'outer;
                }
                let (x, y) = location;
                if !m.contains_key(&(x, y + 1)) {
                    location = (x, y + 1);
                    continue;
                } else if !m.contains_key(&(x - 1, y + 1)) {
                    location = (x - 1, y + 1);
                    continue;
                } else if !m.contains_key(&(x + 1, y + 1)) {
                    location = (x + 1, y + 1);
                    continue;
                } else {
                    break;
                }
            }

            m.insert(location, State::Sand);
            sand_count += 1;
        }

        sand_count.to_string()
    }

    fn part2(&self) -> String {
        let mut m = self.world.clone();
        let max_y = m.keys().map(|(_, y)| y).copied().max().unwrap();
        for x in 0..=1000 {
            m.insert((x, max_y + 2), State::Rock);
        }
        let mut sand_count = 0;
        'outer: loop {
            let mut location = (500, 0);
            loop {
                let (x, y) = location;
                if !m.contains_key(&(x, y + 1)) {
                    location = (x, y + 1);
                    continue;
                } else if !m.contains_key(&(x - 1, y + 1)) {
                    location = (x - 1, y + 1);
                    continue;
                } else if !m.contains_key(&(x + 1, y + 1)) {
                    location = (x + 1, y + 1);
                    continue;
                } else {
                    break;
                }
            }

            m.insert(location, State::Sand);
            sand_count += 1;
            if matches!(m.get(&(500, 0)), Some(State::Sand)) {
                break 'outer;
            }
        }

        sand_count.to_string()
    }
}
