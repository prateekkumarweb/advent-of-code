use std::collections::HashMap;

use aoc::aoc;

enum BaseNode<'a> {
    Val(u16),
    Var(&'a str),
}

impl BaseNode<'_> {
    fn eval(&self, assign: &HashMap<&str, u16>) -> Option<u16> {
        match self {
            BaseNode::Val(x) => Some(*x),
            BaseNode::Var(x) => assign.get(*x).copied(),
        }
    }
}

enum Node<'a> {
    Base(BaseNode<'a>),
    And(BaseNode<'a>, BaseNode<'a>),
    Or(BaseNode<'a>, BaseNode<'a>),
    LShift(BaseNode<'a>, BaseNode<'a>),
    RShift(BaseNode<'a>, BaseNode<'a>),
    Not(BaseNode<'a>),
}

impl Node<'_> {
    fn eval(&self, assign: &HashMap<&str, u16>) -> Option<u16> {
        match self {
            Node::Base(a) => a.eval(assign),
            Node::And(a, b) => a.eval(assign).and_then(|a| b.eval(assign).map(|b| a & b)),
            Node::Or(a, b) => a.eval(assign).and_then(|a| b.eval(assign).map(|b| a | b)),
            Node::LShift(a, b) => a.eval(assign).and_then(|a| b.eval(assign).map(|b| a << b)),
            Node::RShift(a, b) => a.eval(assign).and_then(|a| b.eval(assign).map(|b| a >> b)),
            Node::Not(a) => a.eval(assign).map(|a| !a),
        }
    }
}

fn bi_op<'a>(
    inp: &'a str,
    out: &'a str,
    op_str: &'static str,
    circuits: &mut HashMap<&'a str, Node<'a>>,
) -> bool {
    if inp.contains(op_str) {
        let (a, b) = inp.split_once(op_str).unwrap();
        let a_node = a.parse().map_or_else(|_| BaseNode::Var(a), BaseNode::Val);
        let b_node = b.parse().map_or_else(|_| BaseNode::Var(b), BaseNode::Val);
        circuits.insert(
            out,
            match op_str {
                " AND " => Node::And(a_node, b_node),
                " OR " => Node::Or(a_node, b_node),
                " LSHIFT " => Node::LShift(a_node, b_node),
                " RSHIFT " => Node::RShift(a_node, b_node),
                _ => unreachable!(),
            },
        );
        true
    } else {
        false
    }
}

fn un_op<'a>(
    inp: &'a str,
    out: &'a str,
    op_str: &'static str,
    circuits: &mut HashMap<&'a str, Node<'a>>,
) -> bool {
    if inp.contains(op_str) {
        let a = inp.strip_prefix(op_str).unwrap();
        let a_node = a.parse().map_or_else(|_| BaseNode::Var(a), BaseNode::Val);
        circuits.insert(out, Node::Not(a_node));
        true
    } else {
        false
    }
}

#[aoc(day7, part1)]
pub fn part_1(input: &str) -> u16 {
    let mut circuits = HashMap::new();
    let mut assign = HashMap::new();
    for line in input.lines() {
        let (inp, out) = line.split_once(" -> ").unwrap();
        let done = bi_op(inp, out, " AND ", &mut circuits)
            || bi_op(inp, out, " OR ", &mut circuits)
            || bi_op(inp, out, " LSHIFT ", &mut circuits)
            || bi_op(inp, out, " RSHIFT ", &mut circuits)
            || un_op(inp, out, "NOT ", &mut circuits);
        if !done {
            if let Ok(inp) = inp.parse::<u16>() {
                circuits.insert(out, Node::Base(BaseNode::Val(inp)));
                assign.insert(out, inp);
            } else {
                circuits.insert(out, Node::Base(BaseNode::Var(inp)));
            }
        }
    }
    loop {
        for (&x, node) in &circuits {
            if !assign.contains_key(x) {
                if let Some(val) = node.eval(&assign) {
                    assign.insert(x, val);
                }
            }
        }
        if assign.len() == circuits.len() {
            break;
        }
    }
    assign.get("a").copied().unwrap()
}

#[aoc(day7, part2)]
pub fn part_2(input: &str) -> u16 {
    let mut circuits = HashMap::new();
    let mut assign = HashMap::new();
    for line in input.lines() {
        let (inp, out) = line.split_once(" -> ").unwrap();
        let done = bi_op(inp, out, " AND ", &mut circuits)
            || bi_op(inp, out, " OR ", &mut circuits)
            || bi_op(inp, out, " LSHIFT ", &mut circuits)
            || bi_op(inp, out, " RSHIFT ", &mut circuits)
            || un_op(inp, out, "NOT ", &mut circuits);
        if !done {
            if let Ok(inp) = inp.parse::<u16>() {
                circuits.insert(out, Node::Base(BaseNode::Val(inp)));
                assign.insert(out, inp);
            } else {
                circuits.insert(out, Node::Base(BaseNode::Var(inp)));
            }
        }
    }
    loop {
        for (&x, node) in &circuits {
            if !assign.contains_key(x) {
                if let Some(val) = node.eval(&assign) {
                    assign.insert(x, val);
                }
            }
        }
        if assign.len() == circuits.len() {
            break;
        }
    }
    let b = assign.get("a").copied().unwrap();
    assign.clear();
    assign.insert("b", b);
    loop {
        for (&x, node) in &circuits {
            if !assign.contains_key(x) {
                if let Some(val) = node.eval(&assign) {
                    assign.insert(x, val);
                }
            }
        }
        if assign.len() == circuits.len() {
            break;
        }
    }
    assign.get("a").copied().unwrap()
}
