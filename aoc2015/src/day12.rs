use serde_json::Value;

fn add_nums(val: &Value) -> i64 {
    match val {
        Value::Null | Value::Bool(_) | Value::String(_) => 0,
        Value::Number(number) => number.as_i64().unwrap(),
        Value::Array(values) => values.iter().map(add_nums).sum(),
        Value::Object(map) => map.values().map(add_nums).sum(),
    }
}

fn add_nums2(val: &Value) -> i64 {
    match val {
        Value::Null | Value::Bool(_) | Value::String(_) => 0,
        Value::Number(number) => number.as_i64().unwrap(),
        Value::Array(values) => values.iter().map(add_nums2).sum(),
        Value::Object(map) => {
            if map.values().any(|v| v == "red") {
                0
            } else {
                map.values().map(add_nums2).sum()
            }
        }
    }
}

#[aoc::aoc(day12, part1)]
fn part_1(input: &str) -> i64 {
    let v: Value = serde_json::from_str(input).unwrap();
    add_nums(&v)
}

#[aoc::aoc(day12, part2)]
fn part_2(input: &str) -> i64 {
    let v: Value = serde_json::from_str(input).unwrap();
    add_nums2(&v)
}
