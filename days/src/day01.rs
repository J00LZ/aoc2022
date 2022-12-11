use crate::Day;

pub struct Day01 {
    elfs: Vec<i32>,
}

impl Day for Day01 {
    fn new(input: String) -> Self
    where
        Self: Sized,
    {
        Day01 {
            elfs: input
                .split("\n\n")
                .map(|s| s.split_ascii_whitespace().map(|s| s.parse::<i32>().unwrap()).sum())
                .collect(),
        }
    }

    fn part1(&self) -> String {
        self.elfs.iter().max().unwrap_or(&0).to_string()
    }

    fn part2(&self) -> String {
        let mut e2 = self.elfs.clone();
        e2.sort();
        e2.reverse();
        e2.iter().take(3).sum::<i32>().to_string()
    }
}
