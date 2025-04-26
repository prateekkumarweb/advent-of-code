use aoc::aoc;

fn compute_hash(input: &str, mut current: usize) -> usize {
    input.chars().for_each(|c| {
        let c = c as u8;
        current += c as usize;
        current *= 17;
        current %= 256;
    });
    current
}

#[aoc(day15, part1)]
pub fn part_1(input: &str) -> String {
    input
        .trim()
        .split(',')
        .map(|l| compute_hash(l, 0))
        .sum::<usize>()
        .to_string()
}

#[aoc(day15, part2)]
pub fn part_2(input: &str) -> String {
    let input = input.trim().split(',').map(|l| {
        if l.contains('-') {
            let l = &l[0..l.len() - 1];
            let hash = compute_hash(l, 0);
            (l, hash, '-', 0)
        } else {
            let mut splits = l.split('=');
            let l = splits.next().unwrap();
            let hash = compute_hash(l, 0);
            let f = splits.next().unwrap().parse::<usize>().unwrap();
            (l, hash, '=', f)
        }
    });

    let mut boxes = vec![vec![]; 256];

    for (l, hash, op, f) in input {
        let box_ = &mut boxes[hash];
        let found = box_.iter().position(|(l_, _)| *l_ == l);
        if op == '-' {
            if let Some(found) = found {
                box_.remove(found);
            }
        } else if op == '=' {
            if let Some(found) = found {
                box_[found].1 = f;
            } else {
                box_.push((l, f));
            }
        }
    }

    boxes
        .iter()
        .enumerate()
        .flat_map(|(i, box_)| {
            box_.iter()
                .enumerate()
                .map(move |(j, (_, f))| (i + 1) * (j + 1) * f)
        })
        .sum::<usize>()
        .to_string()
}
