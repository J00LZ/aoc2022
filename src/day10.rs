use std::str::FromStr;

use itertools::Itertools;

use crate::Day;

pub struct Day10 {
    instructions: Vec<Insruction>,
}

struct Cpu<F> {
    x: i64,
    code: Vec<Insruction>,
    pc: usize,
    total_tick: usize,
    tick: usize,
    tick_fn: F,
}

impl<F: FnMut(usize, i64)> Cpu<F> {
    fn new(code: Vec<Insruction>, tick_fn: F) -> Self {
        Self {
            x: 1,
            code,
            pc: 0,
            total_tick: 0,
            tick: 0,
            tick_fn,
        }
    }

    fn run(&mut self) {
        while self.pc < self.code.len() {
            (self.tick_fn)(self.total_tick, self.x);
            let ins = self.code[self.pc];
            self.tick += 1;
            self.total_tick += 1;
            if self.tick >= ins.tick_count() {
                match ins {
                    Insruction::Nop => (),
                    Insruction::AddX(x) => {
                        self.x += x;
                    }
                }
                self.tick = 0;
                self.pc += 1;
            }
        }
    }
}

#[derive(Clone, Copy, Debug)]
enum Insruction {
    Nop,
    AddX(i64),
}

impl Insruction {
    fn tick_count(&self) -> usize {
        match self {
            Self::Nop => 1,
            Self::AddX(_) => 2,
        }
    }
}

impl FromStr for Insruction {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let r = s.split_whitespace().collect_vec();
        match r[..] {
            ["noop"] => Ok(Self::Nop),
            ["addx", x] => Ok(Self::AddX(x.parse().unwrap())),
            _ => Err(()),
        }
    }
}

impl Day for Day10 {
    fn new(input: String) -> Self
    where
        Self: Sized,
    {
        Self {
            instructions: input
                .lines()
                .filter(|x| !x.is_empty())
                .map(|x| x.parse().unwrap())
                .collect(),
        }
    }

    fn part1(&self) -> String {
        let mut counter = 0;
        let mut z = Cpu::new(self.instructions.clone(), |total_tick, x| {
            let total_tick = total_tick + 1;
            if total_tick >= 20 && (total_tick - 20) % 40 == 0 {
                counter += total_tick as i64 * x;
            }
        });
        z.run();
        counter.to_string()
    }

    fn part2(&self) -> String {
        let mut screen = vec![vec!['.'; 40]; 6];
        let mut z = Cpu::new(self.instructions.clone(), |total_tick, x| {
            let x_pos = total_tick % 40;
            let y_pos = total_tick / 40;
            if (x - 1..=x + 1).contains(&(x_pos as i64)) {
                screen[y_pos][x_pos] = '#';
            }
        });
        z.run();

        let st = screen
            .into_iter()
            .map(|x| x.into_iter().collect::<String>())
            .join("\n");
        format!("\n{}\n", st)
    }
}
