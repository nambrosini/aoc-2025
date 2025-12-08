use advent_of_code::util::vec3::Vec3;

advent_of_code::solution!(8);

type Input = Vec<Vec3>;

const MAX: f64 = 180_000_000f64;

fn parse(input: &str) -> Input {
    input.lines().map(|line| line.parse().unwrap()).collect()
}

#[derive(Debug)]
struct Connection {
    a: usize,
    b: usize,
    distance: f64,
}

impl Connection {
    fn new(a: usize, b: usize, distance: f64) -> Self {
        Self { a, b, distance }
    }
}

impl PartialEq for Connection {
    fn eq(&self, other: &Self) -> bool {
        self.a == other.a && self.b == other.b
    }
}

impl Eq for Connection {}

impl PartialOrd for Connection {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Connection {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.distance.total_cmp(&other.distance)
    }
}

pub fn part_one(input: &str) -> Option<usize> {
    let points = parse(input);

    Some(build_circuit(&points, 1000))
}

fn build_circuit(points: &[Vec3], n: usize) -> usize {
    let mut edges = Vec::with_capacity(points.len() * points.len() / 2);
    for i in 0..points.len() - 1 {
        for j in i + 1..points.len() {
            let dist = points[i].euclidean_distance(&points[j]);
            if dist < MAX {
                edges.push(Connection::new(i, j, dist));
            }
        }
    }
    edges.sort();

    let mut dsu = aph_disjoint_set::DisjointSetArrayU16::<1000>::new();
    edges.into_iter().take(n).for_each(|conn| {
        dsu.union(conn.a, conn.b);
    });

    let mut circuits = (0..points.len())
        .fold(std::collections::HashMap::new(), |mut acc, i| {
            *acc.entry(dsu.get_root(i)).or_default() += 1;
            acc
        })
        .into_values()
        .collect::<Vec<_>>();
    circuits.sort_unstable_by_key(|&circuit| std::cmp::Reverse(circuit));

    circuits[0..3].iter().product::<usize>()
}

pub fn part_two(input: &str) -> Option<usize> {
    let points = parse(input);

    let mut edges = Vec::with_capacity(points.len() * points.len() / 2);
    for i in 0..points.len() - 1 {
        for j in i + 1..points.len() {
            let dist = points[i].euclidean_distance(&points[j]);
            if dist < MAX {
                edges.push(Connection::new(i, j, dist));
            }
        }
    }
    edges.sort();

    let mut dsu = aph_disjoint_set::DisjointSetArrayU16::<1000>::new();
    let conn = edges
        .into_iter()
        .filter(|conn| {
            matches!(
                dsu.union(conn.a, conn.b),
                aph_disjoint_set::UnionResult::Success
            )
        })
        .last()
        .unwrap();

    Some(points[conn.a].x() * points[conn.b].x())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let points = parse(&advent_of_code::template::read_file("examples", DAY));
        let result = build_circuit(&points, 10);
        assert_eq!(result, 40);
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(25272));
    }
}
