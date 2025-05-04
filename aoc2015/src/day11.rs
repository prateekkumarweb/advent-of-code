use std::collections::HashSet;

use aoc::aoc;
use itertools::Itertools;

fn next_str(v: &mut [u8]) {
    let mut n = v.len() - 1;
    loop {
        let c = v[n];
        let next_c = c + 1;
        if c == b'z' {
            v[n] = b'a';
            n -= 1;
        } else {
            v[n] = next_c;
            break;
        }
    }
}

fn is_password(p: &[u8]) -> bool {
    let has_cont = p
        .iter()
        .tuple_windows()
        .any(|(&a, &b, &c)| a + 1 == b && b + 1 == c);
    if !has_cont {
        return false;
    }
    let no_conf = !p.iter().any(|&b| b == b'i' || b == b'o' || b == b'l');
    if !no_conf {
        return false;
    }
    let cont_pairs = p
        .iter()
        .tuple_windows()
        .filter(|&(a, b)| a == b)
        .map(|(a, _)| *a)
        .collect::<HashSet<u8>>()
        .len()
        >= 2;
    if !cont_pairs {
        return false;
    }
    true
}

#[aoc(day11, part1)]
pub fn part_1(input: &str) -> String {
    let mut v = input.trim().bytes().collect_vec();
    loop {
        next_str(&mut v);
        if is_password(&v) {
            break;
        }
    }
    v.into_iter().map(|b| b as char).collect()
}

#[aoc(day11, part2)]
pub fn part_1(input: &str) -> String {
    let mut v = input.trim().bytes().collect_vec();
    let mut c = 0;
    loop {
        next_str(&mut v);
        if is_password(&v) {
            c += 1;
            if c == 2 {
                break;
            }
        }
    }
    v.into_iter().map(|b| b as char).collect()
}
