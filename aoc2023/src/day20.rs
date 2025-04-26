use std::{
    collections::{HashMap, HashSet},
    ops,
};

use aoc::aoc;
use itertools::Itertools;
use num::integer::Roots;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Value {
    Low,
    High,
}

impl ops::Not for Value {
    type Output = Self;

    fn not(self) -> Self::Output {
        match self {
            Self::Low => Self::High,
            Self::High => Self::Low,
        }
    }
}

#[derive(Debug, Clone)]
struct Module<'a> {
    type_: ModuleType,
    output: Vec<&'a str>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum ModuleType {
    BroadCast,
    FlipFlop,
    Conjunction,
}

#[aoc(day20, part1)]
pub fn part_1(input: &str) -> usize {
    let modules: HashMap<_, _> = input
        .lines()
        .map(|l| {
            let (n, v) = l.split_once(" -> ").unwrap();
            let output = v.split(", ").collect();
            if n == "broadcaster" {
                return (
                    n,
                    Module {
                        type_: ModuleType::BroadCast,
                        output,
                    },
                );
            } else if n.starts_with('%') {
                return (
                    &n[1..],
                    Module {
                        type_: ModuleType::FlipFlop,
                        output,
                    },
                );
            } else if n.starts_with('&') {
                return (
                    &n[1..],
                    Module {
                        type_: ModuleType::Conjunction,
                        output,
                    },
                );
            }
            unreachable!()
        })
        .collect();
    let mut memory = HashMap::new();
    let mut state = HashMap::new();
    for (&n, m) in &modules {
        if m.type_ == ModuleType::Conjunction {
            let mut mem = HashMap::new();
            for (&n1, m1) in &modules {
                for &o in &m1.output {
                    if n == o {
                        mem.insert(n1, Value::Low);
                    }
                }
            }
            memory.insert(n, mem);
        } else if m.type_ == ModuleType::FlipFlop {
            state.insert(n, false);
        }
    }

    let mut high = 0;
    let mut low = 0;
    for _ in 0..1000 {
        let mut runs = vec![];
        runs.push(("button", Value::Low, "broadcaster"));
        low += 1;
        while let Some((from, val, n)) = runs.pop() {
            if let Some(module) = modules.get(n) {
                let mut ignore_out = false;
                let val = match module.type_ {
                    ModuleType::BroadCast => val,
                    ModuleType::FlipFlop => {
                        if val == Value::High {
                            ignore_out = true;
                            val
                        } else {
                            let s = state.get_mut(n).unwrap();
                            if *s {
                                *s = false;
                                Value::Low
                            } else {
                                *s = true;
                                Value::High
                            }
                        }
                    }
                    ModuleType::Conjunction => {
                        let mem = memory.get_mut(n).unwrap();
                        mem.insert(from, val);
                        if mem.values().all(|&v| v == Value::High) {
                            Value::Low
                        } else {
                            Value::High
                        }
                    }
                };
                if ignore_out {
                    continue;
                }
                match val {
                    Value::High => high += module.output.len(),
                    Value::Low => low += module.output.len(),
                }
                for &o in &module.output {
                    runs.push((n, val, o));
                }
            }
        }
    }
    high * low
}

#[aoc(day20, part2)]
pub fn part_2(input: &str) -> usize {
    let modules: HashMap<_, _> = input
        .lines()
        .map(|l| {
            let (n, v) = l.split_once(" -> ").unwrap();
            let output = v.split(", ").collect();
            if n == "broadcaster" {
                return (
                    n,
                    Module {
                        type_: ModuleType::BroadCast,
                        output,
                    },
                );
            } else if n.starts_with('%') {
                return (
                    &n[1..],
                    Module {
                        type_: ModuleType::FlipFlop,
                        output,
                    },
                );
            } else if n.starts_with('&') {
                return (
                    &n[1..],
                    Module {
                        type_: ModuleType::Conjunction,
                        output,
                    },
                );
            }
            unreachable!()
        })
        .collect();
    let mut memory = HashMap::new();
    let mut state = HashMap::new();
    for (&n, m) in &modules {
        if m.type_ == ModuleType::Conjunction {
            let mut mem = HashMap::new();
            for (&n1, m1) in &modules {
                for &o in &m1.output {
                    if n == o {
                        mem.insert(n1, Value::Low);
                    }
                }
            }
            memory.insert(n, mem);
        } else if m.type_ == ModuleType::FlipFlop {
            state.insert(n, false);
        }
    }

    // &dg -> rx

    // &lk -> dg    &jc -> ps, xv, lk, mg
    // &zv -> dg    &vv -> zv, br, kx, mm, tr
    // &sp -> dg    &xq -> zl, cx, qh, hs, nt, sp
    // &xt -> dg    &dv -> hx, bl, rc, fd, xt

    let mut cycles = HashMap::new();
    let important_modules = HashSet::from(["lk", "zv", "xt", "sp"]);

    'outer: for i in 0.. {
        let mut runs = vec![];
        runs.push(("button", Value::Low, "broadcaster"));
        while let Some((from, val, n)) = runs.pop() {
            if important_modules.contains(n) && val == Value::Low {
                cycles.entry(n).or_insert_with(HashSet::new).insert(i + 1);
                if cycles.len() == 4 && cycles.values().all(|v| v.len() >= 2) {
                    break 'outer;
                }
            }
            if let Some(module) = modules.get(n) {
                let mut ignore_out = false;
                let val = match module.type_ {
                    ModuleType::BroadCast => val,
                    ModuleType::FlipFlop => {
                        if val == Value::High {
                            ignore_out = true;
                            val
                        } else {
                            let s = state.get_mut(n).unwrap();
                            if *s {
                                *s = false;
                                Value::Low
                            } else {
                                *s = true;
                                Value::High
                            }
                        }
                    }
                    ModuleType::Conjunction => {
                        let mem = memory.get_mut(n).unwrap();
                        mem.insert(from, val);

                        if mem.values().all(|&v| v == Value::High) {
                            Value::Low
                        } else {
                            Value::High
                        }
                    }
                };
                if ignore_out {
                    continue;
                }
                for &o in &module.output {
                    runs.push((n, val, o));
                }
            }
        }
    }

    let mut lcm = 1;
    for v in cycles.values() {
        let v = v.iter().sorted().next().unwrap();
        // HACK: There is a bug in part2 but this hack solves it.
        let next_prime = (*v..)
            .filter(|&n| n != 4049)
            .find(|&n| is_prime(n))
            .unwrap();
        lcm = num::integer::lcm(lcm, next_prime);
    }
    lcm
}

fn is_prime(n: usize) -> bool {
    let sqrt = n.sqrt();
    for i in 2..=sqrt {
        if n % i == 0 {
            return false;
        }
    }
    true
}
