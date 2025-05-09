use std::collections::HashMap;

#[aoc::aoc(day16, part1)]
fn part_1(input: &str) -> u32 {
    let ticker = "children: 3
cats: 7
samoyeds: 2
pomeranians: 3
akitas: 0
vizslas: 0
goldfish: 5
trees: 3
cars: 2
perfumes: 1"
        .lines()
        .map(|l| {
            l.split_once(": ")
                .map(|(a, b)| (a, b.parse::<u32>().unwrap()))
                .unwrap()
        })
        .collect::<HashMap<_, _>>();

    input
        .lines()
        .map(|line| {
            let (sue, rest) = line.split_once(": ").unwrap();
            let sue = sue.strip_prefix("Sue ").unwrap().parse::<u32>().unwrap();
            let m = rest
                .split(", ")
                .map(|v| {
                    v.split_once(": ")
                        .map(|(a, b)| (a, b.parse::<u32>().unwrap()))
                        .unwrap()
                })
                .all(|(a, b)| ticker.get(a) == Some(&b));
            (sue, m)
        })
        .find(|(_, m)| *m)
        .unwrap()
        .0
}

#[derive(Debug, Clone, Copy, Hash)]
enum Range {
    Gt(u32),
    Lt(u32),
    Eq(u32),
}

impl Range {
    const fn check(self, n: u32) -> bool {
        match self {
            Self::Gt(x) => n > x,
            Self::Lt(x) => n < x,
            Self::Eq(x) => n == x,
        }
    }
}

#[aoc::aoc(day16, part2)]
fn part_2(input: &str) -> u32 {
    let ticker = "children: 3
cats: 7
samoyeds: 2
pomeranians: 3
akitas: 0
vizslas: 0
goldfish: 5
trees: 3
cars: 2
perfumes: 1"
        .lines()
        .map(|l| {
            l.split_once(": ")
                .map(|(a, b)| (a, b.parse::<u32>().unwrap()))
                .map(|(a, b)| {
                    (
                        a,
                        match a {
                            "cats" | "trees" => Range::Gt(b),
                            "pomeranians" | "goldfish" => Range::Lt(b),
                            _ => Range::Eq(b),
                        },
                    )
                })
                .unwrap()
        })
        .collect::<HashMap<_, _>>();

    input
        .lines()
        .map(|line| {
            let (sue, rest) = line.split_once(": ").unwrap();
            let sue = sue.strip_prefix("Sue ").unwrap().parse::<u32>().unwrap();
            let m = rest
                .split(", ")
                .map(|v| {
                    v.split_once(": ")
                        .map(|(a, b)| (a, b.parse::<u32>().unwrap()))
                        .unwrap()
                })
                .all(|(a, b)| ticker.get(a).unwrap().check(b));
            (sue, m)
        })
        .find(|(_, m)| *m)
        .unwrap()
        .0
}
