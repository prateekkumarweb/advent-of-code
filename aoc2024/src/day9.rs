use std::collections::HashMap;

use aoc_runner_derive::aoc;

#[aoc(day9, part1)]
pub fn solve_part1(input: &str) -> usize {
    let mut id_vec = vec![];
    for (i, c) in input.trim().chars().enumerate() {
        let d = c.to_digit(10).unwrap();
        if i % 2 == 0 {
            for _ in 0..d {
                id_vec.push(Some(i / 2));
            }
        } else {
            for _ in 0..d {
                id_vec.push(None);
            }
        }
    }
    let mut i = 0;
    while i < id_vec.len() {
        if id_vec[i].is_none() {
            let last = loop {
                let last = id_vec.pop().unwrap();
                if last.is_some() {
                    break last;
                }
            };
            id_vec[i] = last;
        }
        i += 1;
    }

    id_vec
        .iter()
        .copied()
        .map(Option::unwrap)
        .enumerate()
        .map(|(i, v)| i * v)
        .sum::<usize>()
}

#[aoc(day9, part2)]
pub fn solve_part2(input: &str) -> usize {
    let mut id_vec = vec![];
    let mut id_map = HashMap::new();
    for (i, c) in input.trim().chars().enumerate() {
        let d = c.to_digit(10).unwrap();
        if i % 2 == 0 {
            id_map.insert(i / 2, id_vec.len());
            for _ in 0..d {
                id_vec.push(Some(i / 2));
            }
        } else {
            for _ in 0..d {
                id_vec.push(None);
            }
        }
    }
    let mut largest = id_vec.iter().rev().find_map(|&x| x).unwrap();
    while largest > 0 {
        let largest_idx = id_map.get(&largest).copied().unwrap();
        let largest_size = input
            .trim()
            .chars()
            .nth(largest * 2)
            .unwrap()
            .to_digit(10)
            .unwrap();
        let mut is_cont = None;
        for j in 0..largest_idx {
            if id_vec[j].is_none() {
                is_cont = is_cont.or(Some(0)).map(|x| x + 1);
                if is_cont.unwrap() == largest_size {
                    for k in 0..largest_size {
                        id_vec[j - k as usize] = Some(largest);
                        id_vec[largest_idx + k as usize] = None;
                    }
                    break;
                }
            } else {
                is_cont = None;
            }
        }
        largest -= 1;
    }

    id_vec
        .iter()
        .copied()
        .map(|v| v.unwrap_or(0))
        .enumerate()
        .map(|(i, v)| i * v)
        .sum::<usize>()
}
