use std::{
    collections::HashMap,
    sync::{LazyLock, RwLock},
};

use proc_macro::TokenStream;
use proc_macro2::Span;
use quote::quote;
use syn::{Ident, ItemFn, Token, parse::Parser, parse_macro_input, punctuated::Punctuated};

struct Solution {
    part_1: Option<String>,
    part_2: Option<String>,
}

static AOC_SOLUTIONS: LazyLock<RwLock<HashMap<u8, Solution>>> =
    LazyLock::new(|| RwLock::new(HashMap::new()));

#[proc_macro_attribute]
pub fn aoc(args: TokenStream, input: TokenStream) -> TokenStream {
    let parser = Punctuated::<Ident, Token![,]>::parse_terminated;
    let args = parser.parse(args).unwrap();
    let mut args = args.into_iter();
    let day = args.next().expect("expected day").to_string();
    let part = args.next().expect("expected part").to_string();
    let day = if day.starts_with("day") {
        let day = day[3..].parse::<u8>().unwrap();
        if !(1..=25).contains(&day) {
            panic!("Expected from day1 to day25");
        }
        day
    } else {
        panic!("Expected day");
    };
    let part = if part.starts_with("part") {
        let part = part[4..].parse::<u8>().unwrap();
        if !(1..=2).contains(&part) {
            panic!("Expected part1 or part2");
        }
        part
    } else {
        panic!("Expected part");
    };
    let input = parse_macro_input!(input as ItemFn);
    let fn_name = &input.sig.ident;
    let new_fn_name = Ident::new(&format!("aoc_day{}_part{}", day, part), Span::call_site());
    assert_eq!(input.sig.inputs.len(), 1);

    let mut lock = AOC_SOLUTIONS.write().unwrap();
    let solution = lock.entry(day).or_insert(Solution {
        part_1: None,
        part_2: None,
    });
    if part == 1 {
        solution.part_1 = Some(new_fn_name.to_string());
    } else if part == 2 {
        solution.part_2 = Some(new_fn_name.to_string());
    }

    quote! {
        pub fn #new_fn_name(input: &str) -> String {
            #input

            let input = utils::Input::parse(input);
            let output = #fn_name(input);
            output.to_string()
        }
    }
    .into()
}

#[proc_macro]
pub fn aoc_main(input: TokenStream) -> TokenStream {
    let year: u32 = input.to_string().parse().unwrap();
    let solutions = AOC_SOLUTIONS.read().unwrap();
    let mut solutions: Vec<(&u8, &Solution)> = solutions.iter().collect();
    solutions.sort_by(|a, b| a.0.cmp(b.0));
    let mut stream: proc_macro2::TokenStream = quote! {};
    for (d, sol) in solutions {
        let day = Ident::new(&format!("day{:02}", d), Span::call_site());
        let input_str = format!("../../input/{}/day{:02}.txt", year, d);
        let mut day_stream = quote! {};
        if let Some(p1) = &sol.part_1 {
            let p1 = Ident::new(p1, Span::call_site());
            day_stream = quote! {
                #day_stream
                let input = include_str!(#input_str);
                let result = crate::#day::#p1(input);
                println!("Day {:02} - Part {}: {}\n", #d, 1, result);
            }
        }
        if let Some(p2) = &sol.part_2 {
            let p2 = Ident::new(p2, Span::call_site());
            day_stream = quote! {
                #day_stream
                let input = include_str!(#input_str);
                let result = crate::#day::#p2(input);
                println!("Day {:02} - Part {}: {}\n", #d, 2, result);
            }
        }
        stream = quote! {
            #stream

            if days.contains(&#d) {
                #day_stream
            }
        };
    }
    quote! {
        fn main() {
            use utils::Parser;
            let args = utils::Args::parse();
            let days = args.day.map(|d| d..=d).unwrap_or(1..=25);

            #stream
        }
    }
    .into()
}
