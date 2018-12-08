use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

#[derive(Debug)]
struct Node {
    children: Vec<Node>,
    metas: Vec<i32>,
}

fn parse_node(data: &Vec<i32>, index: usize) -> (Node, usize) {
    // println!("{}", index);
    let children_number = data[index];
    let meta_number = data[index + 1];
    let mut node = Node{ children: vec![], metas: vec![] };
    let mut current_index = index + 2;
    for _ in 0..children_number {
        let (child_node, next_index) = parse_node(data, current_index);
        node.children.push(child_node);
        current_index = next_index;
    }
    for _ in 0..meta_number {
        node.metas.push(data[current_index]);
        current_index += 1;
    }
    (node, current_index)
}

fn get_total_meta(node: &Node) -> i32 {
    let mut total = node.metas.iter().sum();
    total += node.children.iter().map(|c| get_total_meta(c)).sum::<i32>();
    total
}

fn main() {
    let file = File::open("../input").unwrap();
    // let file = File::open("../example").unwrap();
    let reader = BufReader::new(file);

    let mut data = vec![];
    for _line in reader.lines() {
        let line = _line.unwrap();
        data = line.split_whitespace().map(|s| s.parse::<i32>().unwrap()).collect::<Vec<i32>>();
    }
    // println!("{:?}", data);
    let (node, _index) = parse_node(&data, 0);
    // println!("{:?}", node);
     let total_meta = get_total_meta(&node);
    println!("Result: {}", total_meta);
}
