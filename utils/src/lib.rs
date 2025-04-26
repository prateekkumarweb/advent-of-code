use std::{
    collections::HashMap,
    sync::{LazyLock, RwLock},
};

pub struct Solution {
    pub part_1: Option<fn(input: &str) -> String>,
    pub part_2: Option<fn(input: &str) -> String>,
}

pub static AOC_SOLUTIONS: LazyLock<RwLock<HashMap<u8, Solution>>> =
    LazyLock::new(|| RwLock::new(HashMap::new()));

pub trait Input<'a> {
    fn parse(input: &'a str) -> Self;
}

impl<'a> Input<'a> for &'a str {
    fn parse(input: &'a str) -> Self {
        input
    }
}

impl Input<'_> for String {
    fn parse(input: &str) -> Self {
        input.to_string()
    }
}

impl Input<'_> for usize {
    fn parse(input: &'_ str) -> Self {
        input.parse().unwrap()
    }
}
