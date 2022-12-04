use std::fs::File;
use std::io::prelude::*;
use std::collections::HashMap;
pub fn read_file(path: &str) -> (Vec<Vec<usize>>,HashMap<String, usize>) {
    let preliminary_n_nodes = 1018525;
    let mut result: Vec<Vec<usize>> = Vec::with_capacity(preliminary_n_nodes);
    let file = File::open(path).expect("Could not open file");
    let buf_reader = std::io::BufReader::new(file).lines();
    let mut map: HashMap<String, usize> = HashMap::with_capacity(preliminary_n_nodes);
    let mut i: usize = 0;
    for line in buf_reader {
        let line_str = line.expect("Error reading");
        let v: Vec<&str> = line_str.trim().split('\t').collect();
        let x = v[0];
        let y = v[1];
        let x_ind = *map.entry(x.to_owned()).or_insert(i);
        if x_ind >= i {
            i = x_ind+1;
            result.push(Vec::new());
            assert_eq!(result.len(),i);
            let y_ind = *map.entry(y.to_owned()).or_insert(i);
            if y_ind >= i {
                result.push(Vec::new());
                i = y_ind+1;
            }
            result[x_ind].push(y_ind);
        } else {
            let y_ind = *map.entry(y.to_owned()).or_insert(i);
            if y_ind >= i {
                result.push(Vec::new());
                result[x_ind].push(y_ind);
                i = y_ind+1;
            } else {
                result[x_ind].push(y_ind);
            }
        }
    }
    return (result, map)
}

fn main() {
    let (testa,testb) = read_file("./src/data/CC-Neuron_cci.tsv");
    println!("{:?}",testa[0]);
    println!("{:?}",testa.len());
}
