use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

fn get_first_pair(target: &String) -> Option<usize> {
    let chars = target.as_bytes().iter().map(|&b| b as char).collect::<Vec<char>>();
    for (i, c) in chars.iter().enumerate() {
        // print!("{}", c);
        let pair_c = if c.is_uppercase() { c.to_ascii_lowercase() } else { c.to_ascii_uppercase() };
        if i+1 != chars.len() && chars[i+1] as char == pair_c {
            return Some(i)
        }
    }
    None
}

fn main() {
    let file = File::open("../input").unwrap();
    // let file = File::open("../example").unwrap();
    let reader = BufReader::new(file);
    let mut target = String::from(reader.lines().next().unwrap().unwrap());

    loop {
        match get_first_pair(&target) {
            Some(pair_index) => {
                target.remove(pair_index);
                target.remove(pair_index);
            },
            None => { break; },
        }
    }

    println!("Result: {}", target.len());
}
