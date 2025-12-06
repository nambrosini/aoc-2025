advent_of_code::solution!(6);

#[derive(Default, Clone)]
struct Problem {
    numbers: Vec<i64>,
    operator: String
}

impl Problem {
    fn solve(&self) -> i64 {
        let op: &str = &self.operator;
        match op {
            "*" => self.numbers.iter().product(),
            "+" => self.numbers.iter().sum(),
            _ => unreachable!()
        }
    }
}

fn parse(input: &str) -> Vec<Problem> {
    let mut lines: Vec<&str> = input.trim_end_matches("\n").lines().collect();
    let operations: Vec<&str> = lines.last().unwrap().split(" ").filter(|x| !x.is_empty()).collect();

    let mut problems = vec![Problem::default(); operations.len()];
    operations.iter().enumerate().for_each(|(i, o)| problems[i].operator = o.to_string());

    for l in &lines[..lines.len()-1] {
        let split: Vec<&str> = l.split(" ").filter(|x| !x.is_empty()).collect();
        l.split(" ").filter(|x| !x.is_empty()).map(|x| x.trim().parse().unwrap()).enumerate().for_each(|(i, x)| problems[i].numbers.push(x));
    }

    problems
}

pub fn part_one(input: &str) -> Option<i64> {
    let input = parse(input);

    Some(input.iter().map(|p| p.solve()).sum())
}

pub fn part_two(input: &str) -> Option<u64> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4277556));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
