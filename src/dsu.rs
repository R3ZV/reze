/// Disjoint Set Union datastructure
///
/// This DSU implementation makes use of path compression to optimize for the
/// `find_parent`, all the merges are done from small to large, that is why
/// "rank" each node based one the size of its set length.

pub struct Dsu {
    nodes: usize,
    parent: Vec<usize>,

    /// the size of a particular set
    sz: Vec<u32>,
}

impl Dsu {
    pub fn new(nodes: usize) -> Self {
        let sz = vec![1; nodes];
        let parent = (0..nodes).collect();

        Self { nodes, parent, sz }
    }

    pub fn find_parent(&mut self, node: usize) -> usize {
        if self.parent[node] == node {
            return node;
        }

        self.parent[node] = self.find_parent(self.parent[node]);

        self.parent[node]
    }

    pub fn same_set(&mut self, node_a: usize, node_b: usize) -> bool {
        self.find_parent(node_a) == self.find_parent(node_b)
    }

    /// Small to large merge between two sets.
    ///
    /// Returns [`true`] if a merge occured (e.g. node_a and node_b are not
    /// in the same set)
    pub fn merge(&mut self, node_a: usize, node_b: usize) -> bool {
        if self.same_set(node_a, node_b) {
            return false;
        }

        let a_parent = self.find_parent(node_a);
        let b_parent = self.find_parent(node_b);

        if self.sz[a_parent] > self.sz[b_parent] {
            self.sz[a_parent] += self.sz[b_parent];
            self.parent[b_parent] = a_parent;
        } else {
            self.sz[b_parent] += self.sz[a_parent];
            self.parent[a_parent] = b_parent;
        }

        return true;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn same_set() {
        let mut dsu = Dsu::new(10);

        assert!(!dsu.same_set(1, 3));
        assert!(!dsu.same_set(0, 3));
        assert!(dsu.same_set(3, 3));
    }

    #[test]
    fn merge() {
        let mut dsu = Dsu::new(10);

        dsu.merge(3, 2);
        dsu.merge(4, 2);

        dsu.merge(5, 1);

        dsu.merge(2, 1);

        assert_eq!(dsu.sz[2], 5);
        assert_eq!(dsu.parent[1], 2);
        assert_eq!(dsu.find_parent(5), 2);
    }

    #[test]
    fn find_parent() {
        let mut dsu = Dsu::new(10);

        assert_eq!(dsu.find_parent(1), 1);

        dsu.merge(3, 1);
        dsu.merge(2, 1);

        assert_eq!(dsu.find_parent(1), 1);
        assert_eq!(dsu.find_parent(2), 1);
        assert_eq!(dsu.find_parent(3), 1);
    }
}
