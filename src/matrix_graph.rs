//! A graph operating on [`usize`]
use crate::{Edges, Graph};
use std::collections::HashSet;

#[derive(Default, Debug)]
/// A graph operating on [`usize`]
///
/// The edges are stored as an adjacency matrix.
/// This means that this graph is very fast, but takes always as much space as a complete graph.
///
/// ```
/// # use std::collections::HashSet;
/// # use graph::{Graph, matrix_graph::MatrixGraph, Edges};
/// let mut our_graph = MatrixGraph::default();
///
/// our_graph.set_size(2);
///
/// our_graph.get_targets_mut(&0).unwrap().add(&1);
/// our_graph.get_insert_targets_mut(&2).add(&0);
/// our_graph.get_targets_mut(&1).unwrap().add(&2);
///
/// assert_eq!(our_graph.get_nodes(), HashSet::from([0, 1, 2]));
/// assert_eq!(our_graph.get_targets(&1), Some(&vec![false, false, true]));
/// ```
pub struct MatrixGraph {
    /// The list of edges of the graph, in the order of their originating node. The size of this
    /// determines which nodes exist in the graph.
    edges: Vec<Vec<bool>>,
}

impl Graph<usize, Vec<bool>> for MatrixGraph {
    fn get_insert_targets_mut(&mut self, node: &usize) -> &mut Vec<bool> {
        self.ensure_node(*node);
        self.edges.get_mut(*node).unwrap()
    }

    fn get_targets_mut(&mut self, node: &usize) -> Option<&mut Vec<bool>> {
        self.edges.get_mut(*node)
    }

    fn get_targets(&self, node: &usize) -> Option<&Vec<bool>> {
        self.edges.get(*node)
    }

    fn get_nodes(&self) -> HashSet<usize> {
        HashSet::from_iter(0..self.edges.len())
    }
}

impl MatrixGraph {
    /// Create a new graph with the given size (Nodes from `0` to `size - 1`), and no edges.
    ///
    /// ```
    /// # use std::collections::HashSet;
    /// # use graph::{Graph, matrix_graph::MatrixGraph};
    /// let our_graph = MatrixGraph::new(3);
    ///
    /// assert_eq!(our_graph.get_nodes(), HashSet::from([0, 1, 2]));
    /// ```
    #[allow(clippy::must_use_candidate)]
    pub fn new(size: usize) -> MatrixGraph {
        MatrixGraph {
            edges: vec![vec![false; size]; size],
        }
    }

    /// Sets the size of the graph. Effectively nodes `size` and up are removed
    ///
    /// ```
    /// # use std::collections::HashSet;
    /// # use graph::{Graph, matrix_graph::MatrixGraph};
    /// let mut our_graph = MatrixGraph::new(3);
    ///
    /// // You can shrink the graph
    /// our_graph.set_size(2);
    /// assert_eq!(our_graph.get_nodes(), HashSet::from([0, 1]));
    ///
    /// // Or grow it
    /// our_graph.set_size(6);
    /// assert_eq!(our_graph.get_nodes(), HashSet::from([0, 1, 2, 3, 4, 5]));
    /// ```
    pub fn set_size(&mut self, size: usize) {
        let old_core = size.min(self.edges.len());
        self.edges.resize(size, vec![false; size]);
        for edge_list in &mut self.edges[..old_core] {
            edge_list.resize(size, false);
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

impl Edges<usize> for Vec<bool> {
    fn add(&mut self, target: &usize) {
        if &self.len() > target {
            self[*target] = true;
        }
    }

    fn delete(&mut self, target: &usize) {
        if &self.len() > target {
            self[*target] = false;
        }
    }

    fn edge_iter(&self) -> impl Iterator<Item = usize> {
        self.iter()
            .enumerate()
            .filter_map(|(idx, flag)| if *flag { Some(idx) } else { None })
    }

    fn is_target(&self, target: &usize) -> bool {
        &self.len() > target && self[*target]
    }
}
