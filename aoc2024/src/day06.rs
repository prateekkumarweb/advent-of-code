use aoc::aoc;
use itertools::Itertools;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Dir {
    Up,
    Right,
    Down,
    Left,
}

impl Dir {
    const fn turn_90(self) -> Self {
        match self {
            Self::Up => Self::Right,
            Self::Right => Self::Down,
            Self::Down => Self::Left,
            Self::Left => Self::Up,
        }
    }
}

#[aoc(day6, part1)]
pub fn solve_part1(input: &str) -> usize {
    let mut grid = input
        .lines()
        .map(|l| l.trim().chars().collect_vec())
        .collect_vec();
    let mut current_pos = (0, 0);
    'outer: for (i, row) in grid.iter_mut().enumerate() {
        for (j, cell) in row.iter_mut().enumerate() {
            if *cell == '^' {
                *cell = ' ';
                current_pos = (i as isize, j as isize);
                break 'outer;
            }
        }
    }
    let mut current_dir = Dir::Up;
    loop {
        let (i, j) = current_pos;
        grid[i as usize][j as usize] = ' ';
        let next_pos = match current_dir {
            Dir::Up => (i - 1, j),
            Dir::Right => (i, j + 1),
            Dir::Down => (i + 1, j),
            Dir::Left => (i, j - 1),
        };
        if next_pos.0 < 0
            || next_pos.0 >= grid.len() as isize
            || next_pos.1 < 0
            || next_pos.1 >= grid[0].len() as isize
        {
            break;
        }
        let next_cell = grid[next_pos.0 as usize][next_pos.1 as usize];
        if next_cell == '#' {
            current_dir = current_dir.turn_90();
        } else {
            current_pos = next_pos;
        }
    }

    grid.iter()
        .map(|row| row.iter().filter(|&c| *c == ' ').count())
        .sum::<usize>()
}

use std::collections::HashSet;

impl Dir {
    const fn to_char(self) -> char {
        match self {
            Self::Up => '^',
            Self::Right => '>',
            Self::Down => 'v',
            Self::Left => '<',
        }
    }
}

#[aoc(day6, part2)]
pub fn solve_part2(input: &str) -> usize {
    let mut grid = input
        .lines()
        .map(|l| l.trim().chars().collect_vec())
        .collect_vec();
    let mut current_pos = (0, 0);
    'outer: for (i, row) in grid.iter_mut().enumerate() {
        for (j, cell) in row.iter_mut().enumerate() {
            if *cell == '^' {
                current_pos = (i as isize, j as isize);
                break 'outer;
            }
        }
    }
    let mut current_dir = Dir::Up;
    let mut obstacles = HashSet::new();
    let old_grid = grid.clone();
    let old_pos = current_pos;
    let old_dir = current_dir;
    for oi in 0..grid.len() {
        for oj in 0..grid[0].len() {
            if grid[oi][oj] != '#' {
                let obs = (oi as isize, oj as isize);
                grid[oi][oj] = '#';

                loop {
                    let (i, j) = current_pos;
                    if grid[i as usize][j as usize] == '.' {
                        grid[i as usize][j as usize] = current_dir.to_char();
                    }
                    let next_pos = match current_dir {
                        Dir::Up => (i - 1, j),
                        Dir::Right => (i, j + 1),
                        Dir::Down => (i + 1, j),
                        Dir::Left => (i, j - 1),
                    };
                    if next_pos.0 < 0
                        || next_pos.0 >= grid.len() as isize
                        || next_pos.1 < 0
                        || next_pos.1 >= grid[0].len() as isize
                    {
                        break;
                    }
                    let next_cell = grid[next_pos.0 as usize][next_pos.1 as usize];
                    if next_cell == '#' {
                        // print_grid(&grid);
                        current_dir = current_dir.turn_90();
                    } else {
                        if next_cell == current_dir.to_char() {
                            obstacles.insert(obs);
                            break;
                        }
                        current_pos = next_pos;
                    }
                }

                grid = old_grid.clone();
                current_pos = old_pos;
                current_dir = old_dir;
            }
        }
    }

    obstacles.len()
}
