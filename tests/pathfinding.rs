#[test]
fn test_dijkstra_a_star() {
    use graph::{Edges, Graph, general_graph::GeneralGraph};
    let mut our_graph = GeneralGraph::<(u8, u8)>::default();

    for i in 0..4 {
        for j in 0..4 {
            our_graph.add_node(&(i, j));
        }
    }

    our_graph
        .get_targets_mut(&(0, 0))
        .unwrap()
        .add_all(&[(1, 0), (0, 1)]);
    our_graph.get_targets_mut(&(0, 1)).unwrap().add(&(0, 2));
    our_graph
        .get_targets_mut(&(0, 2))
        .unwrap()
        .add_all(&[(0, 3), (1, 2)]);
    our_graph.get_targets_mut(&(1, 0)).unwrap().add(&(1, 1));
    our_graph
        .get_targets_mut(&(1, 1))
        .unwrap()
        .add_all(&[(2, 1), (1, 2)]);
    our_graph.get_targets_mut(&(1, 2)).unwrap().add(&(1, 3));
    our_graph.get_targets_mut(&(1, 3)).unwrap().add(&(2, 3));
    our_graph.get_targets_mut(&(2, 0)).unwrap().add(&(1, 0));
    our_graph.get_targets_mut(&(2, 1)).unwrap().add(&(3, 1));
    our_graph
        .get_targets_mut(&(2, 2))
        .unwrap()
        .add_all(&[(2, 1), (3, 2)]);
    our_graph.get_targets_mut(&(2, 3)).unwrap().add(&(2, 2));
    our_graph.get_targets_mut(&(3, 0)).unwrap().add(&(2, 0));
    our_graph
        .get_targets_mut(&(3, 1))
        .unwrap()
        .add_all(&[(3, 0), (3, 2)]);
    our_graph.get_targets_mut(&(3, 2)).unwrap().add(&(3, 3));

    let heuristic_gen = |target_0: u8, target_1: u8| {
        move |node: &(u8, u8)| (target_0.abs_diff(node.0) + target_1.abs_diff(node.1)) as usize
    };

    assert_eq!(
        our_graph
            .find_shortest_path(&(0, 0), &(1, 1))
            .unwrap()
            .as_ref(),
        &[(0, 0), (1, 0), (1, 1)]
    );
    assert_eq!(
        our_graph
            .find_shortest_path_a_star(&(0, 0), &(1, 1), heuristic_gen(1, 1))
            .unwrap()
            .as_ref(),
        &[(0, 0), (1, 0), (1, 1)]
    );

    assert_eq!(
        our_graph
            .find_shortest_path(&(0, 0), &(2, 0))
            .unwrap()
            .as_ref(),
        &[(0, 0), (1, 0), (1, 1), (2, 1), (3, 1), (3, 0), (2, 0)]
    );
    assert_eq!(
        our_graph
            .find_shortest_path_a_star(&(0, 0), &(2, 0), heuristic_gen(2, 0))
            .unwrap()
            .as_ref(),
        &[(0, 0), (1, 0), (1, 1), (2, 1), (3, 1), (3, 0), (2, 0)]
    );

    assert_eq!(
        our_graph
            .find_shortest_path(&(0, 0), &(3, 3))
            .unwrap()
            .as_ref(),
        &[(0, 0), (1, 0), (1, 1), (2, 1), (3, 1), (3, 2), (3, 3)]
    );
    assert_eq!(
        our_graph
            .find_shortest_path_a_star(&(0, 0), &(3, 3), heuristic_gen(3, 3))
            .unwrap()
            .as_ref(),
        &[(0, 0), (1, 0), (1, 1), (2, 1), (3, 1), (3, 2), (3, 3)]
    );
}
