use std::collections::HashSet;

use aoc::aoc;

#[aoc(day3, part1)]
pub fn part_1(input: &str) -> usize {
    let mut start = (0, 0);
    let mut houses = HashSet::new();
    houses.insert(start);
    for c in input.chars() {
        start = match c {
            '^' => (start.0 - 1, start.1),
            'v' => (start.0 + 1, start.1),
            '<' => (start.0, start.1 - 1),
            '>' => (start.0, start.1 + 1),
            _ => start,
        };
        houses.insert(start);
    }
    houses.len()
}

#[aoc(day3, part2)]
pub fn part_2(input: &str) -> usize {
    let mut start = (0, 0);
    let mut start_robo = (0, 0);
    let mut houses = HashSet::new();
    houses.insert(start);
    let mut santa = true;
    for c in input.chars() {
        if santa {
            start = match c {
                '^' => (start.0 - 1, start.1),
                'v' => (start.0 + 1, start.1),
                '<' => (start.0, start.1 - 1),
                '>' => (start.0, start.1 + 1),
                _ => start,
            };
            houses.insert(start);
        } else {
            start_robo = match c {
                '^' => (start_robo.0 - 1, start_robo.1),
                'v' => (start_robo.0 + 1, start_robo.1),
                '<' => (start_robo.0, start_robo.1 - 1),
                '>' => (start_robo.0, start_robo.1 + 1),
                _ => start_robo,
            };
            houses.insert(start_robo);
        }
        santa = !santa;
    }
    houses.len()
}
