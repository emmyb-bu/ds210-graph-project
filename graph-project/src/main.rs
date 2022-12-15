use std::fs::File;
use std::io::prelude::*;
use std::collections::{HashMap, HashSet};
use rand::Rng;


fn read_string_graph(path: &str) -> (Vec<Vec<usize>>,HashMap<String, usize>) {
    let preliminary_n_nodes = 1018525;
    let mut result: Vec<Vec<usize>> = Vec::with_capacity(preliminary_n_nodes);
    let file = File::open(path).expect("Could not open file");
    let buf_reader = std::io::BufReader::new(file).lines();
    let mut map: HashMap<String, usize> = HashMap::with_capacity(preliminary_n_nodes);
    let mut i: usize = 0;
    for line in buf_reader {
        let line_str = line.expect("Error reading");
        let v: Vec<&str> = line_str.trim().split(' ').collect();
        // let v: Vec<&str> = line_str.trim().split('\t').collect();
        let x = v[0];
        let y = v[1];
        let x_ind = *map.entry(x.to_owned()).or_insert(i);
        if x_ind >= i {
            i = x_ind+1;
            result.push(Vec::new());
            assert_eq!(result.len(),i);
        } 
        let y_ind = *map.entry(y.to_owned()).or_insert(i);
        if y_ind >= i {
            result.push(Vec::new());
            i = y_ind+1;
        }
        result[x_ind].push(y_ind);
        result[y_ind].push(x_ind);
    }
    return (result, map)
}



type SuperNodes = HashMap<usize,HashSet<usize>>;
type SuperEdges = HashMap<usize,HashMap<usize,usize>>;

fn check_for_singletons(graph: &Vec<Vec<usize>>) {
    for x in graph {
        assert!(x.len() > 1, "There is a lone vertex!");
    }
}

// fn drop_singletons(graph: &mut Vec<Vec<usize>>) -> Vec<Vec<usize>> {
//     let mut singletons: Vec<usize> = Vec::new();
//     let mut i = 0;
//     for connections in graph {
//         if connections.len <= 1 
//     }
//     return 
// }

fn karger(graph: &Vec<Vec<usize>>) -> Vec<Vec<usize>> {
    let n_nodes: usize = graph.len();
    // let mut super_nodes: SuperNodes = HashMap::with_capacity(n_nodes);
    // for v in 1..n_nodes {super_nodes.insert(v, HashSet::from([v]));};
    let mut super_edges: SuperEdges = HashMap::with_capacity(n_nodes);
    let mut i: usize = 0;
    for connections in graph {
        super_edges.insert(i, HashMap::from_iter(connections.into_iter().map(|x| (*x,1usize)).collect::<Vec<(usize,usize)>>()));
        i += 1;
    }
    let mut valid_nodes: Vec<usize> = super_edges.clone().into_keys().collect();
    // let mut valid_nodes: Vec<usize> = (0..n_nodes).collect();
    let mut n_valid_nodes = valid_nodes.len();
    let mut count_connections: Vec<usize> = valid_nodes.clone().iter().map(|i| super_edges.get(&i).unwrap().into_iter().map(|(_x,y)| y).sum()).collect();
    assert_eq!(valid_nodes.len(),count_connections.len(),"Screaming and crying!");
    // let mut count_connections: Vec<usize> = super_edges.clone().into_iter().map(|(_i,connections)| connections.into_iter().map(|(_x,y)| y).sum()).collect();
    // let mut max_connections: usize = *count_connections.iter().max().unwrap();
    loop {
        let x_sample_index = rand::thread_rng().gen_range(0..count_connections.iter().sum());
        let mut x_node = 0;
        let mut x_node_index: usize = 0;
        for i in &valid_nodes {
            x_node_index += count_connections[*i];
            if x_node_index >= x_sample_index {
                x_node = *i;
            };
        }
        // let x_node = loop {
        //     // rejection sampling
        //     let proposal_node = &valid_nodes[rand::thread_rng().gen_range(0..valid_nodes.len())];
        //     if (rand::thread_rng().gen::<f32>()) * ((max_connections+1) as f32) < (count_connections[*proposal_node] as f32) {
        //         break *proposal_node;
        //     }
        // };
        println!("generated xnode");
        let y_sample_index: usize = rand::thread_rng().gen_range(0..count_connections[x_node]);
        let mut y_node_index: usize = 0;
        let mut y_node = 0;
        let mut n_xy_connections = 0;
        for (i, connections) in super_edges.get(&x_node).unwrap() {
            y_node_index += connections;
            if y_node_index >= y_sample_index {
                y_node = *i;
                n_xy_connections = *connections;
                break
            }
        }
        // println!("generated ynode");
        let x_connections = super_edges.get(&x_node).unwrap().clone();
        // println!("x connections");
        // let n_xy_connections = super_edges.get(&x_node).unwrap().get(&y_node).unwrap().clone();
        // println!("counts xy connections");
        valid_nodes.retain(|&x| x != x_node);
        // println!("dropped x");
        for i in &valid_nodes {
            // iterate over valid nodes
            if i == &y_node {
                // move x connections to y
                let y_connections = super_edges.get_mut(&y_node).unwrap();
                for (j, n_x_connections) in &x_connections {
                    match y_connections.get_mut(&j) {
                        None => {
                            // if y is not connected to x's connection, create a new edge for every edge between x and y
                            // :3
                            y_connections.insert(*j,*(n_x_connections));
                            // y_connections.insert(*j,(n_x_connections)*(n_xy_connections));
                        },
                        Some(n_y_connections) => {
                            // if y is connected to x's connection, add counts of edges for every edge between x and y
                            *n_y_connections += n_x_connections 
                            // *n_y_connections += n_x_connections * n_xy_connections
                        }
                    }
                    // count_connections[*i] = count_connections[*i]; //+ (n_x_connections);
                    // count_connections[*i] = count_connections[*i] + (n_x_connections)*(n_xy_connections);
                    // println!("{}",count_connections[*i]);
                    // count_connections[*i] = count_connections[*i] - n_x_connections;
                }
            } else if i == &x_node {
                // we're going to ignore the x node because it's being removed from the list of valid nodes
            } else {
                // replace all connections to x with a connection to y
                let these_connections = super_edges.get_mut(&i).unwrap();
                // transfer connections to x to connections to y
                let these_connections_to_x = these_connections.get(&x_node).clone();
                let mut y_connections: usize = 0;
                match these_connections_to_x {
                    None => {},
                    Some(these_connections_to_x) => {
                        y_connections = match these_connections.get(&y_node) {
                            None => {
                                *these_connections_to_x + (n_xy_connections)
                            },
                            Some(these_connections_to_y) => {
                                these_connections_to_y + *these_connections_to_y + (n_xy_connections)
                            }
                        };
                        }
                    }
                    these_connections.insert(y_node,y_connections);
                    these_connections.remove(&x_node);
                }
            }
            // println!("updates nodes");
            // max_connections = *count_connections.iter().max().unwrap();
            // if count_connections[i-1] > max_connections {max_connzections = count_connections[i-1]};
        // }
        if valid_nodes.len() <= 2 {
            break
        } 
        // else if n_valid_nodes == valid_nodes.len() {
        //     break
        // }
        else {
            n_valid_nodes = valid_nodes.len();
            println!("valid nodes = {}",n_valid_nodes)
        }
    }
    return valid_nodes.iter().map(|i| super_edges.get(i).unwrap().clone().into_keys().collect() ).collect()
    // return super_edges.into_values().into_iter().map(|x| x.into_keys().collect()).collect()
    // return super_edges.into_values().into_iter().map(|x| x.into_keys().collect()).collect()
}


fn main() {
    // let graph: Vec<Vec<usize>> = vec![vec![1, 2], vec![1, 3], vec![1, 4], vec![1, 5], vec![2, 3], vec![2, 4], vec![2, 5], vec![3, 4], vec![3, 5], vec![4, 5], vec![6, 7], vec![6, 8], vec![6, 9], vec![6, 10], vec![7, 8], vec![7, 9], vec![7, 10], vec![8, 9], vec![8, 10], vec![9, 10], vec![1, 6], vec![2, 7]];
    // let mew = karger(&graph);
    let (testa,testb) = read_string_graph("./src/data/facebook_combined.txt");
    println!("Data loaded");
    // check_for_singletons(&testa)
    // let mut cts: usize = 0;
    // for x in testa {
    //     if x.len() < 1 {
    //         cts += 1;
    //     }
    // }
    // println!("{}",cts)
    let karger_results = karger(&testa);
    println!("{:?}",karger_results[0]);
    println!("{:?}",karger_results[1]);
    // let test = &testa[0];
    // let test2 = &testa[test[0]];
    // println!("{:?}",test);
    // println!("{:?}",test2);
    // println!("{:?}",&testa[test2[0]]);
    // // let test=testb.iter().find_map(|(key, &val)| if val == 0 { Some(key) } else { None });
    // println!("{:?}",test.unwrap());
    // println!("{:?}",testa.len());
}

