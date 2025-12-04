advent_of_code::solution!(3);

pub fn part_one(input: &str) -> Option<u64> {
    let banks: Vec<&str> = input.lines().collect();

    Some(max_joltage(&banks, 2))
}

pub fn part_two(input: &str) -> Option<u64> {
    let banks: Vec<&str> = input.lines().collect();

    Some(max_joltage(&banks, 12))
}

fn max_joltage(banks: &[&str], digits: usize) -> u64 {
    let mut sum = 0;

    for b in banks {
        if b.is_empty() {
            continue;
        }
        let mut removals = b.len() - digits;
        let mut stack: Vec<char> = Vec::new();

        for battery in b.chars() {
            while removals > 0 {
                if let Some(&last) = stack.last()
                    && last < battery
                {
                    stack.pop();
                    removals -= 1;
                    continue;
                }
                break;
            }
            stack.push(battery);
        }
        while removals > 0 {
            stack.pop();
            removals -= 1;
        }

        let res: String = stack.iter().collect();
        sum += res.parse::<u64>().unwrap();
    }

    sum
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(357));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3121910778619));
    }
}
