advent_of_code::solution!(3);

type Bank = Vec<u32>;

fn parse(input: &str) -> Vec<Bank> {
    input
        .lines()
        .map(|l| l.chars().map(|b| b.to_digit(10).unwrap()).collect())
        .collect()
}

pub fn part_one(input: &str) -> Option<u32> {
    let banks = parse(input);

    let mut res = 0;

    for bank in banks {
        let mut max = (0, 0);
        for (i, &b) in bank.iter().enumerate() {
            if i == bank.len() - 1 {
                if b > max.1 {
                    max.1 = b;
                }
            } else if b > max.0 {
                max.0 = b;
                max.1 = 0;
            } else if b > max.1 {
                max.1 = b;
            }
        }

        res += max.0 * 10 + max.1;
    }

    Some(res)
}

pub fn part_two(input: &str) -> Option<u64> {
    let banks = input.lines();
    let mut sum = 0;

    for b in banks {
        if b.is_empty() {
            continue;
        }
        let mut removals = b.len() - 12;
        let mut stack: Vec<char> = Vec::new();

        for battery in b.chars() {
            // Greedy logic: If the current digit is greater than the last one on the stack,
            // and we still have removals available, pop the smaller digit.
            while removals > 0 {
                if let Some(&last) = stack.last()
                    && last < battery
                {
                    stack.pop();
                    removals -= 1;
                    continue; // Check the new top of stack against current 'c'
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

    Some(sum)
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
