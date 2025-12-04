use std::fmt::Display;

#[derive(Debug, Clone)]
pub struct Grid<T> {
    pub grid: Vec<Vec<T>>,
}

impl<T> Grid<T>
where
    T: From<char> + Default + Clone + PartialEq,
{
    pub fn from_size(width: usize, height: usize, default: T) -> Grid<T> {
        let grid = vec![vec![default; width]; height];
        Self { grid }
    }

    pub fn parse(input: &str) -> Self {
        let grid = input
            .lines()
            .map(|l| l.chars().map(|c| c.into()).collect())
            .collect();

        Self { grid }
    }

    pub fn count_neighbours(&self, cell: T) -> Grid<u32> {
        let mut grid = Grid::from_size(self.grid[0].len(), self.grid.len(), 9);
        for x in 0..self.grid.len() {
            for y in 0..self.grid[x].len() {
                if self.grid[x][y] != cell {
                    continue;
                }
                grid.grid[x][y] = self.neighbours_for_cell(x, y, &cell);
            }
        }
        grid
    }

    fn neighbours_for_cell(&self, x: usize, y: usize, target: &T) -> u32 {
        let mut count = 0;
        let rows = self.grid.len();
        let cols = self.grid[0].len();

        let x_min = x.saturating_sub(1);
        let x_max = (x + 1).min(rows - 1);
        let y_min = y.saturating_sub(1);
        let y_max = (y + 1).min(cols - 1);

        for i in x_min..=x_max {
            for j in y_min..=y_max {
                if i == x && j == y {
                    continue;
                }

                if self.grid[i][j] == *target {
                    count += 1;
                }
            }
        }
        count
    }
}

impl<T> Display for Grid<T>
where
    T: Display,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in &self.grid {
            for cell in row {
                write!(f, "{}", cell)?;
            }
            writeln!(f)?;
        }
        writeln!(f)
    }
}
