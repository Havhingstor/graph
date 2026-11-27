//! The functions to implement the A* algorithm to calculate the shortest path in a graph.
use std::{cmp::Reverse, collections::HashMap, hash::Hash};

use priority_queue::PriorityQueue;

use crate::{Edges, Graph};

/// Calculate the predecessor table for a given graph and starting node.
///
/// If `end_node` isn't [`None`], the algorithm terminates once it is found, meaning only the path
/// from the `starting_node` to it is guaranteed to be optimal.
///
/// The `heuristic_function` can be used to guide and speed up the search process. For each given
/// node, it should return an estimate of the remaining number of hops from this node to the end
/// node in the shortest path.
///
/// **It is important that this value is always at most the actual number,
/// otherwise the calculated path might not be optimal!**
/// Returning a high value (like [`usize::MAX`]) in this case improves performance.
///
/// The starting node is not inserted in the table, but it is present as the predecessor of its
/// neighbours.
pub(super) fn calculate_a_star<G, T, E>(
    graph: &G,
    starting_node: &T,
    end_node: Option<&T>,
    mut heuristic_function: impl FnMut(&T) -> usize,
) -> HashMap<T, T>
where
    G: Graph<T, E> + ?Sized,
    T: Copy + Hash + Eq,
    E: Edges<T>,
{
    let mut h_values: HashMap<_, _> = HashMap::default();
    let mut g_values: HashMap<_, _> = HashMap::default();
    let mut f_values: PriorityQueue<_, _> = PriorityQueue::default();

    let mut next_node_opt = Some(*starting_node);
    g_values.insert(*starting_node, (*starting_node, 0));

    while let Some(currently_visiting) = next_node_opt {
        let (_, current_g) = g_values.get(&currently_visiting).unwrap();

        if Some(&currently_visiting) == end_node {
            break;
        }
        let new_g = current_g + 1;

        graph
            .get_targets(&currently_visiting)
            .into_iter()
            .flat_map(E::edge_iter)
            .for_each(|neighbour| {
                let (predecessor, old_g) = g_values
                    .entry(neighbour)
                    .or_insert((currently_visiting, usize::MAX));
                if *old_g > new_g {
                    *predecessor = currently_visiting;
                    *old_g = new_g;
                    let current_estimate = h_values
                        .entry(neighbour)
                        .or_insert_with(|| heuristic_function(&neighbour))
                        // Needed here because the heuristic might be `usize::MAX`
                        .saturating_add(new_g);
                    f_values.push(neighbour, Reverse(current_estimate));
                }
            });

        next_node_opt = f_values.pop().map(|(item, _)| item);
    }

    let result_iter = g_values.into_iter().filter_map(|(node, (predecessor, _))| {
        if &node == starting_node {
            None
        } else {
            Some((node, predecessor))
        }
    });
    HashMap::from_iter(result_iter)
}
