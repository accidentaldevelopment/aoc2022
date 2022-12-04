#![deny(clippy::all, clippy::pedantic, rust_2018_idioms)]

use std::collections::HashSet;

mod parse;

const INPUT: &str = include_str!("../../input/day4.txt");

pub fn main() {
    println!("part1: {}", part1(INPUT));
    println!("part2: {}", part2(INPUT));
}

fn part1(input: &str) -> usize {
    input.lines().fold(0, |acc, line| {
        let (_, [p0, p1]) = parse::pair(line).unwrap();
        // This could be done a with a set, but this seemed simpler.
        if (p1.start() >= p0.start() && p1.end() <= p0.end())
            || (p0.start() >= p1.start() && p0.end() <= p1.end())
        {
            acc + 1
        } else {
            acc
        }
    })
}

fn part2(input: &str) -> usize {
    input.lines().fold(0, |acc, line| {
        let (_, [p0, p1]) = parse::pair(line).unwrap();
        let s0 = p0.collect::<HashSet<_>>();
        let s1 = p1.collect::<HashSet<_>>();

        if s0.is_disjoint(&s1) {
            acc
        } else {
            acc + 1
        }
    })
}

#[cfg(test)]
mod tests {
    const INPUT: &str = "2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8";

    #[test]
    fn part1() {
        assert_eq!(super::part1(INPUT), 2);
    }

    #[test]
    fn part2() {
        assert_eq!(super::part2(INPUT), 4);
    }
}
