#![allow(clippy::zero_prefixed_literal)]
use macros::days;

days!(01, 02, 03, 04, 05, 06, 07, 08, 09, 10, 11);

pub trait Day: Send + Sync {
    fn new(input: String) -> Self
    where
        Self: Sized;
    fn part1(&self) -> String;
    fn part2(&self) -> String;
}
