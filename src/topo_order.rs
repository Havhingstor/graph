//! The functions and types to implement the topological order algorithm.
use crate::Edges;

use super::Graph;
use std::collections::{HashMap, HashSet};
use std::hash::Hash;

/// Calculate the topological order of the nodes. Strong components are combined into [`HashSet`]
pub(super) fn topo_order<T, G, E>(graph: &G) -> Vec<HashSet<T>>
where
    T: Copy + Eq + Hash,
    G: Graph<T, E> + ?Sized,
    E: Edges<T>,
{
    let mut result = Vec::default();
    let mut states = HashMap::default();

    for node in &graph.get_nodes() {
        let circs = walk_from_node(graph, node, &mut states, &mut result);
        debug_assert!(circs.is_empty());
    }

    result.reverse();

    result
}

/// Walk through all subsequent nodes from the specified one, add the ones that certainly come
/// after to the current list and return any potential ongoing strong component
fn walk_from_node<'a, T, G, E>(
    graph: &'a G,
    node: &'a T,
    states: &mut HashMap<&'a T, WalkingState>,
    current_list: &mut Vec<HashSet<T>>,
) -> WalkResult<'a, T>
where
    T: Copy + Eq + Hash,
    G: Graph<T, E> + ?Sized,
    E: Edges<T> + 'a,
{
    if let Some(state) = states.get(node) {
        match state {
            WalkingState::CurrentlyVisiting => return WalkResult::with_start(node),
            WalkingState::Visited => return WalkResult::default(),
        }
    }
    states.insert(node, WalkingState::CurrentlyVisiting);

    let mut sub_result = graph
        .get_targets(node)
        .into_iter()
        // Here we generate an iterator over all targets, and `None` is transformed into an
        // empty iterator
        .flat_map(Edges::edge_iter)
        // Now we find all circular references from each following node, and combine them into one
        // object
        .map(|target| walk_from_node(graph, target, states, current_list))
        .reduce(WalkResult::extending)
        .unwrap_or_default();

    states.insert(node, WalkingState::Visited);

    // If this node was one where a sub-circle became apparent, we have now found all nodes in
    // that sub-circle
    sub_result.component_end.remove(node);
    // But we are certainly within the circle (or a pseudo-circle with just our node)
    sub_result.current_component.insert(node.to_owned());

    // So if all sub-circles are finished, we actually found all nodes
    if sub_result.component_end.is_empty() {
        current_list.push(sub_result.current_component);
        sub_result.current_component = HashSet::default();
    }

    sub_result
}

/// The state a node might be in (except not encountered at all)
enum WalkingState {
    /// The node is currently being visited, meaning its successors are walked through at the moment
    CurrentlyVisiting,
    /// The node has been visited before. It and all its successors are already in the list
    Visited,
}

/// The result of walking from a specific node
struct WalkResult<'a, T>
where
    T: Hash + Eq,
{
    /// The set of nodes that are part of the currently encountered strong component, and which have
    /// already been visited fully
    current_component: HashSet<T>,
    /// The components that will stand at the end of the strong component (These are the ones that
    /// connect cycles, the ones that were encountered a second time). They are yet to be visited
    /// the necessary second time
    component_end: HashSet<&'a T>,
}

impl<T> Default for WalkResult<'_, T>
where
    T: Hash + Eq,
{
    fn default() -> Self {
        Self {
            current_component: HashSet::default(),
            component_end: HashSet::default(),
        }
    }
}

impl<'a, T> WalkResult<'a, T>
where
    T: Hash + Eq,
{
    /// Create a new `WalkResult` with a specific component end. This should be used if we encounter
    /// a node a second time, since the component also ends when the visit to this node ends
    fn with_start(first: &'a T) -> Self {
        Self {
            component_end: HashSet::from([first]),
            ..Default::default()
        }
    }

    /// Combine two partial strong components
    fn extending(mut self, other: Self) -> Self {
        self.component_end.extend(other.component_end);
        self.current_component.extend(other.current_component);
        self
    }

    /// Returns true if there are no nodes left in a potential strong component
    fn is_empty(&self) -> bool {
        self.component_end.is_empty() && self.current_component.is_empty()
    }
}
