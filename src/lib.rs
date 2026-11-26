//! Basic types and implementations for a simple non-weighted, directed graph, with a algorithm to
//! calculate a topological order.
#![warn(clippy::missing_docs_in_private_items)]
#![warn(missing_docs)]
#![cfg_attr(doc, feature(rustdoc_missing_doc_code_examples))]
#![cfg_attr(doc, warn(rustdoc::missing_doc_code_examples))]
#![warn(clippy::pedantic)]

use std::{
    collections::{HashMap, HashSet},
    hash::{BuildHasher, Hash},
};

mod dijkstra;
#[cfg(feature = "general_graph")]
pub mod general_graph;
#[cfg(feature = "index_graph")]
pub mod index_graph;
#[cfg(feature = "matrix_graph")]
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
    /// Return all the nodes that can be reached from `node` via one directed edge. If the node is
    /// not in the graph, it is inserted, and its new, empty target list is returned.
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

    /// Return all the nodes that can be reached from `node` via one directed edge. `None` if the
    /// node is not in the graph.
    /// Note: Depending on the implementation a node without outgoing edges might also return
    /// `None`.
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

    /// Return all the nodes that can be reached from `node` via one directed edge. `None` if the
    /// node is not in the graph.
    /// Note: Depending on the implementation a node without outgoing edges might also return
    /// `None`.
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

    /// Calculate the distance and predecessor of the shortest paths of all nodes from the given
    /// starting node.
    ///
    /// The starting node is not inserted in the table, but it is present as the predecessor of its
    /// neighbours.
    /// Uses the **Dijkstra** algorithm to find the shortest paths, see also [Wikipedia](https://en.wikipedia.org/wiki/Dijkstra%27s_algorithm).
    ///
    /// ```
    /// # use graph::{Edges, Graph, general_graph::GeneralGraph};
    /// let mut our_graph = GeneralGraph::<char>::default();
    ///
    /// our_graph.add_node(&'A');
    /// our_graph.add_node(&'B');
    /// our_graph.add_node(&'C');
    /// our_graph.add_node(&'D');
    ///
    /// our_graph.get_targets_mut(&'A').unwrap().add(&'B');
    /// our_graph.get_targets_mut(&'A').unwrap().add(&'C');
    /// our_graph.get_targets_mut(&'B').unwrap().add(&'D');
    /// our_graph.get_targets_mut(&'D').unwrap().add(&'C');
    ///
    /// let dijkstra_table = our_graph.calculate_dijkstra_table(&'A');
    /// assert_eq!(dijkstra_table.get(&'B').unwrap(), &('A', 1));
    /// assert_eq!(dijkstra_table.get(&'C').unwrap(), &('A', 1));
    /// assert_eq!(dijkstra_table.get(&'D').unwrap(), &('B', 2));
    /// ```
    fn calculate_dijkstra_table(&self, starting_node: &T) -> HashMap<T, (T, usize)> {
        dijkstra::calculate_dijkstra(self, starting_node)
    }

    /// Calculates the shortest path from a starting node to an end node.
    ///
    /// If either of the nodes is not part of the graph or the end node cannot be reached from the
    /// starting node, [`None`] is returned.
    /// Uses the **Dijkstra** algorithm to find the shortest path, see also [Wikipedia](https://en.wikipedia.org/wiki/Dijkstra%27s_algorithm).
    ///
    /// ```
    /// # use graph::{Edges, Graph, general_graph::GeneralGraph};
    /// let mut our_graph = GeneralGraph::<char>::default();
    ///
    /// our_graph.add_node(&'A');
    /// our_graph.add_node(&'B');
    /// our_graph.add_node(&'C');
    /// our_graph.add_node(&'D');
    ///
    /// our_graph.get_targets_mut(&'A').unwrap().add(&'B');
    /// our_graph.get_targets_mut(&'A').unwrap().add(&'C');
    /// our_graph.get_targets_mut(&'B').unwrap().add(&'D');
    /// our_graph.get_targets_mut(&'D').unwrap().add(&'C');
    ///
    /// assert_eq!(
    ///     our_graph.find_shortest_path(&'A', &'C').unwrap().as_ref(),
    ///     &['A', 'C']
    /// );
    /// assert_eq!(
    ///     our_graph.find_shortest_path(&'A', &'D').unwrap().as_ref(),
    ///     &['A', 'B', 'D']
    /// );
    /// ```
    fn find_shortest_path(&self, starting_node: &T, end_node: &T) -> Option<Box<[T]>> {
        let mut result = vec![];
        let mut current_node = end_node;
        let dijkstra_table = self.calculate_dijkstra_table(starting_node);
        while current_node != starting_node {
            let (predecessor, _) = dijkstra_table.get(current_node)?;

            result.push(*current_node);
            current_node = predecessor;
        }
        result.push(*starting_node);
        result.reverse();

        Some(result.into_boxed_slice())
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
