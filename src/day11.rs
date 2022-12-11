use std::{collections::VecDeque, str::FromStr};

use itertools::Itertools;

use crate::Day;

pub struct Day11 {
    monkeys: Vec<Monkey>,
}

#[derive(Debug, Clone)]
struct Monkey {
    queue: VecDeque<i64>,
    op: Op,
    test: i64,
    true_target: usize,
    false_target: usize,
}

impl Monkey {
    fn new(
        queue: VecDeque<i64>,
        op: Op,
        test: i64,
        true_target: usize,
        false_target: usize,
    ) -> Self {
        Self {
            queue,
            op,
            test,
            true_target,
            false_target,
        }
    }

    fn run<F: Fn(i64) -> i64>(&mut self, f: F) -> (usize, i64) {
        let item = self.queue.pop_front().unwrap();
        let new_item = self.op.exec(item);
        let new_item = f(new_item);
        if new_item % self.test == 0 {
            (self.true_target, new_item)
        } else {
            (self.false_target, new_item)
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Op {
    Add(OpItem),
    Mul(OpItem),
}

impl Op {
    fn exec(&self, item: i64) -> i64 {
        match self {
            Op::Add(n) => item + n.get_value(item),
            Op::Mul(n) => item * n.get_value(item),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum OpItem {
    Old,
    New(i64),
}

impl OpItem {
    fn get_value(&self, item: i64) -> i64 {
        match self {
            OpItem::Old => item,
            OpItem::New(n) => *n,
        }
    }
}

impl FromStr for OpItem {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s == "old" {
            Ok(OpItem::Old)
        } else {
            Ok(OpItem::New(s.parse().unwrap()))
        }
    }
}

impl Day11 {
    fn calc_with_factor<F: Fn(i64) -> i64>(&self, loop_max: i64, f: F) -> i64 {
        let mut monkeys = self.monkeys.clone();
        let mut counts = vec![0; self.monkeys.len()];
        let mut current_monkey = 0;
        let mut rounds_done = 0;
        loop {
            let mut dests = vec![];
            {
                let m = &mut monkeys[current_monkey];
                while !m.queue.is_empty() {
                    let (target, item) = m.run(&f);
                    counts[current_monkey] += 1;
                    dests.push((target, item));
                }
            }
            for (target, item) in dests {
                monkeys[target].queue.push_back(item);
            }
            current_monkey += 1;
            if current_monkey == monkeys.len() {
                current_monkey = 0;
                rounds_done += 1;
                if rounds_done == loop_max {
                    break;
                }
            }
        }
        counts.sort_unstable();
        counts.reverse();
        counts[0] * counts[1]
    }
}

impl Day for Day11 {
    fn new(input: String) -> Self
    where
        Self: Sized,
    {
        let mut monkeys = Vec::new();
        for l in input
            .lines()
            .filter(|l| !l.is_empty())
            .chunks(6)
            .into_iter()
        {
            let (_name, items, op, test, t, f) = l.collect_tuple().unwrap();
            let items = items
                .split(": ")
                .nth(1)
                .unwrap()
                .split(", ")
                .map(|s| s.parse().unwrap())
                .collect();
            let op = op.split(": ").nth(1).unwrap();
            let op = if op.contains('+') {
                let n = op.split(" + ").nth(1).unwrap().parse().unwrap();
                Op::Add(n)
            } else {
                let n = op.split(" * ").nth(1).unwrap().parse().unwrap();
                Op::Mul(n)
            };
            let test = test
                .split(": divisible by ")
                .nth(1)
                .unwrap()
                .parse()
                .unwrap();

            let t = t
                .split(": throw to monkey ")
                .nth(1)
                .unwrap()
                .parse()
                .unwrap();

            let f = f
                .split(": throw to monkey ")
                .nth(1)
                .unwrap()
                .parse()
                .unwrap();
            monkeys.push(Monkey::new(items, op, test, t, f));
        }

        Self { monkeys }
    }

    fn part1(&self) -> String {
        self.calc_with_factor(20, |x| x / 3).to_string()
    }

    fn part2(&self) -> String {
        let factor = self.monkeys.iter().map(|m| m.test).product::<i64>();
        self.calc_with_factor(10_000, |x| x % factor).to_string()
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn monkey_test() {
        use super::*;

        let mut monkey = Monkey::new(
            vec![79, 98].into_iter().collect(),
            Op::Mul(OpItem::New(19)),
            23,
            2,
            3,
        );
        assert_eq!(monkey.run(|x| x / 3), (3, 500));
        assert_eq!(monkey.run(|x| x / 3), (3, 620));
    }
}
