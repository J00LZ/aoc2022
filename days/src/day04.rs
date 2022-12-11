use crate::Day;

pub struct Day04 {
    assignments: Vec<((i64, i64), (i64, i64))>,
}

impl Day for Day04 {
    fn new(input: String) -> Self
    where
        Self: Sized,
    {
        let assignments = input
            .lines()
            .filter(|it| !it.is_empty())
            .map(|l| {
                let l = l
                    .split(',')
                    .map(|part| {
                        let p = part
                            .split('-')
                            .map(|s| s.parse::<i64>().unwrap())
                            .collect::<Vec<_>>();
                        (p[0], p[1])
                    })
                    .collect::<Vec<_>>();
                (l[0], l[1])
            })
            .collect();
        Self { assignments }
    }

    fn part1(&self) -> String {
        self.assignments
            .iter()
            .copied()
            .map(|((a, b), (c, d))| {
                i64::from(
                    ((a <= c && c <= b) && (a <= d && d <= b))
                        || ((c <= a && a <= d) && (c <= b && b <= d)),
                )
            })
            .sum::<i64>()
            .to_string()
    }

    fn part2(&self) -> String {
        self.assignments
            .iter()
            .copied()
            .map(|((a, b), (c, d))| {
                i64::from(
                    a <= c && c <= b || a <= d && d <= b || c <= a && a <= d || c <= b && b <= d,
                )
            })
            .sum::<i64>()
            .to_string()
    }
}
