use std::collections::{HashMap, VecDeque};

use aoc_runner_derive::aoc;

#[derive(Debug, Clone, Copy)]
enum Op {
    Gt,
    Lt,
}

#[derive(Debug, Clone, Copy)]
enum Var {
    X,
    M,
    A,
    S,
}

#[derive(Debug, Clone, Copy)]
struct Condition {
    var: Var,
    op: Op,
    val: usize,
}

impl Condition {
    fn opp(&self) -> Self {
        Self {
            var: self.var,
            op: match self.op {
                Op::Gt => Op::Lt,
                Op::Lt => Op::Gt,
            },
            val: match self.op {
                Op::Gt => self.val + 1,
                Op::Lt => self.val - 1,
            },
        }
    }
}

#[derive(Debug, Clone)]
struct Workflow<'a> {
    rules: Vec<(Condition, &'a str)>,
    else_rule: &'a str,
}

#[derive(Debug, Clone)]
struct Part {
    x: usize,
    m: usize,
    a: usize,
    s: usize,
}

fn parse_input(input: &str) -> (HashMap<&str, Workflow>, Vec<Part>) {
    let mut workflows = HashMap::new();
    let mut parts = vec![];
    for line in input.lines() {
        if line.is_empty() {
            continue;
        }
        if line.starts_with('{') {
            let line = line[1..line.len() - 1]
                .split(',')
                .map(|p| p.split_once('=').unwrap());
            let mut x = 0;
            let mut m = 0;
            let mut a = 0;
            let mut s = 0;
            for (var, val) in line {
                match var {
                    "x" => x = val.parse().unwrap(),
                    "m" => m = val.parse().unwrap(),
                    "a" => a = val.parse().unwrap(),
                    "s" => s = val.parse().unwrap(),
                    _ => unreachable!(),
                }
            }
            parts.push(Part { x, m, a, s });
            continue;
        }
        let (name, rules) = line.split_once('{').unwrap();
        let rules = rules[0..rules.len() - 1].split(',');
        let mut parsed_rules = vec![];
        let mut else_rule = "";
        for rule in rules {
            if let Some((cond, r_name)) = rule.split_once(':') {
                let var = match cond[0..1].chars().next().unwrap() {
                    'x' => Var::X,
                    'm' => Var::M,
                    'a' => Var::A,
                    's' => Var::S,
                    _ => unreachable!(),
                };
                let op = match &cond[1..2] {
                    ">" => Op::Gt,
                    "<" => Op::Lt,
                    _ => unreachable!(),
                };
                let val = cond[2..].parse().unwrap();
                parsed_rules.push((Condition { var, op, val }, r_name));
            } else {
                else_rule = rule;
                break;
            }
        }
        workflows.insert(
            name,
            Workflow {
                rules: parsed_rules,
                else_rule,
            },
        );
    }
    (workflows, parts)
}

fn run_part(workflows: &HashMap<&str, Workflow>, part: &Part) -> bool {
    let mut start = "in";
    while start != "A" && start != "R" {
        let wf = workflows.get(start).unwrap();
        let mut matched = wf.else_rule;
        for (cond, to_rule) in wf.rules.iter() {
            let var = match cond.var {
                Var::X => part.x,
                Var::M => part.m,
                Var::A => part.a,
                Var::S => part.s,
            };
            let cond_val = match cond.op {
                Op::Gt => var > cond.val,
                Op::Lt => var < cond.val,
            };
            if cond_val {
                matched = to_rule;
                break;
            }
        }
        start = matched;
    }
    start == "A"
}

#[aoc(day19, part1)]
pub fn part_1(input: &str) -> usize {
    let (workflows, parts) = parse_input(input);
    let mut result = 0;
    for part in parts.iter() {
        if run_part(&workflows, &part) {
            result += part.x + part.m + part.a + part.s;
        }
    }
    result
}

#[derive(Debug, Clone, Copy)]
struct Range(usize, usize);

impl Range {
    fn reduce(self, op: Op, val: usize) -> Option<Self> {
        let Self(from, to) = self;
        match op {
            Op::Gt => {
                let val = val + 1;
                if val <= from {
                    Some(Range(from, to))
                } else if val <= to {
                    Some(Range(val, to))
                } else {
                    None
                }
            }
            Op::Lt => {
                let val = val - 1;
                if val >= to {
                    Some(Range(from, to))
                } else if val >= from {
                    Some(Range(from, val))
                } else {
                    None
                }
            }
        }
    }
}

#[derive(Debug, Clone, Copy)]
struct PartRange {
    x: Range,
    m: Range,
    a: Range,
    s: Range,
}

impl PartRange {
    fn reduce(self, cond: Condition) -> Option<Self> {
        Some(match cond.var {
            Var::X => PartRange {
                x: self.x.reduce(cond.op, cond.val)?,
                ..self
            },
            Var::M => PartRange {
                m: self.m.reduce(cond.op, cond.val)?,
                ..self
            },
            Var::A => PartRange {
                a: self.a.reduce(cond.op, cond.val)?,
                ..self
            },
            Var::S => PartRange {
                s: self.s.reduce(cond.op, cond.val)?,
                ..self
            },
        })
    }

    fn combinations(self) -> usize {
        (self.x.1 - self.x.0 + 1)
            * (self.m.1 - self.m.0 + 1)
            * (self.a.1 - self.a.0 + 1)
            * (self.s.1 - self.s.0 + 1)
    }
}

fn run_part_range<'a>(
    workflows: &'a HashMap<&'a str, Workflow>,
    part_range: PartRange,
    start: &'a str,
) -> Vec<(&'a str, PartRange)> {
    let mut result = vec![];
    let mut current_part = part_range;
    let wf = workflows.get(start).unwrap();
    for &(cond, name) in &wf.rules {
        if let Some(c1) = current_part.reduce(cond) {
            result.push((name, c1));
        }
        if let Some(c2) = current_part.reduce(cond.opp()) {
            current_part = c2;
        } else {
            return result;
        }
    }
    result.push((wf.else_rule, current_part));
    result
}

#[aoc(day19, part2)]
pub fn part_2(input: &str) -> usize {
    let (workflows, _) = parse_input(input);
    let mut queue = VecDeque::from(vec![(
        "in",
        PartRange {
            x: Range(1, 4000),
            m: Range(1, 4000),
            a: Range(1, 4000),
            s: Range(1, 4000),
        },
    )]);
    let mut solutions = vec![];
    while let Some((start, part_range)) = queue.pop_front() {
        let run_result = run_part_range(&workflows, part_range, start);
        for r in run_result {
            if r.0 == "A" {
                solutions.push(r.1);
            } else if r.0 == "R" {
            } else {
                queue.push_back(r);
            }
        }
    }
    solutions.into_iter().map(|p| p.combinations()).sum()
}
