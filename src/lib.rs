//! Basic types and implementations for a simple non-weighted, directed graph, with a algorithm to
//! calculate a topological order.
#![warn(clippy::missing_docs_in_private_items)]
#![warn(missing_docs)]
#![cfg_attr(doc, feature(rustdoc_missing_doc_code_examples))]
#![cfg_attr(doc, warn(rustdoc::missing_doc_code_examples))]
#![warn(clippy::pedantic)]

use std::{
    collections::HashSet,
    hash::{BuildHasher, Hash},
};

pub mod general_graph;
pub mod index_graph;
pub mod matrix_graph;
mod topo_order;

/// A non-weighted, directed graph.
///
/// This graph can operate on any type that is [Copy], [Hash], and [Eq].
/// Therefore it might (depending on implemention) operate on copies of keys instead of references,
/// requiring light-weight keys.
///
/// ```
/// # use std::collections::HashSet;
/// # use graph::{Graph, general_graph::GeneralGraph, Edges};
/// let mut our_graph = GeneralGraph::<char>::default();
///
/// our_graph.add_node(&'A');
/// our_graph.add_node(&'B');
///
/// our_graph.get_targets_mut(&'A').unwrap().add(&'B');
/// our_graph.get_targets_mut(&'B').unwrap().add(&'C');
/// our_graph.get_insert_targets_mut(&'C').add(&'A');
///
/// assert_eq!(our_graph.get_nodes(), HashSet::from(['A', 'B', 'C']));
/// assert_eq!(our_graph.get_targets(&'B'), Some(&HashSet::from(['C'])));
/// ```
pub trait Graph<T, E>
where
    T: Copy + Eq + Hash,
    E: Edges<T>,
{
    /// Return all the nodes that can be reached from `from` via one directed edge. If the node is
    /// not in the graph, it is inserted, and its new, empty target list is returned.
    /// Note: Depending on the implementation it might not be guaranteed that all returned elements
    /// are actually nodes.
    ///
    /// ```
    /// # use graph::{Graph, general_graph::GeneralGraph, Edges};
    /// # use std::collections::HashSet;
    /// let mut our_graph = GeneralGraph::<char>::default();
    ///
    /// our_graph.add_node(&'A');
    ///
    /// our_graph.get_insert_targets_mut(&'B').add(&'A');
    ///
    /// assert_eq!(our_graph.get_nodes(), HashSet::from(['A', 'B']));
    /// assert_eq!(our_graph.get_targets(&'B'), Some(&HashSet::from(['A'])));
    /// ```
    fn get_insert_targets_mut(&mut self, node: &T) -> &mut E;
    /// Return all the nodes that can be reached from `from` via one directed edge. `None` if the
    /// node is not in the graph.
    /// Note: Depending on the implementation it might not be guaranteed that all returned elements
    /// are actually nodes.
    ///
    /// ```
    /// # use graph::{Graph, general_graph::GeneralGraph, Edges};
    /// # use std::collections::HashSet;
    /// let mut our_graph = GeneralGraph::<char>::default();
    ///
    /// our_graph.add_node(&'A');
    /// our_graph.add_node(&'B');
    ///
    /// our_graph.get_targets_mut(&'B').unwrap().add(&'A');
    /// assert_eq!(our_graph.get_targets_mut(&'H'), None);
    ///
    /// assert_eq!(our_graph.get_targets(&'B'), Some(&HashSet::from(['A'])));
    /// ```
    fn get_targets_mut(&mut self, node: &T) -> Option<&mut E>;
    /// Return all the nodes that can be reached from `from` via one directed edge. `None` if the
    /// node is not in the graph.
    /// Note: Depending on the implementation it might not be guaranteed that all returned elements
    /// are actually nodes.
    ///
    /// ```
    /// # use graph::{Graph, general_graph::GeneralGraph, Edges};
    /// # use std::collections::HashSet;
    /// let mut our_graph = GeneralGraph::<char>::default();
    ///
    /// our_graph.add_node(&'A');
    /// our_graph.add_node(&'B');
    ///
    /// our_graph.get_targets_mut(&'B').unwrap().add(&'A');
    ///
    /// assert_eq!(our_graph.get_targets(&'H'), None);
    /// assert_eq!(our_graph.get_targets(&'B'), Some(&HashSet::from(['A'])));
    /// ```
    fn get_targets(&self, node: &T) -> Option<&E>;
    /// Returns all the nodes in this graph
    ///
    /// ```
    /// # use graph::{Graph, general_graph::GeneralGraph};
    /// # use std::collections::HashSet;
    /// let mut our_graph = GeneralGraph::<char>::default();
    ///
    /// our_graph.add_node(&'A');
    /// our_graph.add_node(&'B');
    ///
    /// assert_eq!(our_graph.get_nodes(), HashSet::from(['A', 'B']))
    /// ```
    fn get_nodes(&self) -> HashSet<T>;

    /// Calculate the topological order of the nodes. If there are cycles (or more generally strong
    /// components, see also [Wikipedia](https://en.wikipedia.org/wiki/Connectivity_(graph_theory)#Components_and_cuts)),
    /// they are combined into one element. To facilitate that, the ordered elements are stored in
    /// [`HashSet`]s.
    ///
    /// ```
    /// # use graph::{Graph, general_graph::GeneralGraph, Edges};
    /// # use std::collections::HashSet;
    /// let mut our_graph = GeneralGraph::<char>::default();
    ///
    /// our_graph.add_node(&'A');
    /// our_graph.add_node(&'B');
    /// our_graph.add_node(&'C');
    ///
    /// our_graph.get_targets_mut(&'A').unwrap().add(&'B');
    /// our_graph.get_targets_mut(&'B').unwrap().add(&'A');
    /// our_graph.get_targets_mut(&'B').unwrap().add(&'C');
    ///
    /// assert_eq!(
    ///     our_graph.get_topo_sort(),
    ///     [HashSet::from(['A', 'B']), HashSet::from(['C'])]
    /// );
    /// ```
    fn get_topo_sort(&self) -> Vec<HashSet<T>> {
        topo_order::topo_order(self)
    }
}

/// A collection of edges, where `T` is the target of the edge.
///
/// Storing the edges that start from a certain node.
///
/// ```
/// # use graph::Edges;
/// # use std::collections::HashSet;
/// let mut edges = HashSet::from([1, 2, 3]);
///
/// edges.add(&4);
/// edges.remove(&2);
///
/// let iterated: HashSet<_> = edges.edge_iter().collect();
/// assert_eq!(iterated, HashSet::from([1, 3, 4]));
/// ```
pub trait Edges<T>
where
    T: Copy + Eq + Hash,
{
    /// Add a new edge, by specifiying its end node.
    /// Note: Depending on implementation the node might have to be created first by calling
    /// [`Graph::get_insert_targets_mut`] or an equivalent function (like
    /// [`general_graph::GeneralGraph::add_node`]), otherwise it might not be added as a target.
    ///
    /// ```
    /// # use graph::Edges;
    /// # use std::collections::HashSet;
    /// let mut edges: HashSet<_> = Default::default();
    ///
    /// edges.add(&4);
    ///
    /// let iterated: HashSet<_> = edges.edge_iter().collect();
    /// assert_eq!(iterated, HashSet::from([4]));
    /// ```
    fn add(&mut self, target: &T);
    /// Delete an edge, by specifiying its end node.
    ///
    /// ```
    /// # use graph::Edges;
    /// # use std::collections::HashSet;
    /// let mut edges = HashSet::from([1, 2, 3]);
    ///
    /// edges.remove(&2);
    ///
    /// let iterated: HashSet<_> = edges.edge_iter().collect();
    /// assert_eq!(iterated, HashSet::from([1, 3]));
    /// ```
    fn delete(&mut self, target: &T);
    /// Iterate over all edges (or more precisely, their target nodes)
    ///
    /// ```
    /// # use graph::Edges;
    /// # use std::collections::HashSet;
    /// let edges = HashSet::from([1, 2, 3]);
    ///
    /// let iterated: HashSet<_> = edges.edge_iter().collect();
    /// assert_eq!(iterated, HashSet::from([1, 2, 3]));
    /// ```
    fn edge_iter(&self) -> impl Iterator<Item = T>;
    /// Find out if an edge with the given target exists in this collection
    ///
    /// ```
    /// # use graph::Edges;
    /// # use std::collections::HashSet;
    /// let edges = HashSet::from([1, 2, 3]);
    ///
    /// assert!(edges.is_target(&2));
    /// assert!(!edges.is_target(&4));
    /// ```
    fn is_target(&self, target: &T) -> bool;
}

impl<T, S> Edges<T> for HashSet<T, S>
where
    T: Copy + Eq + Hash,
    S: BuildHasher,
{
    fn add(&mut self, target: &T) {
        self.insert(*target);
    }

    fn delete(&mut self, target: &T) {
        self.remove(target);
    }

    fn edge_iter(&self) -> impl Iterator<Item = T> {
        self.iter().copied()
    }

    fn is_target(&self, target: &T) -> bool {
        self.contains(target)
    }
}
