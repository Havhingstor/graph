//! The functions to implement the A* algorithm to calculate the shortest path in a graph.
use std::{cmp::Reverse, collections::HashMap, hash::Hash};

use priority_queue::PriorityQueue;

use crate::{Edges, Graph};

pub(super) fn calculate_a_star<G, T, E>(
    graph: &G,
    starting_node: &T,
    end_node: &T,
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

        if &currently_visiting == end_node {
            break;
        }
        let new_g = current_g + 1;

        graph
            .get_targets(&currently_visiting)
            .into_iter()
            .flat_map(E::edge_iter)
            .for_each(|neighbour| {
                if g_values
                    .get(&neighbour)
                    .is_none_or(|(_, old_g)| old_g > &new_g)
                {
                    g_values.insert(neighbour, (currently_visiting, new_g));
                    let current_estimate = *h_values
                        .entry(neighbour)
                        .or_insert_with(|| heuristic_function(&neighbour))
                        + new_g;
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

#[cfg(test)]
mod test {
    use crate::{Graph, a_star::calculate_a_star, general_graph::GeneralGraph};

    #[test]
    fn test_a_star() {
        let mut graph = GeneralGraph::default();
        for node in "ABCDEFGHI".chars() {
            graph.add_node(&node);
        }

        graph.get_targets_mut(&'A').unwrap().extend("BD".chars());
        graph.get_targets_mut(&'B').unwrap().insert('C');
        graph.get_targets_mut(&'D').unwrap().insert('E');
        graph.get_targets_mut(&'E').unwrap().insert('C');
        graph.get_targets_mut(&'C').unwrap().insert('F');
        graph.get_targets_mut(&'C').unwrap().insert('G');
        graph.get_targets_mut(&'G').unwrap().insert('H');
        graph.get_targets_mut(&'H').unwrap().insert('I');

        let heuristic = |node: &char| {
            if node == &'B' { 5 } else { 0 }
        };

        let table = calculate_a_star(&graph, &'A', &'I', heuristic);
        assert_eq!(table[&'C'], 'B');
    }
}
