use super::Graph;
use std::collections::{HashMap, HashSet};
use std::hash::Hash;

enum TopoOrderNodeStatus {
    Visiting,
    Visited,
}

struct CircDependency<'a, T> {
    /// The actual nodes that are visited twice.
    /// Once their "upper" visit has concluded no additional notes will be found to be affected
    /// (within one sub-circle)
    circ_node: HashSet<&'a T>,
    /// All the nodes within the circle
    affected_nodes: HashSet<&'a T>,
}

impl<T: Eq + Hash> CircDependency<'_, T> {
    fn with_node(node: &T) -> CircDependency<'_, T> {
        CircDependency {
            circ_node: HashSet::from_iter(vec![node]),
            affected_nodes: Default::default(),
        }
    }
}

impl<T> Default for CircDependency<'_, T> {
    fn default() -> Self {
        Self {
            circ_node: Default::default(),
            affected_nodes: Default::default(),
        }
    }
}

pub(super) struct TopoOrder<'a, T, G>
where
    T: Copy + Eq + Hash,
    G: Graph<T> + ?Sized,
{
    node_status: HashMap<&'a T, TopoOrderNodeStatus>,
    graph: &'a G,
    reversed_order: Vec<HashSet<&'a T>>,
}

impl<'a, T, G> TopoOrder<'a, T, G>
where
    T: Copy + Eq + Hash,
    G: Graph<T> + ?Sized,
{
    pub(super) fn calculate_topo_order(graph: &'a G) -> Vec<HashSet<T>> {
        let mut order = TopoOrder {
            graph,
            node_status: Default::default(),
            reversed_order: Default::default(),
        };

        let nodes = graph.get_nodes();
        nodes.iter().for_each(|node| {
            order.go_through_from_node(node);
        });

        order
            .reversed_order
            .into_iter()
            .rev()
            .map(|group| group.into_iter().map(|node| node.to_owned()).collect())
            .collect()
    }

    fn go_through_from_node(&mut self, node: &'a T) -> Option<CircDependency<'a, T>> {
        match self.node_status.get(node) {
            Some(TopoOrderNodeStatus::Visiting) => {
                return Some(CircDependency::with_node(node));
            }
            Some(TopoOrderNodeStatus::Visited) => return None,
            _ => (),
        };

        self.node_status.insert(node, TopoOrderNodeStatus::Visiting);

        let mut result = self
            .graph
            .get_targets(node)
            .iter()
            // Here we generate an iterator over all targets, and `None` is transformed into an
            // empty iterator
            .flat_map(|target_set| target_set.iter())
            // No we find all circular references from each following node, and combine them
            // into one object
            .filter_map(|next| self.go_through_from_node(next))
            .reduce(|mut acc, new_dependency| {
                acc.circ_node.extend(new_dependency.circ_node);
                acc.affected_nodes.extend(new_dependency.affected_nodes);
                acc
            })
            .unwrap_or_default();

        self.node_status.insert(node, TopoOrderNodeStatus::Visited);

        // If this node was one where a sub-circle became apparent, we have now found all nodes in
        // that sub-circle
        result.circ_node.remove(node);
        // But we are certainly within the circle (or a pseudo-circle with just our node)
        result.affected_nodes.insert(node);

        // So if all sub-circles are finished, we actually found all nodes
        if result.circ_node.is_empty() {
            self.reversed_order.push(result.affected_nodes);

            None
        } else {
            Some(result)
        }
    }
}
