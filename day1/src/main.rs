const INPUT: &str = include_str!("../../input/day1.txt");

pub fn main() {
    let input = input_generator(INPUT);

    println!("part1: {}", part1(&input));
    println!("part2: {}", part2(&input));
}

pub fn input_generator(input: &str) -> Vec<usize> {
    let mut calorie_counts = vec![];
    let mut current_cals: usize = 0;

    // There's probably a nice functional way to do this, but oh well.
    for line in input.lines() {
        if line.is_empty() {
            calorie_counts.push(current_cals);
            current_cals = 0;
        } else {
            current_cals += line.parse::<usize>().unwrap();
        }
    }
    calorie_counts.push(current_cals);

    calorie_counts
}

fn part1(input: &[usize]) -> usize {
    input.iter().max().unwrap().to_owned()
}

fn part2(input: &[usize]) -> usize {
    // Would be nice to do this without allocating a new vec.
    let mut input = input.to_vec();
    input.sort_unstable_by(|a, b| b.cmp(a));
    input.iter().take(3).sum()
}

#[cfg(test)]
mod tests {
    const IN_DATA: [usize; 5] = [6000, 4000, 11000, 24000, 10000];

    #[test]
    fn generator() {
        let got = super::input_generator(
            r"1000
2000
3000

4000

5000
6000

7000
8000
9000

10000",
        );
        assert_eq!(got, IN_DATA);
    }

    #[test]
    fn part1() {
        assert_eq!(super::part1(&IN_DATA), 24000);
    }

    #[test]
    fn part2() {
        assert_eq!(super::part2(&IN_DATA), 45000);
    }
}
