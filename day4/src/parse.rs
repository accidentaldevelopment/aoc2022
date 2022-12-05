use std::ops::RangeInclusive;

use nom::{
    character::complete::{char, digit1, newline},
    combinator::map,
    multi::{many0, separated_list1},
    sequence::{separated_pair, terminated},
    Finish, IResult,
};

pub type SectionRange = RangeInclusive<u32>;

fn range(input: &str) -> IResult<&str, SectionRange> {
    map(
        separated_pair(digit1, char('-'), digit1),
        |(r0, r1): (&str, &str)| {
            // This unwrap is safe; we've already matched on the input being numerical.
            r0.parse::<u32>().unwrap()..=r1.parse::<u32>().unwrap()
        },
    )(input)
}

fn pair(input: &str) -> IResult<&str, [SectionRange; 2]> {
    map(separated_pair(range, char(','), range), |(r0, r1)| [r0, r1])(input)
}

pub fn parse(input: &str) -> Result<Vec<[SectionRange; 2]>, nom::error::Error<&str>> {
    let (rest, pairs) =
        terminated(separated_list1(newline, pair), many0(newline))(input).finish()?;
    // Probably should do something smarter in a real-world scenario.
    assert!(rest.is_empty());
    Ok(pairs)
}

#[cfg(test)]
mod tests {
    #[test]
    fn range() {
        assert_eq!(super::range("2-4").unwrap().1, 2..=4);
        assert_eq!(super::range("6-8").unwrap().1, 6..=8);
    }

    #[test]
    fn pair() {
        assert_eq!(super::pair("2-4,6-8").unwrap().1, [2..=4, 6..=8]);
        assert_eq!(super::pair("2-3,4-5").unwrap().1, [2..=3, 4..=5]);
    }

    #[test]
    fn parse() {
        let pairs = super::parse(
            "2-4,6-8
2-3,4-5",
        )
        .unwrap();
        assert_eq!(pairs, vec![[2..=4, 6..=8], [2..=3, 4..=5]]);

        let pairs = super::parse(
            "2-4,6-8
2-3,4-5

",
        )
        .unwrap();
        assert_eq!(pairs, vec![[2..=4, 6..=8], [2..=3, 4..=5]]);
    }
}
