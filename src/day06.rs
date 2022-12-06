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
        let mut w = self.chars.clone();
        let mut v = self.chars.clone();
        w.append(&mut v);
        for (i, w) in w.windows(4).enumerate() {
            if w.iter().unique().count() == 4 {
                return (i + 4).to_string();
            }
        }
        panic!("Huh?");
    }

    fn part2(&self) -> String {
        let mut w = self.chars.clone();
        let mut v = self.chars.clone();
        w.append(&mut v);
        for (i, w) in w.windows(14).enumerate() {
            if w.iter().unique().count() == 14 {
                return (i + 14).to_string();
            }
        }
        panic!("Huh?");
    }
}
