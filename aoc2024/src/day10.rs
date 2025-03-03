use std::{cell::RefCell, rc::Rc};

use aoc_runner_derive::aoc;
use itertools::Itertools;

#[aoc(day10, part1)]
pub fn solve_part1(input: &str) -> u32 {
    let grid = input
        .lines()
        .map(|line| line.chars().map(|c| c.to_digit(10).unwrap()).collect_vec())
        .collect_vec();
    let mut result = 0;
    for (i, row) in grid.iter().enumerate() {
        for (j, cell) in row.iter().enumerate() {
            if *cell == 0 {
                let grid2 = Rc::new(RefCell::new(grid.clone()));
                result += search(grid2.clone(), i, j, 0);
            }
        }
    }
    result
}

fn neighbours(grid: Rc<RefCell<Vec<Vec<u32>>>>, i: usize, j: usize) -> Vec<(usize, usize)> {
    let mut result = vec![];
    if i > 0 {
        result.push((i - 1, j));
    }
    if j > 0 {
        result.push((i, j - 1));
    }
    if i < grid.as_ref().borrow().len() - 1 {
        result.push((i + 1, j));
    }
    if j < grid.as_ref().borrow()[0].len() - 1 {
        result.push((i, j + 1));
    }
    result
}

fn search(grid: Rc<RefCell<Vec<Vec<u32>>>>, i: usize, j: usize, start: u32) -> u32 {
    if grid.as_ref().borrow()[i][j] == 9 {
        grid.as_ref().borrow_mut()[i][j] = 10;
        return 1;
    }
    grid.as_ref().borrow_mut()[i][j] = 10;
    let neighbours = neighbours(grid.clone(), i, j);
    neighbours
        .into_iter()
        .filter(|n| grid.as_ref().borrow()[n.0][n.1] == start + 1)
        .map(|(i, j)| search(grid.clone(), i, j, start + 1))
        .sum()
}

#[aoc(day10, part2)]
pub fn solve_part2(input: &str) -> u32 {
    let grid = input
        .lines()
        .map(|line| line.chars().map(|c| c.to_digit(10).unwrap()).collect_vec())
        .collect_vec();
    let mut result = 0;
    for (i, row) in grid.iter().enumerate() {
        for (j, cell) in row.iter().enumerate() {
            if *cell == 0 {
                result += search2(&grid, i, j, 0);
            }
        }
    }
    result
}

fn neighbours2(grid: &[Vec<u32>], i: usize, j: usize) -> Vec<(usize, usize)> {
    let mut result = vec![];
    if i > 0 {
        result.push((i - 1, j));
    }
    if j > 0 {
        result.push((i, j - 1));
    }
    if i < grid.len() - 1 {
        result.push((i + 1, j));
    }
    if j < grid[0].len() - 1 {
        result.push((i, j + 1));
    }
    result
}

fn search2(grid: &[Vec<u32>], i: usize, j: usize, start: u32) -> u32 {
    if grid[i][j] == 9 {
        return 1;
    }
    let neighbours = neighbours2(grid, i, j);
    neighbours
        .into_iter()
        .filter(|n| grid[n.0][n.1] == start + 1)
        .map(|(i, j)| search2(grid, i, j, start + 1))
        .sum()
}
