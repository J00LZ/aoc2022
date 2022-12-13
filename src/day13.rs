use itertools::Itertools;

use crate::{
    parser::{Parse, Parser},
    Day,
};

pub struct Day13 {
    int_or_list: Vec<IntOrList>,
}

#[derive(Debug, Clone)]
enum IntOrList {
    Int(i64),
    List(Vec<IntOrList>),
}

impl IntOrList {
    fn to_vec(&self) -> Vec<IntOrList> {
        match self {
            Self::Int(i) => vec![Self::Int(*i)],
            Self::List(list) => list.clone(),
        }
    }
}

impl PartialEq for IntOrList {
    fn eq(&self, other: &Self) -> bool {
        self.cmp(other) == std::cmp::Ordering::Equal
    }
}

impl Eq for IntOrList {}

impl PartialOrd for IntOrList {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for IntOrList {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match (self, other) {
            (Self::Int(i1), Self::Int(i2)) => i1.cmp(i2),
            (l, r) => l.to_vec().cmp(&r.to_vec()),
        }
    }
}

fn parse_list(parser: &mut crate::parser::Parser) -> Vec<IntOrList> {
    let mut list = vec![];
    loop {
        if parser.matches("]") {
            break;
        } else if parser.matches(",") {
        } else {
            list.push(parser.parse());
        }
    }
    list
}

impl<'c> Parse<'c> for IntOrList {
    fn parse(parser: &mut crate::parser::Parser<'c>) -> Self {
        if parser.matches("[") {
            Self::List(parse_list(parser))
        } else {
            Self::Int(parser.parse_int().unwrap())
        }
    }
}

fn make_l(i: i64) -> IntOrList {
    let f = format!("[[{}]]", i);
    let mut parser = Parser::new(&f);
    IntOrList::parse(&mut parser)
}

impl Day for Day13 {
    fn new(input: String) -> Self
    where
        Self: Sized,
    {
        Self {
            int_or_list: input
                .lines()
                .filter(|line| !line.is_empty())
                .map(|line| crate::parser::Parser::new(line).parse())
                .collect(),
        }
    }

    fn part1(&self) -> String {
        let mut count = 0;
        for (idx, c) in self.int_or_list.iter().chunks(2).into_iter().enumerate() {
            let (a, b) = c.into_iter().collect_tuple().unwrap();
            if a < b {
                count += idx + 1;
            }
        }

        count.to_string()
    }

    fn part2(&self) -> String {
        let mut l = self.int_or_list.clone();
        l.push(make_l(2));
        l.push(make_l(6));
        l.sort();
        l.iter()
            .positions(|item| item == &make_l(2) || item == &make_l(6))
            .map(|i| i + 1)
            .product::<usize>()
            .to_string()
    }
}
