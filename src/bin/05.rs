use std::ops::RangeInclusive;

use advent_of_code::util::ranges::merge_ranges;

advent_of_code::solution!(5);

fn parse(input: &str) -> (Vec<RangeInclusive<usize>>, Vec<usize>) {
    let mut split = input.trim_end_matches("\n").split("\n\n");

    let fresh: Vec<RangeInclusive<usize>> = split
        .next()
        .unwrap()
        .split("\n")
        .map(|l| {
            let mut l = l.split("-");
            l.next().unwrap().parse().unwrap()..=l.next().unwrap().parse().unwrap()
        })
        .collect();

    let ing = split
        .next()
        .unwrap()
        .split("\n")
        .map(|l| l.parse().unwrap())
        .collect();

    (fresh, ing)
}

pub fn part_one(input: &str) -> Option<usize> {
    let (fresh, ing) = parse(input);

    let fresh = merge_ranges(&fresh);

    Some(
        ing.iter()
            .filter(|i| fresh.iter().any(|f| f.contains(i)))
            .count(),
    )
}

pub fn part_two(input: &str) -> Option<usize> {
    let (fresh, _) = parse(input);

    Some(
        merge_ranges(&fresh)
            .iter()
            .fold(0, |acc, x| acc + x.clone().count()),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(14));
    }
}
