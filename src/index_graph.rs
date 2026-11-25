//! A graph operating on [`usize`]
use crate::{Edges, Graph};
use std::collections::HashSet;

#[derive(Default, Debug)]
/// A graph operating on [`usize`]
///
/// ```
/// # use std::collections::HashSet;
/// # use graph::{Graph, index_graph::IndexGraph, Edges};
/// let mut our_graph = IndexGraph::default();
///
/// our_graph.set_size(2);
///
/// our_graph.get_targets_mut(&0).unwrap().add(&1);
/// our_graph.get_insert_targets_mut(&2).add(&0);
/// our_graph.get_targets_mut(&1).unwrap().add(&2);
///
/// assert_eq!(our_graph.get_nodes(), HashSet::from([0, 1, 2]));
/// assert_eq!(our_graph.get_targets(&1).unwrap().clone_hash_set(), HashSet::from([2]));
/// ```
pub struct IndexGraph {
    /// The list of edges of the graph, in the order of their originating node. The size of this
    /// determines which nodes exist in the graph.
    edges: Vec<LimitedSet>,
}

impl Graph<usize, LimitedSet> for IndexGraph {
    fn get_insert_targets_mut(&mut self, node: &usize) -> &mut LimitedSet {
        self.ensure_node(*node);
        self.edges.get_mut(*node).unwrap()
    }

    fn get_targets_mut(&mut self, node: &usize) -> Option<&mut LimitedSet> {
        self.edges.get_mut(*node)
    }

    fn get_targets(&self, node: &usize) -> Option<&LimitedSet> {
        self.edges.get(*node)
    }

    fn get_nodes(&self) -> HashSet<usize> {
        HashSet::from_iter(0..self.edges.len())
    }
}

impl IndexGraph {
    /// Create a new graph with the given size (Nodes from `0` to `size - 1`), and no edges.
    ///
    /// ```
    /// # use std::collections::HashSet;
    /// # use graph::{Graph, index_graph::IndexGraph};
    /// let our_graph = IndexGraph::new(3);
    ///
    /// assert_eq!(our_graph.get_nodes(), HashSet::from([0, 1, 2]));
    /// ```
    #[allow(clippy::must_use_candidate)]
    pub fn new(size: usize) -> IndexGraph {
        IndexGraph {
            edges: vec![LimitedSet::new(size); size],
        }
    }

    /// Sets the size of the graph. Effectively nodes `size` and up are removed
    ///
    /// ```
    /// # use std::collections::HashSet;
    /// # use graph::{Graph, index_graph::IndexGraph};
    /// let mut our_graph = IndexGraph::new(3);
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
        self.edges.resize(size, LimitedSet::new(size));
        self.edges
            .iter_mut()
            .for_each(|edge_list| edge_list.set_bound(size));
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

#[derive(Debug, Clone)]
/// A [`HashSet`] with an upper bound, to enforce the limit of nodes in an [`IndexGraph`]
///
/// ```
/// # use graph::{index_graph::{IndexGraph, LimitedSet}, Edges, Graph};
/// # use std::collections::HashSet;
/// let mut graph = IndexGraph::new(4);
/// let mut targets = graph.get_targets_mut(&0).unwrap();
/// targets.add(&3);
/// targets.add(&2);
/// targets.add(&4); // Too high, so won't be added
/// targets.delete(&2);
/// assert_eq!(targets.as_ref(), &HashSet::from([3]))
/// ```
pub struct LimitedSet {
    /// The internal [`HashSet`] that builds the foundation of the `LimitedSet`.
    internal: HashSet<usize>,
    /// The upper bound. Any newly added value must be smaller than this.
    upper_bound: usize,
}

impl LimitedSet {
    #[must_use]
    /// Clone the internal [`HashSet`] and get full access to it.
    ///
    /// ```
    /// # use graph::{index_graph::{IndexGraph, LimitedSet}, Edges, Graph};
    /// # use std::collections::HashSet;
    /// let mut graph = IndexGraph::new(4);
    /// let mut targets = graph.get_targets_mut(&0).unwrap();
    /// targets.add(&3);
    /// targets.add(&2);
    /// assert_eq!(targets.clone_hash_set(), HashSet::from([3, 2]))
    /// ```
    pub fn clone_hash_set(&self) -> HashSet<usize> {
        self.internal.clone()
    }

    /// Create a new `LimitedSet` with the given upper bound.
    fn new(bound: usize) -> LimitedSet {
        LimitedSet {
            internal: HashSet::default(),
            upper_bound: bound,
        }
    }

    /// Set the bound of the `LimitedSet`, and remove all entries at or above it.
    fn set_bound(&mut self, upper_bound: usize) {
        self.internal = self
            .internal
            .iter()
            .filter_map(|target| {
                if *target < upper_bound {
                    Some(*target)
                } else {
                    None
                }
            })
            .collect();
        self.upper_bound = upper_bound;
    }
}

impl Edges<usize> for LimitedSet {
    fn add(&mut self, target: &usize) {
        if target < &self.upper_bound {
            self.internal.insert(*target);
        }
    }

    fn delete(&mut self, target: &usize) {
        self.internal.delete(target);
    }

    fn edge_iter(&self) -> impl Iterator<Item = usize> {
        self.internal.edge_iter()
    }

    fn is_target(&self, target: &usize) -> bool {
        self.internal.is_target(target)
    }
}

impl AsRef<HashSet<usize>> for LimitedSet {
    fn as_ref(&self) -> &HashSet<usize> {
        &self.internal
    }
}
