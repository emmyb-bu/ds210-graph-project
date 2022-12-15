use std::collections::{HashMap};

use load::read_string_graph;
use karger::karger;
use karger::check_for_singletons;

mod karger;
mod load;






fn main() {
    let (graph,_map) = read_string_graph("./src/data/facebook_combined.txt");
    println!("Data loaded");
    check_for_singletons(&graph);
    let karger_results = karger(&graph);
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

