use std::collections::HashSet;

use crate::Day;

pub struct Day03 {
    lines: Vec<String>,
}

fn score_chars(z: impl Iterator<Item = char>) -> i64 {
    z.map(|c| match c {
        'A'..='Z' => c as i64 - 'A' as i64 + 27,
        'a'..='z' => c as i64 - 'a' as i64 + 1,
        _ => 0,
    })
    .sum()
}

impl Day for Day03 {
    fn new(input: String) -> Self
    where
        Self: Sized,
    {
        Self {
            lines: input.lines().map(|l| l.to_string()).collect(),
        }
    }

    fn part1(&self) -> String {
        self.lines
            .iter()
            .map(|l| {
                let len = l.len();
                l.split_at(len / 2)
            })
            .map(|(lhs, rhs)| {
                score_chars(
                    lhs.chars()
                        .collect::<HashSet<_>>()
                        .intersection(&rhs.chars().collect::<HashSet<_>>())
                        .copied(),
                )
            })
            .sum::<i64>()
            .to_string()
    }

    fn part2(&self) -> String {
        self.lines
            .chunks(3)
            .map(|v| {
                let z = match v {
                    [a, b, c] => a
                        .chars()
                        .collect::<HashSet<_>>()
                        .intersection(&b.chars().collect::<HashSet<_>>())
                        .copied()
                        .collect::<HashSet<_>>()
                        .intersection(&c.chars().collect::<HashSet<_>>())
                        .copied()
                        .collect::<Vec<_>>(),
                    _ => unreachable!(),
                };

                score_chars(z.into_iter())
            })
            .sum::<i64>()
            .to_string()
    }
}
