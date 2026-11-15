//! A graph operating on [`usize`]
use crate::Graph;
use std::collections::HashSet;

#[derive(Default, Debug)]
/// A graph operating on [`usize`]
pub struct IndexGraph {
    /// The list of edges of the graph, in the order of their originating node. The size of this
    /// determines which nodes exist in the graph.
    edges: Vec<HashSet<usize>>,
}

impl Graph<usize> for IndexGraph {
    fn get_insert_targets_mut(&mut self, node: usize) -> &mut HashSet<usize> {
        self.ensure_node(node);
        self.edges.get_mut(node).unwrap()
    }

    fn get_targets_mut(&mut self, node: &usize) -> Option<&mut HashSet<usize>> {
        self.edges.get_mut(*node)
    }

    fn get_targets(&self, node: &usize) -> Option<&HashSet<usize>> {
        self.edges.get(*node)
    }

    fn get_nodes(&self) -> HashSet<usize> {
        HashSet::from_iter(0..self.edges.len())
    }
}

impl IndexGraph {
    /// Create a new graph with the given size (Nodes from `0` to `size - 1`), and no nodes.
    #[allow(clippy::must_use_candidate)]
    pub fn new(size: usize) -> IndexGraph {
        IndexGraph {
            edges: vec![HashSet::default(); size],
        }
    }

    /// Sets the size of the graph. Effectively nodes `size` and up are removed
    pub fn set_size(&mut self, size: usize) {
        self.edges.resize(size, HashSet::default());
        for edge_list in &mut self.edges {
            *edge_list = edge_list
                .iter()
                .filter_map(|target| if *target < size { Some(*target) } else { None })
                .collect();
        }
    }

    /// Ensure the node is in the graph.
    ///
    /// This is achieved by resizing the graph if necessary.
    fn ensure_node(&mut self, node: usize) {
        if node >= self.edges.len() {
            self.set_size(node + 1);
        }
    }
}
