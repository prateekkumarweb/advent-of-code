use std::collections::VecDeque;

use aoc::aoc;
use itertools::Itertools;

// | - 7 J L F

fn find_neighbors(
    grid: &Vec<Vec<(char, Option<usize>)>>,
    x: usize,
    y: usize,
) -> Vec<(usize, usize)> {
    let mut neighbors = Vec::new();
    let cell = grid[y][x].0;

    if x > 0 {
        let neighbor = grid[y][x - 1].0;
        let is_connected =
            ['S', '-', '7', 'J'].contains(&cell) && ['-', 'F', 'L'].contains(&neighbor);
        if is_connected {
            neighbors.push((x - 1, y));
        }
    }
    if x < grid[y].len() - 1 {
        let neighbor = grid[y][x + 1].0;
        let is_connected =
            ['S', '-', 'F', 'L'].contains(&cell) && ['-', '7', 'J'].contains(&neighbor);
        if is_connected {
            neighbors.push((x + 1, y));
        }
    }
    if y > 0 {
        let neighbor = grid[y - 1][x].0;
        let is_connected =
            ['S', '|', 'L', 'J'].contains(&cell) && ['|', '7', 'F'].contains(&neighbor);
        if is_connected {
            neighbors.push((x, y - 1));
        }
    }
    if y < grid.len() - 1 {
        let neighbor = grid[y + 1][x].0;
        let is_connected =
            ['S', '|', '7', 'F'].contains(&cell) && ['|', 'L', 'J'].contains(&neighbor);
        if is_connected {
            neighbors.push((x, y + 1));
        }
    }

    neighbors
}

fn find_start(grid: &[Vec<(char, Option<usize>)>]) -> (usize, usize) {
    let (mut x, mut y) = (0, 0);
    let mut found = false;
    for (j, row) in grid.iter().enumerate() {
        for (i, cell) in row.iter().enumerate() {
            if cell.0 == 'S' {
                x = i;
                y = j;
                found = true;
                break;
            }
        }
        if found {
            break;
        }
    }
    (x, y)
}

fn replace_start(grid: &mut Vec<Vec<(char, Option<usize>)>>, start_x: usize, start_y: usize) {
    let mut neighbors = find_neighbors(grid, start_x, start_y)
        .iter()
        .map(|(nx, ny)| {
            if *nx > start_x {
                'R'
            } else if *nx < start_x {
                'L'
            } else if *ny > start_y {
                'D'
            } else if *ny < start_y {
                'U'
            } else {
                unreachable!()
            }
        })
        .collect_vec();
    neighbors.sort_unstable();

    match neighbors[..] {
        ['D', 'U'] => grid[start_y][start_x] = ('|', Some(0)),
        ['L', 'R'] => grid[start_y][start_x] = ('-', Some(0)),
        ['D', 'L'] => grid[start_y][start_x] = ('7', Some(0)),
        ['D', 'R'] => grid[start_y][start_x] = ('F', Some(0)),
        ['L', 'U'] => grid[start_y][start_x] = ('J', Some(0)),
        ['U', 'R'] => grid[start_y][start_x] = ('L', Some(0)),
        _ => unreachable!(),
    }
}

fn traverse(grid: &mut Vec<Vec<(char, Option<usize>)>>) {
    let (start_x, start_y) = find_start(grid);

    grid[start_y][start_x].1 = Some(0);

    let mut queue = VecDeque::new();
    queue.push_back((start_x, start_y));

    while !queue.is_empty() {
        let (x, y) = queue.pop_front().unwrap();

        let neighbors = find_neighbors(grid, x, y);

        for neighbor in neighbors {
            let (nx, ny) = neighbor;
            if grid[ny][nx].1.is_none() {
                grid[ny][nx].1 = Some(grid[y][x].1.unwrap() + 1);
                queue.push_back((nx, ny));
            }
        }
    }
}

#[aoc(day10, part1)]
pub fn part_1(input: &str) -> String {
    let mut grid = input
        .lines()
        .map(|l| l.chars().map(|c| (c, None)).collect_vec())
        .collect_vec();

    traverse(&mut grid);

    let mut max = 0;

    for row in &grid {
        for cell in row {
            if let Some(d) = cell.1 {
                max = max.max(d);
            }
        }
    }

    max.to_string()
}

#[aoc(day10, part2)]
pub fn part_2(input: &str) -> String {
    let mut grid = input
        .lines()
        .map(|l| l.chars().map(|c| (c, None)).collect_vec())
        .collect_vec();

    let (start_x, start_y) = find_start(&grid);

    grid[start_y][start_x].1 = Some(0);

    traverse(&mut grid);

    replace_start(&mut grid, start_x, start_y);

    let mut count_i = 0;

    for (y, row) in grid.iter().enumerate() {
        for (x, cell) in row.iter().enumerate() {
            if cell.1.is_none() {
                let no_loop_x = (0..x)
                    .map(|i| grid[y][i])
                    .filter_map(|c| {
                        if c.1.is_some() && c.0 != '-' {
                            match c.0 {
                                '|' => Some((1, 1)),
                                '7' | 'F' => Some((0, 1)),
                                'J' | 'L' => Some((1, 0)),
                                _ => unreachable!(),
                            }
                        } else {
                            None
                        }
                    })
                    .reduce(|(a, b), (c, d)| (a + c, b + d))
                    .unwrap_or((0, 0));
                let no_loop_x = no_loop_x.0.min(no_loop_x.1);

                if no_loop_x % 2 == 1 {
                    count_i += 1;
                }
            }
        }
    }

    count_i.to_string()
}
