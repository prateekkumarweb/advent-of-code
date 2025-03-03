use std::collections::HashMap;

use aoc_runner_derive::aoc;

#[aoc(day5, part1)]
pub fn solve_part1(input: &str) -> usize {
    let mut orders: Vec<(usize, usize)> = vec![];
    let lines = input.lines();
    let mut count = 0;
    for line in lines {
        if line.contains('|') {
            let mut splits = line.split('|').map(|w| w.parse().unwrap());
            orders.push((splits.next().unwrap(), splits.next().unwrap()));
        } else if !line.is_empty() {
            let pages = line
                .split(',')
                .map(|w| w.parse().unwrap())
                .collect::<Vec<usize>>();
            let mut pages_map: HashMap<usize, usize> = HashMap::new();
            pages.iter().enumerate().for_each(|(i, p)| {
                pages_map.insert(*p, i);
            });
            let mut violated = false;
            for order in orders.iter() {
                if let (Some(&a), Some(&b)) = (pages_map.get(&order.0), pages_map.get(&order.1)) {
                    if a > b {
                        violated = true;
                        break;
                    }
                }
            }
            if !violated {
                count += pages[pages.len() / 2];
            }
        }
    }
    count
}

#[aoc(day5, part2)]
pub fn solve_part2(input: &str) -> usize {
    let mut orders: Vec<(usize, usize)> = vec![];
    let lines = input.lines();
    let mut count = 0;
    for line in lines {
        if line.contains('|') {
            let mut splits = line.split('|').map(|w| w.parse().unwrap());
            orders.push((splits.next().unwrap(), splits.next().unwrap()));
        } else if !line.is_empty() {
            let mut pages = line
                .split(',')
                .map(|w| w.parse().unwrap())
                .collect::<Vec<usize>>();
            let mut pages_map: HashMap<usize, usize> = HashMap::new();
            pages.iter().enumerate().for_each(|(i, p)| {
                pages_map.insert(*p, i);
            });
            let mut violated = false;
            for order in orders.iter() {
                if let (Some(&a), Some(&b)) = (pages_map.get(&order.0), pages_map.get(&order.1)) {
                    if a > b {
                        violated = true;
                        break;
                    }
                }
            }
            if violated {
                pages.sort_by(|&a, &b| {
                    if orders.contains(&(a, b)) {
                        return std::cmp::Ordering::Less;
                    } else if orders.contains(&(b, a)) {
                        return std::cmp::Ordering::Greater;
                    }
                    std::cmp::Ordering::Equal
                });
                count += pages[pages.len() / 2];
            }
        }
    }
    count
}
