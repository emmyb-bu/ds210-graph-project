use load::read_string_graph;
use karger::karger;

mod karger; // karger and tests
mod load; // loading dataset 

fn main() {
    let (graph,_map) = read_string_graph("./src/data/facebook_combined.txt");
    println!("Data loaded");
    let karger_results = karger(&graph);
    println!("{:?}",karger_results[0]);
    println!("{:?}",karger_results[1]);
}

