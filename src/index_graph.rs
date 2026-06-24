use crate::Graph;
use std::collections::HashSet;

#[derive(Default, Debug)]
pub struct IndexGraph {
    nodes: Vec<HashSet<usize>>,
}

impl Graph<usize> for IndexGraph {
    fn get_insert_targets_mut(&mut self, node: usize) -> &mut HashSet<usize> {
        self.ensure_node(node);
        self.nodes.get_mut(node).unwrap()
    }

    fn get_targets_mut(&mut self, node: &usize) -> Option<&mut HashSet<usize>> {
        self.nodes.get_mut(*node)
    }

    fn get_targets(&self, node: &usize) -> Option<&HashSet<usize>> {
        self.nodes.get(*node)
    }

    fn get_nodes(&self) -> HashSet<usize> {
        HashSet::from_iter(0..self.nodes.len())
    }
}

impl IndexGraph {
    pub fn new(size: usize) -> IndexGraph {
        IndexGraph {
            nodes: vec![HashSet::default(); size],
        }
    }

    pub fn set_size(&mut self, size: usize) {
        self.nodes.resize(size, HashSet::default());
    }

    fn ensure_node(&mut self, node: usize) {
        if node >= self.nodes.len() {
            self.set_size(node + 1);
        }
    }
}
