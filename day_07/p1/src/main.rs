use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::collections::HashMap;

struct Node {
    parents: Vec<char>,
    children: Vec<char>,
}

impl Node {
    fn new() -> Node {
        Node { parents: vec![], children: vec![] }
    }
}

fn main() {
    let file = File::open("../input").unwrap();
    // let file = File::open("../example").unwrap();
    let reader = BufReader::new(file);
    let mut task_dependencies: HashMap<char, Node> = HashMap::new();
    for _line in reader.lines() {
        let line = _line.unwrap();
        let mut data = line.split_whitespace().collect::<Vec<&str>>();
        //println!("{:?} {:?}", data[1], data[7]);
        let this_char = data[1].as_bytes()[0] as char;
        let child_char = data[7].as_bytes()[0] as char;
        println!("{} {}", this_char, child_char);
        task_dependencies.entry(this_char).or_insert(Node::new()).children.push(child_char);
        task_dependencies.entry(child_char).or_insert(Node::new()).parents.push(this_char);
    }
    let mut tasks_to_do = task_dependencies.keys().map(|&c| c).collect::<Vec<char>>();
    let mut done_tasks: Vec<char> = vec![];
    tasks_to_do.sort();
    while !tasks_to_do.is_empty() {
        for task in tasks_to_do.clone() {
            // print!("{}", task);
            let parent_tasks = &task_dependencies.get(&task).unwrap().parents;
            let parent_tasks_to_do = parent_tasks.iter().filter(|t| tasks_to_do.binary_search(t).is_ok()).map(|&c| c).collect::<Vec<char>>();
            println!("{} {:?}", task, parent_tasks_to_do);
            if parent_tasks_to_do.is_empty() {
                let index = tasks_to_do.binary_search(&task).unwrap();
                tasks_to_do.remove(index);
                done_tasks.push(task);
                break;
            }
        }
        // break;
    }
    print!("Result: ");
    for t in done_tasks {
        print!("{}", t);
    }
    println!("");
}

// not ZPHISRYMTGNCJOUEQWBKFLVAXD
