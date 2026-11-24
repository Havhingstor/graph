//! Basic types and implementations for a simple non-weighted, directed graph, with a algorithm to
//! calculate a topological order.
#![warn(clippy::missing_docs_in_private_items)]
#![warn(missing_docs)]
#![cfg_attr(doc, feature(rustdoc_missing_doc_code_examples))]
#![cfg_attr(doc, warn(rustdoc::missing_doc_code_examples))]
#![warn(clippy::pedantic)]

use std::{collections::HashSet, hash::Hash};

pub mod general_graph;
pub mod index_graph;
mod topo_order;

/// A non-weighted, directed graph.
///
/// This graph can operate on any type that is [Copy], [Hash], and [Eq].
/// Therefore it might (depending on implemention) operate on copies of keys instead of references,
/// requiring light-weight keys.
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
pub trait Graph<T>
where
    T: Copy + Eq + Hash,
{
    /// Return all the nodes that can be reached from `from` via one directed edge. If the node is
    /// not in the graph, it is inserted, and its new, empty target list is returned
    ///
    /// ```
    /// # use graph::{Graph, general_graph::GeneralGraph};
    /// # use std::collections::HashSet;
    /// let mut our_graph = GeneralGraph::<char>::default();
    ///
    /// our_graph.add_node('A');
    ///
    /// our_graph.get_insert_targets_mut('B').insert('A');
    ///
    /// assert_eq!(our_graph.get_nodes(), HashSet::from(['A', 'B']));
    /// assert_eq!(our_graph.get_targets(&'B'), Some(&HashSet::from(['A'])));
    /// ```
    fn get_insert_targets_mut(&mut self, node: &T) -> &mut HashSet<T>;
    /// Return all the nodes that can be reached from `from` via one directed edge. `None` if the
    /// node is not in the graph
    ///
    /// ```
    /// # use graph::{Graph, general_graph::GeneralGraph};
    /// # use std::collections::HashSet;
    /// let mut our_graph = GeneralGraph::<char>::default();
    ///
    /// our_graph.add_node('A');
    /// our_graph.add_node('B');
    ///
    /// our_graph.get_targets_mut(&'B').unwrap().insert('A');
    /// assert_eq!(our_graph.get_targets_mut(&'H'), None);
    ///
    /// assert_eq!(our_graph.get_targets(&'B'), Some(&HashSet::from(['A'])));
    /// ```
    fn get_targets_mut(&mut self, node: &T) -> Option<&mut HashSet<T>>;
    /// Return all the nodes that can be reached from `from` via one directed edge. `None` if the
    /// node is not in the graph
    ///
    /// ```
    /// # use graph::{Graph, general_graph::GeneralGraph};
    /// # use std::collections::HashSet;
    /// let mut our_graph = GeneralGraph::<char>::default();
    ///
    /// our_graph.add_node('A');
    /// our_graph.add_node('B');
    ///
    /// our_graph.get_targets_mut(&'B').unwrap().insert('A');
    ///
    /// assert_eq!(our_graph.get_targets(&'H'), None);
    /// assert_eq!(our_graph.get_targets(&'B'), Some(&HashSet::from(['A'])));
    /// ```
    fn get_targets(&self, node: &T) -> Option<&HashSet<T>>;
    /// Returns all the nodes in this graph
    ///
    /// ```
    /// # use graph::{Graph, general_graph::GeneralGraph};
    /// # use std::collections::HashSet;
    /// let mut our_graph = GeneralGraph::<char>::default();
    ///
    /// our_graph.add_node('A');
    /// our_graph.add_node('B');
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
    /// # use graph::{Graph, general_graph::GeneralGraph};
    /// # use std::collections::HashSet;
    /// let mut our_graph = GeneralGraph::<char>::default();
    ///
    /// our_graph.add_node('A');
    /// our_graph.add_node('B');
    /// our_graph.add_node('C');
    ///
    /// our_graph.get_targets_mut(&'A').unwrap().insert('B');
    /// our_graph.get_targets_mut(&'B').unwrap().insert('A');
    /// our_graph.get_targets_mut(&'B').unwrap().insert('C');
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
