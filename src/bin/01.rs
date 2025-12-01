advent_of_code::solution!(1);

fn parse(input: &str) -> Vec<&str> {
    input.lines().collect()
}

pub fn part_one(input: &str) -> Option<u64> {
    let input = parse(input);

    let mut dial: i32 = 50;
    let mut res = 0;

    for rotation in input {
        let direction = &rotation[0..1];
        let count: i32 = rotation[1..].parse().unwrap();

        if direction == "L" {
            dial = (dial - count) % 100;
        } else {
            dial = (dial + count) % 100;
        }

        if dial == 0 {
            res += 1;
        }
    }

    Some(res)
}

pub fn part_two(input: &str) -> Option<i32> {
    let input = parse(input);

    let mut dial: i32 = 50;
    let mut res = 0;

    for rotation in input {
        let direction = &rotation[0..1];
        let count: i32 = rotation[1..].parse().unwrap();

        for _ in 0..count {
            if direction == "L" {
                dial -= 1;
                if dial == 0 {
                    res += 1;
                }
                if dial < 0 {
                    dial += 100;
                }
            } else {
                dial = (dial + 1) % 100;
                if dial == 0 {
                    res += 1;
                }
            }
        }
    }

    Some(res)
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
        assert_eq!(result, Some(6));
    }
}
