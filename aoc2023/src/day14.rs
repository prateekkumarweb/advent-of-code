use std::{
    collections::hash_map::DefaultHasher,
    hash::{Hash, Hasher},
};

use aoc::aoc;

#[derive(Debug, Hash, PartialEq, Eq, Clone)]
struct Grid(Vec<Vec<char>>);

impl Grid {
    fn width(&self) -> usize {
        self.0[0].len()
    }

    fn height(&self) -> usize {
        self.0.len()
    }

    fn tilt_north(&mut self) {
        for c in 0..self.width() {
            for i in 1..self.height() {
                if self.0[i][c] == 'O' {
                    let found = (0..i)
                        .rev()
                        .find(|&j| self.0[j][c] != '.')
                        .map_or(0, |j| j + 1);
                    if self.0[found][c] == '.' {
                        self.0[found][c] = 'O';
                        self.0[i][c] = '.';
                    }
                }
            }
        }
    }

    fn tilt_west(&mut self) {
        for r in 0..self.height() {
            for i in 1..self.width() {
                if self.0[r][i] == 'O' {
                    let found = (0..i)
                        .rev()
                        .find(|&j| self.0[r][j] != '.')
                        .map_or(0, |j| j + 1);
                    if self.0[r][found] == '.' {
                        self.0[r][found] = 'O';
                        self.0[r][i] = '.';
                    }
                }
            }
        }
    }

    fn tilt_south(&mut self) {
        for c in 0..self.width() {
            for i in (0..self.height() - 1).rev() {
                if self.0[i][c] == 'O' {
                    let found = (i + 1..self.height())
                        .find(|&j| self.0[j][c] != '.')
                        .map_or(self.height() - 1, |j| j - 1);
                    if self.0[found][c] == '.' {
                        self.0[found][c] = 'O';
                        self.0[i][c] = '.';
                    }
                }
            }
        }
    }

    fn tilt_east(&mut self) {
        for r in 0..self.height() {
            for i in (0..self.width() - 1).rev() {
                if self.0[r][i] == 'O' {
                    let found = (i + 1..self.width())
                        .find(|&j| self.0[r][j] != '.')
                        .map_or(self.width() - 1, |j| j - 1);
                    if self.0[r][found] == '.' {
                        self.0[r][found] = 'O';
                        self.0[r][i] = '.';
                    }
                }
            }
        }
    }

    fn cycle(&mut self) {
        self.tilt_north();
        self.tilt_west();
        self.tilt_south();
        self.tilt_east();
    }

    fn load(&self) -> usize {
        self.0
            .iter()
            .enumerate()
            .map(|(i, line)| line.iter().filter(|&&c| c == 'O').count() * (self.height() - i))
            .sum()
    }
}

fn tilt_north1(column: &mut Vec<char>) {
    let mut i;
    let mut c = 0;
    loop {
        while c < column.len() && column[c] != '.' {
            c += 1;
        }
        if c == column.len() {
            break;
        }
        i = c + 1;
        while i < column.len() && column[i] == '.' {
            i += 1;
        }
        if i == column.len() {
            break;
        }
        if column[i] == '#' {
            c = i + 1;
        } else if column[i] == 'O' {
            column[c] = 'O';
            column[i] = '.';
            c += 1;
        }
    }
}

#[aoc(day14, part1)]
pub fn part_1(input: &str) -> String {
    let mut input = input.lines().fold(vec![], |mut acc, line| {
        if acc.is_empty() {
            acc.resize(line.len(), vec![]);
        }
        line.char_indices().for_each(|(i, c)| acc[i].push(c));
        acc
    });
    input.iter_mut().for_each(tilt_north1);
    input
        .iter()
        .map(|col| {
            col.iter()
                .enumerate()
                .map(|(i, &c)| if c == 'O' { col.len() - i } else { 0 })
                .sum::<usize>()
        })
        .sum::<usize>()
        .to_string()
}

#[aoc(day14, part2)]
pub fn part_2(input: &str) -> String {
    let mut input = Grid(input.lines().map(|line| line.chars().collect()).collect());
    let num_cycles = 1_000_000_000;
    let mut grid_cycled = vec![];
    input.cycle();
    grid_cycled.push((
        {
            let mut hasher = DefaultHasher::new();
            input.hash(&mut hasher);
            hasher.finish()
        },
        input.load(),
    ));
    let mut tortoise = 0;
    let mut hare = 0;
    for c in 1..num_cycles {
        input.cycle();
        grid_cycled.push((
            {
                let mut hasher = DefaultHasher::new();
                input.hash(&mut hasher);
                hasher.finish()
            },
            input.load(),
        ));
        if c % 2 == 0 {
            tortoise += 1;
            hare += 2;
            if grid_cycled[tortoise] == grid_cycled[hare] {
                break;
            }
        }
    }
    let cycle_length = hare - tortoise;
    let remainder = (num_cycles - 1 - tortoise) % cycle_length;
    grid_cycled[remainder + tortoise].1.to_string()
}
