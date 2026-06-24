use std::{collections::HashSet, hash::Hash};

pub mod general_graph;
pub mod index_graph;
mod topo_order;

use topo_order::TopoOrder;

pub trait Graph<T>
where
    T: Copy + Eq + Hash,
{
    fn get_insert_targets_mut(&mut self, node: T) -> &mut HashSet<T>;
    fn get_targets_mut(&mut self, node: &T) -> Option<&mut HashSet<T>>;
    fn get_targets(&self, node: &T) -> Option<&HashSet<T>>;
    fn get_nodes(&self) -> HashSet<T>;

    fn get_topo_sort(&self) -> Vec<HashSet<T>> {
        TopoOrder::calculate_topo_order(self)
    }
}
