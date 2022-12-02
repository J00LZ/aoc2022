use std::str::FromStr;

use crate::Day;

pub struct Day02 {
    games: Vec<(RPS, RPS)>,
}

#[derive(Clone, Copy, Debug, PartialEq)]
enum RPS {
    Rock,
    Paper,
    Scissors,
}

impl RPS {
    fn game_result((lhs, rhs): (RPS, RPS)) -> i64 {
        (match (lhs, rhs) {
            (RPS::Rock, RPS::Scissors) => 0,
            (RPS::Paper, RPS::Rock) => 0,
            (RPS::Scissors, RPS::Paper) => 0,
            (RPS::Rock, RPS::Paper) => 6,
            (RPS::Paper, RPS::Scissors) => 6,
            (RPS::Scissors, RPS::Rock) => 6,
            _ => 3,
        } + rhs.score_rhs())
    }

    fn score_rhs(&self) -> i64 {
        match self {
            RPS::Rock => 1,
            RPS::Paper => 2,
            RPS::Scissors => 3,
        }
    }

    fn round_thing(&self, r: RoundResult) -> Self {
        match r {
            RoundResult::Win => self.win(),
            RoundResult::Draw => *self,
            RoundResult::Lose => self.lose(),
        }
    }

    fn win(&self) -> Self {
        match self {
            RPS::Rock => RPS::Paper,
            RPS::Paper => RPS::Scissors,
            RPS::Scissors => RPS::Rock,
        }
    }

    fn lose(&self) -> Self {
        match self {
            RPS::Rock => RPS::Scissors,
            RPS::Paper => RPS::Rock,
            RPS::Scissors => RPS::Paper,
        }
    }

    fn game_result2((lhs, rhs): (RPS, RoundResult)) -> i64 {
        Self::game_result((lhs, lhs.round_thing(rhs)))
    }
}

impl FromStr for RPS {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "A" => Ok(RPS::Rock),
            "B" => Ok(RPS::Paper),
            "C" => Ok(RPS::Scissors),
            "X" => Ok(RPS::Rock),
            "Y" => Ok(RPS::Paper),
            "Z" => Ok(RPS::Scissors),
            _ => Err(()),
        }
    }
}

enum RoundResult {
    Win,
    Lose,
    Draw,
}

impl From<RPS> for RoundResult {
    fn from(rps: RPS) -> Self {
        match rps {
            RPS::Rock => RoundResult::Lose,
            RPS::Paper => RoundResult::Draw,
            RPS::Scissors => RoundResult::Win,
        }
    }
}

impl Day for Day02 {
    fn new(input: String) -> Self
    where
        Self: Sized,
    {
        let games = input
            .lines()
            .filter(|l| !l.is_empty())
            .map(|l| {
                let v = l
                    .split_ascii_whitespace()
                    .map(|s| s.parse().unwrap())
                    .collect::<Vec<_>>();
                (v[0], v[1])
            })
            .collect();
        Day02 { games }
    }

    fn part1(&self) -> String {
        self.games
            .iter()
            .copied()
            .map(RPS::game_result)
            .sum::<i64>()
            .to_string()
    }

    fn part2(&self) -> String {
        self.games
            .iter()
            .copied()
            .map(|(lhs, rhs)| RPS::game_result2((lhs, rhs.into())))
            .sum::<i64>()
            .to_string()
    }
}
