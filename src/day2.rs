use std::{cmp::Ordering, str::FromStr};

use aoc_runner_derive::{aoc, aoc_generator};

#[derive(PartialEq, Eq)]
#[cfg_attr(test, derive(Debug))]
pub enum Shape {
    Rock,
    Paper,
    Scissors,
}

#[cfg_attr(test, derive(Debug, PartialEq, Eq))]
pub struct Round {
    them: Shape,
    us: Shape,
}

#[aoc_generator(day2)]
pub fn input_generator(input: &str) -> Vec<Round> {
    input
        .lines()
        .map(|l| {
            let mut tokens = l.split(' ');
            let us = tokens.next().unwrap().parse::<Shape>().unwrap();
            let them = tokens.next().unwrap().parse::<Shape>().unwrap();
            Round::from((us, them))
        })
        .collect()
}

#[aoc(day2, part1)]
fn part1(input: &[Round]) -> u64 {
    input.iter().fold(0, |score, round| score + round.outcome())
}

#[aoc(day2, part2)]
fn part2(input: &[Round]) -> u64 {
    input.iter().fold(0, |score, round| {
        score
            + match round.us {
                Shape::Rock => round.them.defeats().points(),
                Shape::Paper => round.them.points() + 3,
                Shape::Scissors => round.them.loses_to().points() + 6,
            }
    })
}

impl FromStr for Shape {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "A" | "X" => Ok(Self::Rock),
            "B" | "Y" => Ok(Self::Paper),
            "C" | "Z" => Ok(Self::Scissors),
            _ => unreachable!("puzzle input is invalid"),
        }
    }
}

impl PartialOrd for Shape {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        if self == other {
            return Some(Ordering::Equal);
        }

        Some(match (self, other) {
            (Self::Rock, Self::Scissors)
            | (Self::Scissors, Self::Paper)
            | (Self::Paper, Self::Rock) => Ordering::Greater,
            _ => Ordering::Less,
        })
    }
}

impl Ord for Shape {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}

impl Shape {
    fn points(&self) -> u64 {
        match self {
            Shape::Rock => 1,
            Shape::Paper => 2,
            Shape::Scissors => 3,
        }
    }

    fn defeats(&self) -> Self {
        match self {
            Shape::Rock => Self::Scissors,
            Shape::Paper => Self::Rock,
            Shape::Scissors => Self::Paper,
        }
    }

    fn loses_to(&self) -> Self {
        match self {
            Shape::Rock => Self::Paper,
            Shape::Paper => Self::Scissors,
            Shape::Scissors => Self::Rock,
        }
    }
}

impl From<(Shape, Shape)> for Round {
    fn from((them, us): (Shape, Shape)) -> Self {
        Self { them, us }
    }
}

impl Round {
    fn outcome(&self) -> u64 {
        self.us.points()
            + match self.us.cmp(&self.them) {
                Ordering::Less => 0,
                Ordering::Equal => 3,
                Ordering::Greater => 6,
            }
    }
}

#[cfg(test)]
mod tests {
    use super::{Round, Shape};

    const IN_DATA: [Round; 3] = [
        Round {
            them: Shape::Rock,
            us: Shape::Paper,
        },
        Round {
            them: Shape::Paper,
            us: Shape::Rock,
        },
        Round {
            them: Shape::Scissors,
            us: Shape::Scissors,
        },
    ];

    #[test]
    fn generator() {
        let got = super::input_generator(
            r"A Y
B X
C Z",
        );
        assert_eq!(got, IN_DATA);
    }

    #[test]
    fn part1() {
        assert_eq!(super::part1(&IN_DATA), 15);
    }

    #[test]
    fn part2() {
        assert_eq!(super::part2(&IN_DATA), 12);
    }

    mod outcomes {
        use crate::day2::Shape;

        #[test]
        fn rock_beats_scissors() {
            assert!(Shape::Rock > Shape::Scissors);
        }

        #[test]
        fn scissors_beats_papper() {
            assert!(Shape::Scissors > Shape::Paper);
        }

        #[test]
        fn paper_beats_rock() {
            assert!(Shape::Paper > Shape::Rock);
        }
    }
}
