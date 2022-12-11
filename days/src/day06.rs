use itertools::Itertools;

use crate::Day;

pub struct Day06 {
    chars: Vec<char>,
}

impl Day for Day06 {
    fn new(input: String) -> Self
    where
        Self: Sized,
    {
        Self {
            chars: input.chars().collect(),
        }
    }

    fn part1(&self) -> String {
        for (i, w) in self.chars.windows(4).enumerate() {
            if w.iter().unique().count() == 4 {
                return (i + 4).to_string();
            }
        }
        panic!("Huh?");
    }

    fn part2(&self) -> String {
        for (i, w) in self.chars.windows(14).enumerate() {
            if w.iter().unique().count() == 14 {
                return (i + 14).to_string();
            }
        }
        panic!("Huh?");
    }
}
