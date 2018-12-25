use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

fn get_node_diff(n1: &Vec<i32>, n2: &Vec<i32>) -> usize {
    let n_len = n1.len();
    let mut diff = 0;
    for i in 0..n_len {
        diff += (n1[i] - n2[i]).abs() as usize;
    }
    diff
}

fn main() {
    let mut debug = true;
    debug = false;
    let file_name = if debug { "../example" } else { "../input" };
    let file = File::open(file_name).unwrap();
    let reader = BufReader::new(file);

    let mut nodes: Vec<Vec<i32>> = vec![];
    for line_result in reader.lines() {
        let line = line_result.unwrap();
        nodes.push(line.trim().split(',').into_iter().map(|v| v.parse::<i32>().unwrap()).collect::<Vec<i32>>());
    }

    let mut constellations: Vec<Vec<usize>> = vec![];
    let node_len = nodes.len();
    let far_len = 4;
    for node_i in 0..node_len {
        let mut near_i: Option<usize> = None;
        for compare_i in 0..node_i {
            if get_node_diff(&nodes[node_i], &nodes[compare_i]) < far_len {
                near_i = Some(compare_i);
                break;
            }
        }
        match near_i {
            Some(i) => {
                let cons_i = constellations.iter().position(|c| c.contains(&i)).unwrap();
                constellations[cons_i].push(node_i);
            },
            None => { constellations.push(vec![node_i]); },
        }
    }

    // println!("constellations: {:?}", constellations);
    loop {
        let mut changed = false;
        let cons_len = constellations.len();
        for cons_i in 0..cons_len {
            let mut near_cons_i: Option<usize> = None;
            {
                let this_cons = &constellations[cons_i];
                for compare_cons_i in 0..cons_i {
                    for n in constellations[compare_cons_i].iter() {
                        for this_n in this_cons.iter() {
                            if get_node_diff(&nodes[*n], &nodes[*this_n]) < far_len {
                                near_cons_i = Some(compare_cons_i);
                                break;
                            }
                        }
                        if near_cons_i.is_some() { break; }
                    }
                    if near_cons_i.is_some() { break; }
                }
            }
            match near_cons_i {
                Some(near_i) => {
                    let mut this_cons = constellations.remove(cons_i);
                    constellations[near_i].append(&mut this_cons);
                    changed = true;
                    break;
                },
                None => {}
            }
        }
        if !changed {
            break;
        }
    }
    // println!("nodes: {:?}", nodes);
    println!("constellations: {:?}", constellations);
    println!("Result: {}", constellations.len());
}
