#![allow(clippy::zero_prefixed_literal)]
use once_cell::sync::Lazy;
use paste::paste;

macro_rules! days {
    ($($n:expr),+) => {
        $(
            paste! {
                pub mod [<day $n>];
                pub static [<DAY $n>]: Lazy<[<day $n>]::[<Day $n>]> =
                    Lazy::new(|| [<day $n>]::[<Day $n>]::new(std::fs::read_to_string(format!("./input/day{:02}.txt", $n)).unwrap()) );

                    #[cfg(test)]
                    mod [<tests_day $n>] {
                        use std::fs::{read_dir, DirEntry};

                        use super::*;

                        fn get_files(part: u8) -> Vec<(DirEntry, DirEntry)> {
                            let tests_inputs = read_dir("./testinput")
                                .unwrap()
                                .into_iter()
                                .filter(|it| {
                                    it.as_ref()
                                        .unwrap()
                                        .file_name()
                                        .to_string_lossy()
                                        .starts_with("[<day $n>]")
                                })
                                .flatten()
                                .collect::<Vec<_>>();
                            let outputs = read_dir("./testoutput")
                                .unwrap()
                                .into_iter()
                                .filter(|it| {
                                    it.as_ref()
                                        .unwrap()
                                        .file_name()
                                        .to_string_lossy()
                                        .starts_with(&format!("[<day $n>]-p{}", part))
                                })
                                .flatten()
                                .collect::<Vec<_>>();
                            tests_inputs.into_iter().zip(outputs).collect()
                        }

                        #[test]
                        fn test_part1() {
                            for (i, o) in get_files(1) {
                                let input = std::fs::read_to_string(i.path()).unwrap();
                                let output = std::fs::read_to_string(o.path()).unwrap();
                                let day = [<day $n>]::[<Day $n>]::new(input);
                                assert_eq!(day.part1(), output);
                            }
                        }

                        #[test]
                        fn test_part2() {
                            for (i, o) in get_files(2) {
                                let input = std::fs::read_to_string(i.path()).unwrap();
                                let output = std::fs::read_to_string(o.path()).unwrap();
                                let day = [<day $n>]::[<Day $n>]::new(input);
                                assert_eq!(day.part2(), output);
                            }
                        }
                    }
            }
        )+
        pub static DAYS: Lazy<Vec<&dyn Day>> = Lazy::new(|| {
            vec![
                paste! {
                    $(
                        &*[<DAY $n>] as &dyn Day
                    ),+
                }
            ]
        });

    };
}

days!(01);

pub trait Day: Send + Sync {
    fn new(input: String) -> Self
    where
        Self: Sized;
    fn part1(&self) -> String;
    fn part2(&self) -> String;
}
