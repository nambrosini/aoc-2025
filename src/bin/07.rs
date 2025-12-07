use std::collections::{HashSet, VecDeque};

use advent_of_code::util::{direction::Direction, grid::Grid, position::Vec2};

advent_of_code::solution!(7);

const DIR: Direction = Direction::Down;

pub fn part_one(input: &str) -> Option<u64> {
    let grid: Grid<char> = Grid::parse(input);
    let start = grid.find(&'S').unwrap();

    let mut split_set = HashSet::new();

    let mut queue = VecDeque::new();
    queue.push_back(start);

    while let Some(beam) = queue.pop_front() {
        let new_beam = beam + DIR.into();

        if !grid.is_inbound(new_beam) {
            continue;
        }

        if grid.get(new_beam).unwrap() == '^' {
            split_set.insert(new_beam);
            let new_beam_left = new_beam + Direction::Left.into();
            if grid.is_inbound(new_beam_left) && !queue.contains(&new_beam_left) {
                queue.push_back(new_beam_left);
            }
            let new_beam_right = new_beam + Direction::Right.into();
            if grid.is_inbound(new_beam_right) && !queue.contains(&new_beam_right) {
                queue.push_back(new_beam_right);
            }
        } else {
            queue.push_back(new_beam);
        }
    }

    Some(split_set.len() as u64)
}

pub fn part_two(input: &str) -> Option<u64> {
    let grid: Grid<char> = Grid::parse(input);
    let start = grid.find(&'S').unwrap();

    let mut count = 1;

    let mut queue: VecDeque<(Vec2, u64)> = VecDeque::new();
    queue.push_back((start, 1));

    while let Some(beam) = queue.pop_front() {
        let new_beam = beam.0 + DIR.into();

        if !grid.is_inbound(new_beam) {
            continue;
        }

        if grid.get(new_beam).unwrap() == '^' {
            count += beam.1;
            let new_beam_left = new_beam + Direction::Left.into();
            if grid.is_inbound(new_beam_left) {
                if let Some(pos) = queue.iter().position(|b| b.0 == new_beam_left) {
                    queue[pos].1 += beam.1;
                } else {
                    queue.push_back((new_beam_left, beam.1));
                }
            }
            let new_beam_right = new_beam + Direction::Right.into();
            if grid.is_inbound(new_beam_right) {
                if let Some(pos) = queue.iter().position(|b| b.0 == new_beam_right) {
                    queue[pos].1 += beam.1;
                } else {
                    queue.push_back((new_beam_right, beam.1));
                }
            }
        } else if let Some(pos) = queue.iter().position(|b| b.0 == new_beam) {
            queue[pos].1 += beam.1;
        } else {
            queue.push_back((new_beam, beam.1));
        }
    }

    Some(count)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(21));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(40));
    }
}
