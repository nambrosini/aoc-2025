use std::cmp::Ordering;

pub struct DisjointSet {
    parent: Vec<usize>,
    rank: Vec<usize>,
}

impl DisjointSet {
    pub fn new(n: usize) -> Self {
        Self {
            parent: (0..n).collect(),
            rank: vec![0; n],
        }
    }

    pub fn find(&mut self, i: usize) -> usize {
        if self.parent[i] == i {
            return i;
        }

        let p = self.parent[i];
        let root = self.find(p);
        self.parent[i] = root;
        root
    }

    pub fn union(&mut self, i: usize, j: usize) {
        let root_i = self.find(i);
        let root_j = self.find(j);

        if root_i != root_j {
            match self.rank[root_i].cmp(&self.rank[root_j]) {
                Ordering::Less => self.parent[root_i] = root_j,
                Ordering::Greater => self.parent[root_j] = root_i,
                Ordering::Equal => {
                    self.parent[root_j] = root_i;
                    self.rank[root_i] += 1;
                }
            }
        }
    }
}
