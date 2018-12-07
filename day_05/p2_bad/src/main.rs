use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

fn get_next_index(current_index: usize, deleted_indexes: &Vec<usize>) -> usize {
    let mut i = current_index + 1;
    loop {
        if deleted_indexes.contains(&i) { i += 1; } else { return i; }
    }
}

fn scan_pair_to_update_deleted_indexes(deleted_indexes: &mut Vec<usize>, target: &String) {
    let chars = target.as_bytes().iter().map(|&b| b as char).collect::<Vec<char>>();
    let mut i = 0;
    let initial_len = deleted_indexes.len();
    loop {
        let c = chars[i];
        let pair_c = if c.is_uppercase() { c.to_ascii_lowercase() } else { c.to_ascii_uppercase() };
        let mut next_i = get_next_index(i, &deleted_indexes);
        if next_i != chars.len() && chars[next_i] as char == pair_c {
            deleted_indexes.append(&mut vec![i, next_i]);
            i = next_i;
            next_i = get_next_index(next_i, &deleted_indexes);
        }
        i = next_i;
        if i >= chars.len() - 1 { break; }
    }
    // println!("{}", deleted_indexes.len());
    if deleted_indexes.len() != initial_len {
        scan_pair_to_update_deleted_indexes(deleted_indexes, target)
    }
}

fn main() {
    let file = File::open("../input").unwrap();
    // let file = File::open("../example").unwrap();
    let reader = BufReader::new(file);
    let target = String::from(reader.lines().next().unwrap().unwrap());
    let mut deleted_indexes = vec![];
    scan_pair_to_update_deleted_indexes(&mut deleted_indexes, &target);
    println!("Result: {}", target.len() - deleted_indexes.len());
}
