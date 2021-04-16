use aoc_runner_derive::aoc;

// let's learn some nom!
use nom::{
    branch::alt,
    character::complete::{char, none_of, one_of},
    multi::many0,
    sequence::{delimited, preceded, tuple},
    IResult,
};

#[derive(Debug, PartialEq)]
enum Character {
    Raw(char),
    SimpleEscape(char),
    HexEscape(char, char),
}

fn unescape(input: &str) -> IResult<&str, Vec<Character>> {
    delimited(char('"'), many0(parse_character), char('"'))(input)
}

fn parse_character(input: &str) -> IResult<&str, Character> {
    alt((is_raw, is_simple_escape, is_hex_escape))(input)
}

fn is_raw(input: &str) -> IResult<&str, Character> {
    let (input, c) = regular_char(input)?;

    Ok((input, Character::Raw(c)))
}

fn is_simple_escape(input: &str) -> IResult<&str, Character> {
    let (input, c) = preceded(char('\\'), one_of("\"\\"))(input)?;

    Ok((input, Character::SimpleEscape(c)))
}

fn is_hex_escape(input: &str) -> IResult<&str, Character> {
    let escape = tuple((char('\\'), char('x')));
    let digits = tuple((regular_char, regular_char));

    let (input, (first, second)) = preceded(escape, digits)(input)?;

    Ok((input, Character::HexEscape(first, second)))
}

fn regular_char(input: &str) -> IResult<&str, char> {
    none_of("\"\\")(input)
}

#[aoc(day8, part1)]
pub fn part1(input: &str) -> usize {
    let total: usize = input.lines().map(|l| l.trim().chars().count()).sum();
    let unescaped: usize = input
        .lines()
        .map(|l| unescape(l.trim()).unwrap().1.len())
        .sum();

    total - unescaped
}

#[aoc(day8, part2)]
pub fn part2(input: &str) -> usize {
    let total: usize = input.lines().map(|l| l.trim().chars().count()).sum();

    let double_escaped: usize = input
        .lines()
        .flat_map(|l| unescape(l.trim()).unwrap().1)
        .map(|c| match c {
            Character::Raw(_) => 1,
            Character::SimpleEscape(_) => 4, // transform \" -> \\\"
            Character::HexEscape(_, _) => 5, // transform \x42 -> \\x42
        })
        .sum();

    let quotes = input.lines().count() * 2 * 3; // each line starts and ends with the 3 chars "\"

    quotes + double_escaped - total
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    static INPUT: &str = include_str!("../input/2015/day8.txt");

    #[test]
    fn test_part1() {
        assert_eq!(part1(INPUT), 1333);
    }

    #[test_case(r#""""# => 4; "empty string")]
    #[test_case(r#""abc""# => 4; "simple literal string")]
    #[test_case(r#""aaa\"aaa""# => 6; "simple escape")]
    #[test_case(r#""aaa\x27aaa""# => 5; "hex escape")]
    #[test_case(r#"""
    "abc"
    "aaa\"aaa"
    "\x27""# => 19; "multi-line")]
    #[test_case(INPUT => 2046; "real input")]

    fn test_part2(input: &str) -> usize {
        part2(input)
    }

    #[test]
    fn test_is_raw() {
        assert_eq!(is_raw("a"), Ok(("", Character::Raw('a'))));
    }

    #[test]
    fn test_is_simple_escape() {
        assert_eq!(
            is_simple_escape(r"\\"),
            Ok(("", Character::SimpleEscape('\\')))
        );
        assert_eq!(
            is_simple_escape(r#"\""#),
            Ok(("", Character::SimpleEscape('"')))
        );
    }

    #[test]
    fn test_is_hex_escape() {
        assert_eq!(
            is_hex_escape(r"\x42"),
            Ok(("", Character::HexEscape('4', '2')))
        );
    }

    #[test]
    fn test_parse_character() {
        assert_eq!(parse_character("a"), Ok(("", Character::Raw('a'))));
        assert_eq!(
            parse_character(r"\\"),
            Ok(("", Character::SimpleEscape('\\')))
        );
        assert_eq!(
            parse_character(r"\x42"),
            Ok(("", Character::HexEscape('4', '2')))
        );
    }

    #[test]
    fn test_unescape() {
        assert_eq!(
            unescape(r#""a\\\"b\\\x42cd""#),
            Ok((
                "",
                vec![
                    Character::Raw('a'),
                    Character::SimpleEscape('\\'),
                    Character::SimpleEscape('"'),
                    Character::Raw('b'),
                    Character::SimpleEscape('\\'),
                    Character::HexEscape('4', '2'),
                    Character::Raw('c'),
                    Character::Raw('d'),
                ]
            ))
        )
    }
}
