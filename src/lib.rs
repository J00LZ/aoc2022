#![allow(clippy::zero_prefixed_literal)]
use once_cell::sync::Lazy;
use paste::paste;

macro_rules! days {
    ($($n:expr),+ $(,)?) => {
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
                                        .starts_with(&format!("day{:02}", $n))
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
                                        .starts_with(&format!("day{:02}-p{}", $n, part))
                                })
                                .flatten()
                                .collect::<Vec<_>>();
                            tests_inputs.into_iter().zip(outputs).collect()
                        }

                        #[test]
                        fn test_part1() {
                            println!("Testing part 1");
                            for (i, o) in get_files(1) {
                                println!("Testing {}", i.file_name().to_string_lossy());
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
                $(
                    paste! {
                        &*[<DAY $n>] as &dyn Day
                    }
                ),+
            ]
        });

        #[cfg(test)]
        mod tests {
            use std::fs::read_to_string;

            use super::*;

            $(
                paste! {
                    #[test]
                    fn [<day $n _part1>]() {
                        assert_eq!(
                            [<DAY $n>].part1(),
                            read_to_string(&format!("./output/day{:02}-p1.txt", $n)).unwrap()
                        );
                    }

                    #[test]
                    fn [<day $n _part2>]() {
                        assert_eq!(
                            [<DAY $n>].part2(),
                            read_to_string(&format!("./output/day{:02}-p2.txt", $n)).unwrap()
                        );
                    }
                }

            )+
        }

    };
}

days!(01, 02, 03, 04, 05, 06, 07, 08, 09, 10, 11, 12);

pub trait Day: Send + Sync {
    fn new(input: String) -> Self
    where
        Self: Sized;
    fn part1(&self) -> String;
    fn part2(&self) -> String;
}
