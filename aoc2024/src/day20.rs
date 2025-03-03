use std::collections::HashMap;

use aoc_runner_derive::aoc;
use itertools::Itertools;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Item {
    Empty,
    Wall,
}

#[derive(Debug)]
struct Grid {
    grid: Vec<Vec<Item>>,
}

impl Grid {
    fn width(&self) -> usize {
        self.grid[0].len()
    }

    fn height(&self) -> usize {
        self.grid.len()
    }
}

#[aoc(day20, part1)]
pub fn solve_part1(input: &str) -> usize {
    let grid = input.lines().map(|l| l.chars().collect_vec()).collect_vec();
    let mut start = (0, 0);
    let mut end = (0, 0);
    let grid = grid
        .iter()
        .enumerate()
        .map(|(i, row)| {
            row.iter()
                .enumerate()
                .map(|(j, &c)| match c {
                    '.' => Item::Empty,
                    '#' => Item::Wall,
                    'S' => {
                        start = (i, j);
                        Item::Empty
                    }
                    'E' => {
                        end = (i, j);
                        Item::Empty
                    }
                    _ => panic!("Invalid character at ({}, {}): {}", i, j, c),
                })
                .collect_vec()
        })
        .collect_vec();
    let grid = Grid { grid };
    let distances = find_distances(&grid, start, end);
    let min_distance = distances[end.0][end.1].0;
    let mut savings = HashMap::new();
    for (i, row) in distances.iter().enumerate() {
        for (j, _) in row.iter().enumerate() {
            if grid.grid[i][j] == Item::Wall {
                if i > 0 && i + 1 < grid.height() {
                    let (up, down) = (distances[i - 1][j].0, distances[i + 1][j].1);
                    if up != usize::MAX && down != usize::MAX && up + down + 2 < min_distance {
                        let saved = min_distance - (up + down + 2);
                        let entry = savings.entry(saved).or_insert(0);
                        *entry += 1;
                    }
                    let (down, up) = (distances[i + 1][j].0, distances[i - 1][j].1);
                    if up != usize::MAX && down != usize::MAX && up + down + 2 < min_distance {
                        let saved = min_distance - (up + down + 2);
                        let entry = savings.entry(saved).or_insert(0);
                        *entry += 1;
                    }
                }
                if j > 0 && j + 1 < grid.width() {
                    let (left, right) = (distances[i][j - 1].0, distances[i][j + 1].1);
                    if left != usize::MAX && right != usize::MAX && left + right + 2 < min_distance
                    {
                        let saved = min_distance - (left + right + 2);
                        let entry = savings.entry(saved).or_insert(0);
                        *entry += 1;
                    }
                    let (right, left) = (distances[i][j + 1].0, distances[i][j - 1].1);
                    if left != usize::MAX && right != usize::MAX && left + right + 2 < min_distance
                    {
                        let saved = min_distance - (left + right + 2);
                        let entry = savings.entry(saved).or_insert(0);
                        *entry += 1;
                    }
                }
            }
        }
    }
    let mut result = 0;
    for (saved, count) in savings {
        if saved >= 100 {
            result += count;
        }
    }
    result
}

fn find_distances(
    grid: &Grid,
    start: (usize, usize),
    end: (usize, usize),
) -> Vec<Vec<(usize, usize)>> {
    let mut queue = std::collections::VecDeque::new();
    let mut distances = vec![vec![(usize::MAX, usize::MAX); grid.width()]; grid.height()];
    queue.push_back(start);
    distances[start.0][start.1].0 = 0;
    while let Some((i, j)) = queue.pop_front() {
        for (di, dj) in &[(0, 1), (1, 0), (0, -1), (-1, 0)] {
            let ni = i as i32 + di;
            let nj = j as i32 + dj;
            if ni >= 0 && ni < grid.height() as i32 && nj >= 0 && nj < grid.width() as i32 {
                let ni = ni as usize;
                let nj = nj as usize;
                if distances[ni][nj].0 == usize::MAX && grid.grid[ni][nj] == Item::Empty {
                    distances[ni][nj].0 = distances[i][j].0 + 1;
                    queue.push_back((ni, nj));
                }
            }
        }
    }
    queue.push_back(end);
    distances[end.0][end.1].1 = 0;
    while let Some((i, j)) = queue.pop_front() {
        for (di, dj) in &[(0, 1), (1, 0), (0, -1), (-1, 0)] {
            let ni = i as i32 + di;
            let nj = j as i32 + dj;
            if ni >= 0 && ni < grid.height() as i32 && nj >= 0 && nj < grid.width() as i32 {
                let ni = ni as usize;
                let nj = nj as usize;
                if distances[ni][nj].1 == usize::MAX && grid.grid[ni][nj] == Item::Empty {
                    distances[ni][nj].1 = distances[i][j].1 + 1;
                    queue.push_back((ni, nj));
                }
            }
        }
    }
    distances
}

#[aoc(day20, part2)]
pub fn solve_part2(input: &str) -> usize {
    let grid = input.lines().map(|l| l.chars().collect_vec()).collect_vec();
    let mut start = (0, 0);
    let mut end = (0, 0);
    let grid = grid
        .iter()
        .enumerate()
        .map(|(i, row)| {
            row.iter()
                .enumerate()
                .map(|(j, &c)| match c {
                    '.' => Item::Empty,
                    '#' => Item::Wall,
                    'S' => {
                        start = (i, j);
                        Item::Empty
                    }
                    'E' => {
                        end = (i, j);
                        Item::Empty
                    }
                    _ => panic!("Invalid character at ({}, {}): {}", i, j, c),
                })
                .collect_vec()
        })
        .collect_vec();
    let grid = Grid { grid };
    let distances = find_distances(&grid, start, end);
    let min_distance = distances[end.0][end.1].0;
    let mut savings = HashMap::new();
    let distances = distances
        .into_iter()
        .enumerate()
        .flat_map(|(i, row)| row.into_iter().enumerate().map(move |(j, d)| ((i, j), d)))
        .collect_vec();
    for (cheat_start, cheat_end) in distances.iter().tuple_combinations() {
        let (cheat_start_index, cheat_start_distance) = *cheat_start;
        let (cheat_end_index, cheat_end_distance) = *cheat_end;
        if grid.grid[cheat_start_index.0][cheat_start_index.1] == Item::Wall
            || grid.grid[cheat_end_index.0][cheat_end_index.1] == Item::Wall
        {
            continue;
        }
        let cheat_length = cheat_start_index.0.abs_diff(cheat_end_index.0)
            + cheat_start_index.1.abs_diff(cheat_end_index.1);
        if cheat_length > 20 {
            continue;
        }
        if cheat_start_distance.0 + cheat_end_distance.1 + cheat_length < min_distance {
            let saved =
                min_distance - (cheat_start_distance.0 + cheat_end_distance.1 + cheat_length);
            let entry = savings.entry(saved).or_insert(0);
            *entry += 1;
        }
        if cheat_start_distance.1 + cheat_end_distance.0 + cheat_length < min_distance {
            let saved =
                min_distance - (cheat_start_distance.1 + cheat_end_distance.0 + cheat_length);
            let entry = savings.entry(saved).or_insert(0);
            *entry += 1;
        }
    }
    let mut result = 0;
    for (saved, count) in savings {
        if saved >= 100 {
            result += count;
        }
    }
    result
}
