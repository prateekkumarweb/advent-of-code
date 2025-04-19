use aoc_runner_derive::aoc;
use itertools::Itertools;

#[derive(Debug)]
struct Pattern(Vec<Vec<char>>);

impl Pattern {
    fn width(&self) -> usize {
        self.0[0].len()
    }

    fn height(&self) -> usize {
        self.0.len()
    }

    fn transpose(&self) -> Self {
        let mut new = vec![vec![' '; self.height()]; self.width()];
        (0..self.width()).for_each(|x| {
            (0..self.height()).for_each(|y| {
                new[x][y] = self.0[y][x];
            });
        });
        Self(new)
    }

    fn find_mirror(&self) -> usize {
        for i in 1..self.0.len() {
            if (0..i)
                .rev()
                .zip(i..self.height())
                .all(|(a, b)| self.0[a] == self.0[b])
            {
                return i;
            }
        }
        0
    }

    fn find_mirror_smudge(&self) -> usize {
        for i in 1..self.0.len() {
            let (a, b, l) = (0..i)
                .rev()
                .zip(i..self.height())
                .map(|(a, b)| {
                    self.0[a]
                        .iter()
                        .zip(self.0[b].iter())
                        .filter(|(a, b)| a != b)
                        .count()
                })
                .fold((0, 0, 0), |(a, b, l), x| {
                    if x == 0 {
                        (a + 1, b, l + 1)
                    } else if x == 1 {
                        (a, b + 1, l + 1)
                    } else {
                        (a, b, l + 1)
                    }
                });
            if a == l - 1 && b == 1 {
                return i;
            }
        }
        0
    }
}

#[aoc(day13, part1)]
pub fn part_1(input: &str) -> String {
    input
        .split("\n\n")
        .map(|l| {
            l.trim()
                .split('\n')
                .map(|l| l.chars().collect_vec())
                .collect_vec()
        })
        .map(Pattern)
        .map(|p| {
            let top = p.find_mirror();
            let p = p.transpose();
            let left = p.find_mirror();
            top * 100 + left
        })
        .sum::<usize>()
        .to_string()
}

#[aoc(day13, part2)]
pub fn part_2(input: &str) -> String {
    input
        .split("\n\n")
        .map(|l| {
            l.trim()
                .split('\n')
                .map(|l| l.chars().collect_vec())
                .collect_vec()
        })
        .map(Pattern)
        .map(|p| {
            let top = p.find_mirror_smudge();
            let p = p.transpose();
            let left = p.find_mirror_smudge();
            top * 100 + left
        })
        .sum::<usize>()
        .to_string()
}
