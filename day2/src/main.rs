use std::{cmp::Ordering, convert::Infallible, fmt::Debug, str::FromStr};

const INPUT: &str = include_str!("../../input/day2.txt");

#[derive(PartialEq, Eq)]
#[cfg_attr(test, derive(Debug))]
enum Shape {
    Rock,
    Paper,
    Scissors,
}

#[derive(PartialEq, Eq)]
#[cfg_attr(test, derive(Debug))]
enum Outcome {
    Win,
    Lose,
    Draw,
}

trait Round {
    fn outcome(&self) -> u64;
}

#[cfg_attr(test, derive(Debug, PartialEq, Eq))]
struct WrongRound {
    them: Shape,
    us: Shape,
}

#[cfg_attr(test, derive(Debug, PartialEq, Eq))]
struct CorrectRound {
    them: Shape,
    outcome: Outcome,
}

pub fn main() {
    println!("part1: {}", part1(&input_generator::<WrongRound>(INPUT)));
    println!("part2: {}", part2(&input_generator::<CorrectRound>(INPUT)));
}

fn input_generator<R>(input: &str) -> Vec<R>
where
    R: FromStr + Round,
    <R as FromStr>::Err: Debug,
{
    input.lines().map(|l| l.parse::<R>().unwrap()).collect()
}

fn part1(input: &[WrongRound]) -> u64 {
    input.iter().fold(0, |score, round| score + round.outcome())
}

fn part2(input: &[CorrectRound]) -> u64 {
    input.iter().fold(0, |score, round| score + round.outcome())
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

// Bunch of FromStr impls. `Err`  associate types are marked `Infallible`, which is not literally
// true. But we're going to violently assume AoC is always giving us valid input.

impl FromStr for Outcome {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "X" => Ok(Self::Lose),
            "Y" => Ok(Self::Draw),
            "Z" => Ok(Self::Win),
            _ => unreachable!("puzzle input is invalid"),
        }
    }
}

impl FromStr for WrongRound {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut tokens = s.split(' ');
        let them = tokens.next().unwrap().parse::<Shape>().unwrap();
        let us = tokens.next().unwrap().parse::<Shape>().unwrap();
        Ok(Self { them, us })
    }
}

impl FromStr for CorrectRound {
    type Err = Infallible;

    fn from_str(line: &str) -> Result<Self, Self::Err> {
        let mut tokens = line.split(' ');
        let them = tokens.next().unwrap().parse::<Shape>().unwrap();
        let outcome = tokens.next().unwrap().parse::<Outcome>().unwrap();
        Ok(Self { them, outcome })
    }
}

impl Round for WrongRound {
    fn outcome(&self) -> u64 {
        self.us.points()
            + match self.us.cmp(&self.them) {
                Ordering::Less => 0,
                Ordering::Equal => 3,
                Ordering::Greater => 6,
            }
    }
}

impl Round for CorrectRound {
    fn outcome(&self) -> u64 {
        match self.outcome {
            Outcome::Win => 6 + self.them.loses_to().points(),
            Outcome::Draw => 3 + self.them.points(),
            Outcome::Lose => self.them.defeats().points(),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{CorrectRound, Outcome};

    use super::{Shape, WrongRound};

    const WR_DATA: [WrongRound; 3] = [
        WrongRound {
            them: Shape::Rock,
            us: Shape::Paper,
        },
        WrongRound {
            them: Shape::Paper,
            us: Shape::Rock,
        },
        WrongRound {
            them: Shape::Scissors,
            us: Shape::Scissors,
        },
    ];

    const CORRECT_DATA: [CorrectRound; 3] = [
        CorrectRound {
            them: Shape::Rock,
            outcome: Outcome::Draw,
        },
        CorrectRound {
            them: Shape::Paper,
            outcome: Outcome::Lose,
        },
        CorrectRound {
            them: Shape::Scissors,
            outcome: Outcome::Win,
        },
    ];

    #[test]
    fn generator() {
        let got = super::input_generator::<WrongRound>(
            r"A Y
B X
C Z",
        );
        assert_eq!(got, WR_DATA);

        let got = super::input_generator::<CorrectRound>(
            r"A Y
B X
C Z",
        );
        assert_eq!(got, CORRECT_DATA);
    }

    #[test]
    fn part1() {
        assert_eq!(super::part1(&WR_DATA), 15);
    }

    #[test]
    fn part2() {
        assert_eq!(super::part2(&CORRECT_DATA), 12);
    }

    mod outcomes {
        use crate::Shape;

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
