#![deny(clippy::all, clippy::pedantic, rust_2018_idioms)]

use std::collections::HashSet;

const INPUT: &str = include_str!("../../input/day3.txt");

pub fn main() {
    // let input = input_generator(INPUT);

    println!("part1: {}", part1(INPUT));
    println!("part2: {}", part2(INPUT));
}

fn part1(input: &str) -> usize {
    input
        .lines()
        .map(|line| {
            let (s0, s1) = line.split_at(line.len() / 2);
            let h0 = s0.bytes().collect::<HashSet<_>>();
            let h1 = s1.bytes().collect::<HashSet<_>>();
            priority(*h0.intersection(&h1).next().unwrap())
        })
        .sum()
}

fn part2(input: &str) -> usize {
    input
        .lines()
        .collect::<Vec<&str>>()
        .chunks_exact(3)
        .map(|group| {
            let h0 = group[0].bytes().collect::<HashSet<_>>();
            let h1 = group[1].bytes().collect::<HashSet<_>>();
            let h2 = group[2].bytes().collect::<HashSet<_>>();
            priority(*h0.intersection(&h1).find(|i| h2.contains(i)).unwrap())
        })
        .sum()
}

fn priority(i: u8) -> usize {
    match i {
        b'a'..=b'z' => usize::from(i - b'a' + 1),
        b'A'..=b'Z' => usize::from(i - b'A' + 27),
        _ => unimplemented!("input must be a letter"),
    }
}

#[cfg(test)]
mod tests {
    const INPUT: &str = r"vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw";

    #[test]
    fn part1() {
        assert_eq!(super::part1(INPUT), 157);
    }

    #[test]
    fn part2() {
        assert_eq!(super::part2(INPUT), 70);
    }
}
