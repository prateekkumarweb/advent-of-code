use aoc_runner_derive::aoc;
use itertools::Itertools;

#[aoc(day17, part1)]
pub fn solve_part1(input: &str) -> String {
    let mut lines = input.lines();
    let mut reg_a = lines
        .next()
        .unwrap()
        .trim()
        .strip_prefix("Register A: ")
        .unwrap()
        .parse::<usize>()
        .unwrap();
    let mut reg_b = lines
        .next()
        .unwrap()
        .trim()
        .strip_prefix("Register B: ")
        .unwrap()
        .parse::<usize>()
        .unwrap();
    let mut reg_c = lines
        .next()
        .unwrap()
        .trim()
        .strip_prefix("Register C: ")
        .unwrap()
        .parse::<usize>()
        .unwrap();
    lines.next().unwrap();
    let program = lines
        .next()
        .unwrap()
        .trim()
        .strip_prefix("Program: ")
        .unwrap()
        .split(',')
        .map(|s| s.parse::<usize>().unwrap())
        .collect_vec();

    let mut pointer = 0;
    let mut result = vec![];
    while pointer < program.len() {
        let opcode = program[pointer];
        let operand = program[pointer + 1];
        let combo_operand = || match operand {
            0..=3 => operand,
            4 => reg_a,
            5 => reg_b,
            6 => reg_c,
            _ => unreachable!(),
        };
        // dbg!(pointer, reg_a, reg_b, reg_c, &result);
        match opcode {
            0 => {
                reg_a >>= combo_operand();
            }
            1 => {
                reg_b ^= operand;
            }
            2 => {
                reg_b = combo_operand() % 8;
            }
            3 => {
                if reg_a != 0 {
                    pointer = operand;
                    continue;
                }
            }
            4 => {
                reg_b ^= reg_c;
            }
            5 => {
                result.push(combo_operand() % 8);
            }
            6 => {
                reg_b = reg_a >> combo_operand();
            }
            7 => {
                reg_c = reg_a >> combo_operand();
            }
            _ => unreachable!(),
        }
        pointer += 2;
    }
    result.into_iter().map(|x| x.to_string()).join(",")
}

#[aoc(day17, part2)]
pub fn solve_part2(input: &str) -> usize {
    let mut lines = input.lines();
    lines
        .next()
        .unwrap()
        .trim()
        .strip_prefix("Register A: ")
        .unwrap()
        .parse::<usize>()
        .unwrap();
    lines
        .next()
        .unwrap()
        .trim()
        .strip_prefix("Register B: ")
        .unwrap()
        .parse::<usize>()
        .unwrap();
    lines
        .next()
        .unwrap()
        .trim()
        .strip_prefix("Register C: ")
        .unwrap()
        .parse::<usize>()
        .unwrap();
    lines.next().unwrap();
    let program = lines
        .next()
        .unwrap()
        .trim()
        .strip_prefix("Program: ")
        .unwrap()
        .split(',')
        .map(|s| s.parse::<usize>().unwrap())
        .collect_vec();

    // let mut code_a = String::from("A");
    // let mut code_b = String::from("B");
    // let mut code_c = String::from("C");
    // let mut out = String::new();

    // for (opcode, operand) in program.iter().tuples() {
    //     match *opcode {
    //         0 => {
    //             let combo_operand = match operand {
    //                 0..=3 => operand.to_string(),
    //                 4 => code_a.to_string(),
    //                 5 => code_b.to_string(),
    //                 6 => code_c.to_string(),
    //                 _ => unreachable!(),
    //             };
    //             code_a = format!("({}) >> ({})", code_a, combo_operand);
    //         }
    //         1 => {
    //             code_b = format!("({}) ^ {}", code_b, operand);
    //         }
    //         2 => {
    //             let combo_operand = match operand {
    //                 0..=3 => operand.to_string(),
    //                 4 => code_a.to_string(),
    //                 5 => code_b.to_string(),
    //                 6 => code_c.to_string(),
    //                 _ => unreachable!(),
    //             };
    //             code_b = format!("({}) % 8", combo_operand);
    //         }
    //         3 => {}
    //         4 => {
    //             code_b = format!("({}) ^ ({})", code_b, code_c);
    //         }
    //         5 => {
    //             let combo_operand = match operand {
    //                 0..=3 => operand.to_string(),
    //                 4 => code_a.to_string(),
    //                 5 => code_b.to_string(),
    //                 6 => code_c.to_string(),
    //                 _ => unreachable!(),
    //             };
    //             out = format!("print ({}) % 8", combo_operand);
    //         }
    //         6 => {
    //             let combo_operand = match operand {
    //                 0..=3 => operand.to_string(),
    //                 4 => code_a.to_string(),
    //                 5 => code_b.to_string(),
    //                 6 => code_c.to_string(),
    //                 _ => unreachable!(),
    //             };
    //             code_b = format!("({}) >> ({})", code_a, combo_operand);
    //         }
    //         7 => {
    //             let combo_operand = match operand {
    //                 0..=3 => operand.to_string(),
    //                 4 => code_a.to_string(),
    //                 5 => code_b.to_string(),
    //                 6 => code_c.to_string(),
    //                 _ => unreachable!(),
    //             };
    //             code_c = format!("({}) >> ({})", code_a, combo_operand);
    //         }
    //         _ => unreachable!(),
    //     }
    // }

    // println!("Code A: {}", code_a);
    // println!("Code B: {}", code_b);
    // println!("Code C: {}", code_c);
    // println!("Output: {}", out);

    // Using above print statements, we can see that the program is equivalent to the following:
    // Code A: (A) >> (3)
    // Code B: ((((A) % 8) ^ 1) ^ ((A) >> (((A) % 8) ^ 1))) ^ 4
    // Code C: (A) >> (((A) % 8) ^ 1)
    // Output: print (((((A) % 8) ^ 1) ^ ((A) >> (((A) % 8) ^ 1))) ^ 4) % 8
    //
    // In every loop B and C are reassigned, so no use of them keeping track of them.
    // Also after every loop, A becomes A/8 (right shift by 3)
    // We search from backwards

    let mut result = program.clone();
    let mut found_a_all = vec![0];
    while let Some(result) = result.pop() {
        let mut new_fount_a_all = vec![];
        for found_a in &found_a_all {
            let mut possibilities = vec![];
            'outer: for i in 0..8 {
                let mut reg_a = (found_a << 3) + i;
                let mut reg_b = 0;
                let mut reg_c = 0;
                let mut pointer = 0;
                while pointer < program.len() {
                    let opcode = program[pointer];
                    let operand = program[pointer + 1];
                    let combo_operand = || match operand {
                        0..=3 => operand,
                        4 => reg_a,
                        5 => reg_b,
                        6 => reg_c,
                        _ => unreachable!(),
                    };
                    match opcode {
                        0 => {
                            reg_a >>= combo_operand();
                        }
                        1 => {
                            reg_b ^= operand;
                        }
                        2 => {
                            reg_b = combo_operand() % 8;
                        }
                        3 => {}
                        4 => {
                            reg_b ^= reg_c;
                        }
                        5 => {
                            let value = combo_operand() % 8;
                            if result != value {
                                continue 'outer;
                            }
                        }
                        6 => {
                            reg_b = reg_a >> combo_operand();
                        }
                        7 => {
                            reg_c = reg_a >> combo_operand();
                        }
                        _ => unreachable!(),
                    }
                    pointer += 2;
                }
                possibilities.push((found_a << 3) + i);
            }
            new_fount_a_all.extend(possibilities);
        }
        found_a_all = new_fount_a_all;
    }

    found_a_all.into_iter().min().unwrap()
}
