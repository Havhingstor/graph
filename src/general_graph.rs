use std::{
    collections::{HashMap, HashSet},
    hash::Hash,
};

use crate::Graph;

#[derive(Default, Debug)]
pub struct GeneralGraph<T> {
    nodes: HashMap<T, HashSet<T>>,
}

impl<T> Graph<T> for GeneralGraph<T>
where
    T: Copy + Eq + Hash,
{
    fn get_insert_targets_mut(&mut self, node: T) -> &mut HashSet<T> {
        self.nodes.entry(node).or_default()
    }

    fn get_targets_mut(&mut self, node: &T) -> Option<&mut HashSet<T>> {
        self.nodes.get_mut(node)
    }

    fn get_targets(&self, node: &T) -> Option<&HashSet<T>> {
        self.nodes.get(node)
    }

    fn get_nodes(&self) -> HashSet<T> {
        let mut result = HashSet::from_iter(self.nodes.keys().copied());

        self.nodes
            .values()
            .for_each(|targets| result.extend(targets));

        result
    }
}

impl<T> GeneralGraph<T>
where
    T: Copy + Eq + Hash,
{
    pub fn add_node(&mut self, node: T) {
        self.get_insert_targets_mut(node);
    }

    pub fn delete_node(&mut self, node: &T) {
        self.nodes.remove(node);
        self.nodes.iter_mut().for_each(|(_, targets)| {
            targets.remove(node);
        });
    }
}
