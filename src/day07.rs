use itertools::Itertools;

use crate::Day;

pub struct Day07 {
    dir_sizes: Vec<i64>,
}

fn foo(lines: &mut std::str::Lines) -> Vec<i64> {
    let mut subdirs = Vec::new();
    let mut total = 0;

    loop {
        match lines
            .next()
            .map(|s| s.split_whitespace().collect_vec())
            .as_deref()
        {
            Some(["$", "cd", ".."]) | None => break,
            Some(["$", "cd", dir]) if *dir != "/" => {
                subdirs.extend(foo(lines));
                total += subdirs.last().unwrap();
            }
            Some([s, _]) if *s != "$" && *s != "dir" => {
                total += s.parse::<i64>().unwrap();
            }
            _ => (),
        }
    }

    subdirs.push(total);

    subdirs
}

impl Day for Day07 {
    fn new(input: String) -> Self
    where
        Self: Sized,
    {
        Self {
            dir_sizes: foo(&mut input.lines()),
        }
    }

    fn part1(&self) -> String {
        self.dir_sizes
            .iter()
            .copied()
            .filter(|&x| x <= 100000)
            .sum::<i64>()
            .to_string()
    }

    fn part2(&self) -> String {
        let mut sizes = self.dir_sizes.clone();
        // sizes.last() is the size of the root directory
        let needed = 30_000_000 - (70_000_000 - sizes.last().unwrap());
        sizes.sort_unstable();
        sizes
            .into_iter()
            .find(|&s| s >= needed)
            .unwrap()
            .to_string()
    }
}
