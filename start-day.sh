#!/usr/bin/env bash
set -eo pipefail

day=$1

if [[ -z $day ]]; then
    echo "No day argument supplied" >&2
    exit 1
fi

# make sure repo is up to date
git pull

cargo aoc input -d $day -y 2015

# create solution
cat > src/day$day.rs <<EOF
use aoc_runner_derive::{aoc, aoc_generator};

//#[aoc_generator(day$day)]
//pub fn generator(input: &str) -> Vec<_> {
//    todo!()
//}

#[aoc(day$day, part1)]
pub fn part1(input: &str) -> usize {
    todo!()
}

#[aoc(day$day, part2)]
pub fn part2(input: &str) -> usize {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    static INPUT: &str = include_str!("../input/2015/day$day.txt");

    #[test]
    fn test_part1() {
        assert_eq!(part1(INPUT), 0);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(INPUT), 0);
    }
}
EOF

sed -i "s/aoc_lib! { year = 2015 }/mod day$day;\naoc_lib! { year = 2015 }/" src/lib.rs
cargo fmt

# start VS and the web page
start https://adventofcode.com/2015/day/$day
code .
