use std::collections::{HashMap, HashSet};

use aoc::aoc;
use itertools::Itertools;

#[aoc(day23, part1)]
pub fn solve_part1(input: &str) -> usize {
    let input = input
        .lines()
        .map(|l| {
            let mut parts = l.split('-');
            (
                parts.next().unwrap().trim().to_string(),
                parts.next().unwrap().trim().to_string(),
            )
        })
        .collect_vec();
    let mut connected = HashMap::new();
    for (a, b) in &input {
        let a_val = connected.entry(a.clone()).or_insert_with(HashSet::new);
        a_val.insert(b.clone());
        let b_val = connected.entry(b.clone()).or_insert_with(HashSet::new);
        b_val.insert(a.clone());
    }
    let mut visited = HashSet::new();
    for (a, b) in &input {
        let a_map = connected.get(a).unwrap();
        let b_map = connected.get(b).unwrap();
        let common = a_map.intersection(b_map).collect_vec();
        for c in common {
            let mut triad = vec![a, b, c];
            triad.sort();
            if triad.iter().any(|x| x.starts_with('t')) {
                visited.insert(triad);
            }
        }
    }
    visited.len()
}

#[aoc(day23, part2)]
pub fn solve_part2(input: &str) -> String {
    let input = input
        .lines()
        .map(|l| {
            let mut parts = l.split('-');
            (
                parts.next().unwrap().trim().to_string(),
                parts.next().unwrap().trim().to_string(),
            )
        })
        .collect_vec();
    let mut connected = HashMap::new();
    for (a, b) in &input {
        let a_val = connected.entry(a.clone()).or_insert_with(HashSet::new);
        a_val.insert(b.clone());
        let b_val = connected.entry(b.clone()).or_insert_with(HashSet::new);
        b_val.insert(a.clone());
    }
    let vertices = connected.keys().cloned().collect_vec();
    let mut cliques = HashSet::from_iter(input.iter().map(|(a, b)| vec![a.clone(), b.clone()]));
    let highest_degree = connected
        .values()
        .map(std::collections::HashSet::len)
        .max()
        .unwrap();
    for _ in 3..=highest_degree {
        let mut current_cliques = HashSet::new();
        for clique in &cliques {
            for vertex in &vertices {
                if !clique.contains(vertex) {
                    let mut is_clique = true;
                    for v in clique {
                        if !connected[v].contains(vertex) {
                            is_clique = false;
                            break;
                        }
                    }
                    if is_clique {
                        let mut new_clique = clique.clone();
                        new_clique.push(vertex.clone());
                        new_clique.sort();
                        current_cliques.insert(new_clique);
                    }
                }
            }
        }
        cliques = current_cliques;
        if cliques.is_empty() {
            break;
        }
    }
    let mut clique = cliques.into_iter().next().unwrap();
    clique.sort();
    clique.join(",")
}
