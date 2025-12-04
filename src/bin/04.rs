use std::fmt::{Debug, Display};

use advent_of_code::util::grid::Grid;

advent_of_code::solution!(4);

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
enum Cell {
    #[default]
    Empty,
    PaperRoll,
}

impl From<char> for Cell {
    fn from(value: char) -> Self {
        match value {
            '.' => Cell::Empty,
            '@' => Cell::PaperRoll,
            _ => unreachable!(),
        }
    }
}

impl Display for Cell {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Cell::Empty => write!(f, "@"),
            Cell::PaperRoll => write!(f, "."),
        }
    }
}

pub fn part_one(input: &str) -> Option<usize> {
    let grid = Grid::<Cell>::parse(input);

    let neighbours_count = grid.count_neighbours(Cell::PaperRoll);

    Some(
        neighbours_count
            .grid
            .iter()
            .flatten()
            .filter(|c| c < &&4)
            .count(),
    )
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut grid = Grid::<Cell>::parse(input);
    let mut neighbours_count = grid.count_neighbours(Cell::PaperRoll);
    let mut removed = 0;

    while neighbours_count
        .grid
        .iter()
        .flatten()
        .filter(|c| c < &&4)
        .count()
        != 0
    {
        let mut grid_clone = grid.clone();
        for (i, row) in neighbours_count.grid.iter().enumerate() {
            for (j, cell) in row.iter().enumerate() {
                if cell < &4 {
                    grid_clone.grid[i][j] = Cell::Empty;
                    removed += 1;
                }
            }
        }
        grid = grid_clone;
        neighbours_count = grid.count_neighbours(Cell::PaperRoll);
    }

    Some(removed)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(13));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(43));
    }
}
