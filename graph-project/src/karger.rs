use std::collections::{HashMap};
use rand::Rng;

type SuperEdges = HashMap<usize,HashMap<usize,u128>>;
type Graph = Vec<Vec<usize>>;

pub fn karger(graph: &Graph) -> Graph {
    let n_nodes: usize = graph.len();
    let mut super_edges: SuperEdges = HashMap::with_capacity(n_nodes);
    let mut i: usize = 0;
    for connections in graph {
        super_edges.insert(i, HashMap::from_iter(connections.into_iter().map(|x| (*x,1u128)).collect::<Vec<(usize,u128)>>()));
        i += 1;
    }
    let mut valid_nodes: Vec<usize> = super_edges.clone().into_keys().collect();
    let mut n_valid_nodes = valid_nodes.len();
    loop {
        let count_connections: Vec<u128> = valid_nodes.clone().iter().map(|i| super_edges.get(&i).unwrap().clone().into_values().sum()).collect();
        let total_connections = count_connections.iter().sum();
        let mut x_sample_index = 0;
        if total_connections != 0 {
            x_sample_index = rand::thread_rng().gen_range(0..total_connections)
        }
        let mut x_node = 0;
        let mut x_node_index: u128 = 0;
        for i in 0..valid_nodes.len() {
            x_node_index += count_connections[i];
            if x_node_index >= x_sample_index {
                x_node = valid_nodes[i];
                break
            };
        }
        let count_x_connections = count_connections[valid_nodes.iter().position(|&r| r == x_node).unwrap()];
        let mut y_node_index: u128 = 0;
        let mut y_node = 0;
        let mut n_xy_connections = 0;
        if count_x_connections != 0 {
            let y_sample_index: u128 = rand::thread_rng().gen_range(0..count_x_connections);
            for (i, connections) in super_edges.get(&x_node).unwrap() {
                y_node_index += connections;
                if y_node_index >= y_sample_index {
                    y_node = *i;
                    n_xy_connections = *connections;
                    break
                }
            }
       }
        let x_connections = super_edges.get(&x_node).unwrap().clone();
        valid_nodes.retain(|&x| x != x_node);
        for i in &valid_nodes {
            if i == &y_node {
                let y_connections = super_edges.get_mut(&y_node).unwrap();
                for (j, n_x_connections) in &x_connections {
                    match y_connections.get_mut(&j) {
                        None => {
                            y_connections.insert(*j,*(n_x_connections));
                        },
                        Some(n_y_connections) => {
                            *n_y_connections += n_x_connections 
                        }
                    }
                }
            } else if i == &x_node {} else {
                let these_connections = super_edges.get_mut(&i).unwrap();
                let these_connections_to_x = these_connections.get(&x_node).clone();
                let mut to_y_connections: u128 = 0;
                match these_connections_to_x {
                    None => {},
                    Some(these_connections_to_x) => {
                        to_y_connections = match these_connections.get(&y_node) {
                            None => {
                                *these_connections_to_x + (n_xy_connections)
                            },
                            Some(these_connections_to_y) => {
                                *these_connections_to_y + (n_xy_connections)
                            }
                        };
                        }
                    }
                    these_connections.insert(y_node,to_y_connections);
                    these_connections.remove(&x_node);
                }
            }
            super_edges.get_mut(&y_node).unwrap().remove(&y_node);
        if valid_nodes.len() <= 2 {
            break
        } else {
            n_valid_nodes = valid_nodes.len();
            println!("valid nodes = {}",n_valid_nodes)
        }
    }
    return valid_nodes.iter().map(|i| super_edges.get(i).unwrap().clone().into_keys().collect() ).collect()
}

#[test]
fn cut_a_simple_graph() {
    // test if just one iteration actually performs a contraction
    let example_graph = vec![
        vec![1],
        vec![0]
    ];
    let karger_results = karger(&example_graph);
    assert_eq!(vec![vec![0]],karger_results);
}
