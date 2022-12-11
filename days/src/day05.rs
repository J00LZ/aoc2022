use itertools::Itertools;
use regex::Regex;

use crate::Day;

pub struct Day05 {
    crates: Vec<Vec<char>>,
    instructions: Vec<(usize, usize, usize)>,
}

impl Day for Day05 {
    fn new(input: String) -> Self
    where
        Self: Sized,
    {
        let (stacks, moves) = input.split("\n\n").collect_tuple().unwrap();
        let mut boxes = stacks
            .lines()
            .map(|line| {
                line.chars()
                    .chunks(4)
                    .into_iter()
                    .map(|chunk| chunk.collect::<String>().trim().to_owned())
                    .collect_vec()
            })
            .rev();
        let stacks = boxes.next().unwrap().len();
        // println!("{:?}", boxes.collect_vec());

        let mut crates = vec![vec![]; stacks];
        for b in boxes {
            for (i, c) in b.iter().enumerate() {
                if c.is_empty() {
                    continue;
                }
                crates[i].push(c.chars().nth(1).unwrap());
            }
        }

        let instructions = moves
            .lines()
            .map(|line| {
                let re = Regex::new(r"move (\d+) from (\d+) to (\d+)").unwrap();
                let caps = re.captures(line).unwrap();
                (
                    caps[1].parse::<usize>().unwrap(),
                    caps[2].parse::<usize>().unwrap(),
                    caps[3].parse::<usize>().unwrap(),
                )
            })
            .collect_vec();

        Self {
            crates,
            instructions,
        }
    }

    fn part1(&self) -> String {
        let mut crates = self.crates.clone();
        for &(count, from, to) in self.instructions.iter() {
            for _ in 0..count {
                let elem = crates[from - 1].pop().unwrap();
                crates[to - 1].push(elem);
            }
        }
        crates.iter().flat_map(|v| v.last()).collect()
    }

    fn part2(&self) -> String {
        let mut crates = self.crates.clone();
        for (count, from, to) in self.instructions.iter() {
            let count = crates[*from - 1].len() - count..;
            let mut c = crates[*from - 1].drain(count).collect_vec();
            crates[*to - 1].append(&mut c);
        }
        crates.iter().flat_map(|v| v.last()).collect()
    }
}
