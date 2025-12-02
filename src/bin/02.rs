use std::ops::RangeInclusive;

use fancy_regex::Regex;

advent_of_code::solution!(2);

fn parse(input: &str) -> Vec<RangeInclusive<u64>> {
    input
        .lines()
        .flat_map(|l| l.split(','))
        .filter_map(|range_str| {
            let (start, end) = range_str.split_once('-')?;
            Some(start.parse().ok()?..=end.parse().ok()?)
        })
        .collect()
}

pub fn part_one(input: &str) -> Option<u64> {
    let re = Regex::new(r"^(\d+)\1$").unwrap();
    let input = parse(input);
    let mut sum = 0;
    for r in input {
        for id in r {
            if re.is_match(&id.to_string()).unwrap() {
                sum += id;
            }
        }
    }

    Some(sum)
}

pub fn part_two(input: &str) -> Option<u64> {
    let re = Regex::new(r"^(\d+)\1+$").unwrap();
    let input = parse(input);
    let mut sum = 0;
    for r in input {
        for id in r {
            if re.is_match(&id.to_string()).unwrap() {
                sum += id;
            }
        }
    }

    Some(sum)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1227775554));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4174379265));
    }
}
