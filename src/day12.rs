use aoc_runner_derive::{aoc, aoc_generator};
use json::JsonValue;

#[aoc_generator(day12)]
pub fn generator(input: &str) -> JsonValue {
    json::parse(input).unwrap()
}

#[aoc(day12, part1)]
pub fn part1(json: &JsonValue) -> i32 {
    add_children(json, false)
}

#[aoc(day12, part2)]
pub fn part2(json: &JsonValue) -> i32 {
    add_children(json, true)
}

fn add_children(json: &JsonValue, skip_red: bool) -> i32 {
    match json {
        JsonValue::Null => 0,
        JsonValue::Boolean(_) => 0,
        JsonValue::Short(x) => x.parse().unwrap_or_default(),
        JsonValue::String(x) => x.parse().unwrap_or_default(),
        JsonValue::Number(x) => f32::from(*x) as i32,
        JsonValue::Array(values) => values.iter().map(|v| add_children(v, skip_red)).sum(),
        JsonValue::Object(values) => {
            if skip_red && values.iter().any(|(_, value)| *value == "red") {
                0
            } else {
                values
                    .iter()
                    .map(|(_, value)| add_children(value, skip_red))
                    .sum()
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    static INPUT: &str = include_str!("../input/2015/day12.txt");

    #[test]
    fn test_part1() {
        let json = generator(INPUT);
        assert_eq!(part1(&json), 111754);
    }

    #[test]
    fn test_part2() {
        let json = generator(INPUT);
        assert_eq!(part2(&json), 65402);
    }
}
