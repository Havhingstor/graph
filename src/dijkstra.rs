//! The functions to implement the Dijkstra algorithm to calculate the shortest path in a graph.
use std::{
    collections::{HashMap, HashSet},
    hash::Hash,
};

use crate::{Edges, Graph};

/// Calculate the dijkstra distance and predecessor table for a given graph and starting node.
///
/// The starting node is not inserted in the table, but it is present as the predecessor of its
/// neighbours.
pub(super) fn calculate_dijkstra<G, T, E>(graph: &G, starting_node: &T) -> HashMap<T, (T, usize)>
where
    G: Graph<T, E> + ?Sized,
    T: Copy + Hash + Eq,
    E: Edges<T>,
{
    let mut unseen = graph.get_nodes();

    let mut result = HashMap::default();

    let mut next_node_opt = Some((*starting_node, 0));

    while let Some((currently_visiting, distance)) = next_node_opt {
        let new_distance = distance + 1;
        unseen.remove(&currently_visiting);

        let neighbours: HashSet<_> = graph
            .get_targets(&currently_visiting)
            .into_iter()
            .flat_map(E::edge_iter)
            .collect();

        let updating_nodes = neighbours.intersection(&unseen);

        for updating in updating_nodes {
            if result
                .get(updating)
                .is_none_or(|(_, previous_distance)| previous_distance > &new_distance)
            {
                result.insert(*updating, (currently_visiting, new_distance));
            }
        }

        next_node_opt = find_next_node(&unseen, &result);
    }

    result
}

/// Finds the next node (and its current distance) from the unseen ones.
///
/// Selects the node with the shortest current distance. If there are no nodes with a current
/// distance, [`None`] is returned.
fn find_next_node<T>(unseen: &HashSet<T>, distances: &HashMap<T, (T, usize)>) -> Option<(T, usize)>
where
    T: Copy + Hash + Eq,
{
    let min = unseen
        .iter()
        .map(|node| (node, distances.get(node).map(|(_, distance)| distance)))
        .min_by_key(|(_, distance)| distance.unwrap_or(&usize::MAX));

    if let Some(min) = min
        && let Some(distance) = min.1
    {
        Some((*min.0, *distance))
    } else {
        None
    }
}
