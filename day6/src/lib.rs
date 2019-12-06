enum DistanceState {
    Finished(u32),
    You(u32),
    San(u32),
    Nothing,
}

use DistanceState::*;

struct Node {
    pub name: String,
    pub relation: Vec<usize>,
}

impl Node {
    pub fn new(name: &str) -> Self {
        Node {
            name: String::from(name),
            relation: Vec::new(),
        }
    }

    pub fn relation(&mut self, idx: usize) {
        self.relation.push(idx);
    }
}

pub struct Tree {
    tree: Vec<Node>,
}

impl Tree {
    /// create a new tree
    pub fn new() -> Self {
        Tree { tree: Vec::new() }
    }

    /// return the index where the node was inserted
    fn insert(&mut self, n: &str) -> usize {
        self.tree.push(Node::new(n));
        self.tree.len() - 1
    }

    /// create an orbits between two node (existing or not)
    pub fn orbits(&mut self, a: &str, b: &str) {
        let a = self
            .tree
            .iter()
            .position(|n| n.name == a)
            .unwrap_or_else(|| self.insert(a));
        let b = self
            .tree
            .iter()
            .position(|n| n.name == b)
            .unwrap_or_else(|| self.insert(b));

        self.tree[a].relation(b);
    }

    /// return the total number of direct and indirect orbits
    pub fn total_orbits(&self) -> u32 {
        let base = self.tree.iter().position(|n| n.name == "COM").unwrap();
        self._total_orbits(base, 0)
    }

    fn _total_orbits(&self, idx: usize, level: u32) -> u32 {
        let n = &self.tree[idx];
        level
            + n.relation
                .iter()
                .map(|r| self._total_orbits(*r, level + 1))
                .sum::<u32>()
    }

    pub fn distance(&self) -> u32 {
        let base = self.tree.iter().position(|n| n.name == "COM").unwrap();
        match self._distance(base) {
            Finished(u) => u,
            You(u) => panic!("you {}", u),
            San(s) => panic!("san {}", s),
            Nothing => panic!("Nothing was found"),
        }
    }

    fn _distance(&self, idx: usize) -> DistanceState {
        let n = &self.tree[idx];
        if n.name == "SAN" {
            return San(0);
        } else if n.name == "YOU" {
            return You(0);
        }
        let mut you = None;
        let mut san = None;
        for i in n.relation.iter() {
            match self._distance(*i) {
                Finished(u) => return Finished(u),
                You(u) => you = Some(u),
                San(u) => san = Some(u),
                Nothing => (),
            }
        }
        match (you, san) {
            (Some(y), Some(s)) => Finished(y + s),
            (Some(y), None) => You(y + 1),
            (None, Some(s)) => San(s + 1),
            (None, None) => Nothing,
        }
    }
}
