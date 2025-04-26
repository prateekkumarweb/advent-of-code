use std::collections::HashMap;

use aoc::aoc;

fn count_symbols(grid: &Vec<Vec<char>>, start_x: usize, end_x: usize, y: usize) -> usize {
    let mut neighbours = vec![];
    let start_idx = if start_x > 0 { start_x - 1 } else { start_x };
    let end_idx = if end_x < grid[y].len() - 1 {
        end_x + 1
    } else {
        end_x
    };
    if y > 0 {
        for c in &grid[y - 1][start_idx..end_idx] {
            neighbours.push(*c);
        }
    }
    if y < grid.len() - 1 {
        for c in &grid[y + 1][start_idx..end_idx] {
            neighbours.push(*c);
        }
    }
    if start_idx != start_x {
        neighbours.push(grid[y][start_idx]);
    }
    if end_idx != end_x {
        neighbours.push(grid[y][end_idx - 1]);
    }
    neighbours
        .iter()
        .filter(|&c| *c != '.' && !c.is_ascii_digit())
        .count()
}

#[derive(Debug, PartialEq, Eq, Hash)]
struct GearIndex {
    x: usize,
    y: usize,
}

fn find_gears(grid: &Vec<Vec<char>>, start_x: usize, end_x: usize, y: usize) -> Vec<GearIndex> {
    let mut gears = vec![];
    let start_idx = if start_x > 0 { start_x - 1 } else { start_x };
    let end_idx = if end_x < grid[y].len() - 1 {
        end_x + 1
    } else {
        end_x
    };
    if y > 0 {
        for (i, &c) in grid[y - 1][start_idx..end_idx].iter().enumerate() {
            if c == '*' {
                gears.push(GearIndex {
                    x: i + start_idx,
                    y: y - 1,
                });
            }
        }
    }
    if y < grid.len() - 1 {
        for (i, &c) in grid[y + 1][start_idx..end_idx].iter().enumerate() {
            if c == '*' {
                gears.push(GearIndex {
                    x: i + start_idx,
                    y: y + 1,
                });
            }
        }
    }
    if start_idx != start_x {
        let c = grid[y][start_idx];
        if c == '*' {
            gears.push(GearIndex { x: start_idx, y });
        }
    }
    if end_idx != end_x {
        let c = grid[y][end_idx - 1];
        if c == '*' {
            gears.push(GearIndex { x: end_idx - 1, y });
        }
    }
    gears
}

#[aoc(day3, part1)]
pub fn part_1(input: &str) -> String {
    let grid = input
        .lines()
        .map(|line| {
            let chars = line.chars().collect::<Vec<_>>();
            chars
        })
        .collect::<Vec<_>>();

    let width = grid[0].len();

    let mut sum = 0;

    for (y, line) in grid.iter().enumerate() {
        let mut x = 0;
        while x < width {
            let char_x = line[x];
            let mut char_num;
            if char_x.is_ascii_digit() {
                char_num = char_x.to_digit(10).unwrap();
                let start_x = x;
                x += 1;
                while x < width {
                    let char_x = line[x];
                    if char_x.is_ascii_digit() {
                        char_num = char_num * 10 + char_x.to_digit(10).unwrap();
                        x += 1;
                    } else {
                        break;
                    }
                }
                let end_x = x;
                let count = count_symbols(&grid, start_x, end_x, y);
                if count > 0 {
                    sum += char_num;
                }
            }
            x += 1;
        }
    }

    format!("{sum}")
}

#[aoc(day3, part2)]
pub fn part_2(input: &str) -> String {
    let grid = input
        .lines()
        .map(|line| {
            let chars = line.chars().collect::<Vec<_>>();
            chars
        })
        .collect::<Vec<_>>();

    let width = grid[0].len();

    let mut gear_nieghbours_map = HashMap::new();

    for (y, line) in grid.iter().enumerate() {
        let mut x = 0;

        while x < width {
            let char_x = line[x];
            let mut num;

            if char_x.is_ascii_digit() {
                let start_x = x;
                num = char_x.to_digit(10).unwrap();
                x += 1;

                while x < width {
                    let char_x = line[x];

                    if char_x.is_ascii_digit() {
                        num = num * 10 + char_x.to_digit(10).unwrap();
                        x += 1;
                    } else {
                        break;
                    }
                }

                let end_x = x;
                let gears = find_gears(&grid, start_x, end_x, y);

                for gear in gears {
                    let gear_numbers = gear_nieghbours_map.entry(gear).or_insert(vec![]);
                    gear_numbers.push(num);
                }
            }

            x += 1;
        }
    }

    let gear_ratios_sum = gear_nieghbours_map
        .iter()
        .filter_map(|(_, gear_neighbors)| {
            if gear_neighbors.len() == 2 {
                Some(gear_neighbors[0] * gear_neighbors[1])
            } else {
                None
            }
        })
        .sum::<u32>();

    format!("{gear_ratios_sum}")
}
