//! A general graph operating on `T`
//!
//! The nodes are stored as owned objects in [`HashMap`] and [`HashSet`], `T` should therefore be
//! light-weight.
use std::{
    collections::{HashMap, HashSet},
    hash::Hash,
};

use crate::Graph;

#[derive(Default, Debug)]
/// A general graph operating on `T`
///
/// The nodes are stored as owned objects in [`HashMap`] and [`HashSet`], `T` should therefore be
/// light-weight.
///
/// ```
/// # use std::collections::HashSet;
/// # use graph::{Graph, general_graph::GeneralGraph};
/// let mut our_graph = GeneralGraph::<char>::default();
///
/// our_graph.add_node('A');
/// our_graph.add_node('B');
///
/// our_graph.get_targets_mut(&'A').unwrap().insert('B');
/// our_graph.get_targets_mut(&'B').unwrap().insert('C');
/// our_graph.get_insert_targets_mut('C').insert('A');
///
/// assert_eq!(our_graph.get_nodes(), HashSet::from(['A', 'B', 'C']));
/// assert_eq!(our_graph.get_targets(&'B'), Some(&HashSet::from(['C'])));
/// ```
pub struct GeneralGraph<T> {
    /// The directed edges of the graph, as a function of their origin node.
    edges: HashMap<T, HashSet<T>>,
}

impl<T> Graph<T> for GeneralGraph<T>
where
    T: Copy + Eq + Hash,
{
    fn get_insert_targets_mut(&mut self, node: T) -> &mut HashSet<T> {
        self.edges.entry(node).or_default()
    }

    fn get_targets_mut(&mut self, node: &T) -> Option<&mut HashSet<T>> {
        self.edges.get_mut(node)
    }

    fn get_targets(&self, node: &T) -> Option<&HashSet<T>> {
        self.edges.get(node)
    }

    fn get_nodes(&self) -> HashSet<T> {
        let mut result = HashSet::from_iter(self.edges.keys().copied());

        self.edges
            .values()
            .for_each(|targets| result.extend(targets));

        result
    }
}

impl<T> GeneralGraph<T>
where
    T: Copy + Eq + Hash,
{
    /// Add a node to the graph, without adding any edges to connect it to the already present
    /// nodes.
    ///
    /// ```
    /// # use std::collections::HashSet;
    /// # use graph::{Graph, general_graph::GeneralGraph};
    /// let mut our_graph = GeneralGraph::<char>::default();
    ///
    /// our_graph.add_node('A');
    ///
    /// assert_eq!(our_graph.get_nodes(), HashSet::from(['A']));
    /// ```
    pub fn add_node(&mut self, node: T) {
        self.get_insert_targets_mut(node);
    }

    /// Delete a node from the graph, and all edges connecting to it.
    ///
    /// ```
    /// # use std::collections::HashSet;
    /// # use graph::{Graph, general_graph::GeneralGraph};
    /// let mut our_graph = GeneralGraph::<char>::default();
    ///
    /// our_graph.add_node('A');
    /// assert_eq!(our_graph.get_nodes(), HashSet::from(['A']));
    ///
    /// our_graph.delete_node(&'A');
    /// assert_eq!(our_graph.get_nodes(), HashSet::default());
    /// ```
    pub fn delete_node(&mut self, node: &T) {
        self.edges.remove(node);
        self.edges.iter_mut().for_each(|(_, targets)| {
            targets.remove(node);
        });
    }

    /// Minimizes the stored graph.
    ///
    /// Duplicate node storage is removed where it isn't needed.
    /// The actual set of nodes or edges doesn't change.
    ///
    /// ```
    /// # use std::collections::HashSet;
    /// # use graph::{Graph, general_graph::GeneralGraph};
    /// let mut our_graph = GeneralGraph::<char>::default();
    ///
    /// our_graph.add_node('A');
    /// our_graph.add_node('B');
    ///
    /// our_graph.get_targets_mut(&'A').unwrap().insert('B');
    ///
    /// our_graph.minimize();
    ///
    /// assert_eq!(our_graph.get_nodes(), HashSet::from(['A', 'B']));
    /// ```
    pub fn minimize(&mut self) {
        // All the nodes that don't need to be stored as keys
        let sinks: HashSet<T> = self
            .edges
            .iter()
            .filter_map(
                |(key, value)| {
                    if value.is_empty() { Some(*key) } else { None }
                },
            )
            .collect();

        // Create one owning iterator iterating over all the nodes that are both in sinks and in the
        // targets of another node. First, each of the nodes is once for each edge that leads to it,
        // deduplication is done when collecting into the HashSet.
        let sinks: HashSet<T> = self
            .edges
            .values()
            .flat_map(|targets| sinks.intersection(targets))
            .copied()
            .collect();

        for node in sinks {
            self.edges.remove(&node);
        }
    }
}
