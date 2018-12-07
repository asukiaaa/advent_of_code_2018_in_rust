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

fn get_executable_tasks(task_dependencies: &HashMap<char, Node>, tasks_to_do: &Vec<char>, done_tasks: &Vec<char>) -> Vec<char> {
    task_dependencies.iter().filter(
        |(t, node)|
        tasks_to_do.contains(t) &&
            node.parents.iter().filter(|t| !done_tasks.contains(t)).count() == 0
    ).map(|(&t, _)| t).collect::<Vec<char>>()
}

fn assign_tasks(worker_tasks: &mut Vec<Option<char>>, executable_tasks: &Vec<char>) {
    let executing_tasks = worker_tasks.iter().filter(|t| t.is_some()).map(|t| t.unwrap()).collect::<Vec<char>>();
    println!("executable {:?}", executable_tasks);
    println!("executing {:?}", executing_tasks);
    let mut assignable_tasks = executable_tasks.iter().filter(|t| !executing_tasks.contains(t)).map(|&c| c).collect::<Vec<char>>();
    assignable_tasks.sort();
    assignable_tasks.reverse();
    println!("assignable {:?}", assignable_tasks);
    for w_task in worker_tasks.iter_mut() {
        if assignable_tasks.len() > 0 &&  w_task.is_none() {
            *w_task = Some(assignable_tasks.pop().unwrap())
        }
    }
}

fn get_steps_for_task(task: char) -> i32 {
    task as i32 - 4
}

fn execute_tasks(worker_tasks: &mut Vec<Option<char>>, worker_steps: &mut Vec<i32>, tasks_to_do: &mut Vec<char>, done_tasks: &mut Vec<char>) {
    println!("worker_tasks {:?}\nworker_steps {:?}", worker_tasks, worker_steps);
    for (i, worker_task) in worker_tasks.iter_mut().enumerate() {
        // println!("{} {:?}", i, worker_task);
        if worker_task.is_none() { continue; }
        let worker_step: &mut i32 = &mut worker_steps[i];
        *worker_step += 1;
        let task = worker_task.unwrap();
        let task_step = get_steps_for_task(task);
        // println!("{:?} {}", worker_task, task_step);
        if *worker_step == task_step {
            *worker_step = 0;
            done_tasks.push(task);
            *worker_task = None;
            let index = tasks_to_do.binary_search(&task).unwrap();
            tasks_to_do.remove(index);
        }
    }
}

fn main() {
    let worker_number = 5;

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
        // println!("{} {}", this_char, child_char);
        task_dependencies.entry(this_char).or_insert(Node::new()).children.push(child_char);
        task_dependencies.entry(child_char).or_insert(Node::new()).parents.push(this_char);
    }
    let mut tasks_to_do = task_dependencies.keys().map(|&c| c).collect::<Vec<char>>();
    let mut done_tasks: Vec<char> = vec![];
    tasks_to_do.sort();

    let mut world_step = 0;
    let mut worker_tasks: Vec<Option<char>> = vec![None; worker_number];
    let mut worker_steps = vec![0; worker_number];
    while !tasks_to_do.is_empty() {
        let executable_tasks = get_executable_tasks(&task_dependencies, &tasks_to_do, &done_tasks);
        // println!("{:?}", executable_tasks);
        assign_tasks(&mut worker_tasks, &executable_tasks);
        execute_tasks(&mut worker_tasks, &mut worker_steps, &mut tasks_to_do, &mut done_tasks);
        world_step += 1;
        // break;
    }
    print!("Done tasks ");
    for t in done_tasks {
        print!("{}", t);
    }
    println!("");
    println!("Result: {}", world_step);
}
