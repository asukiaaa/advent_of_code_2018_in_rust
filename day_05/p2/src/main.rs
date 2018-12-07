use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

fn get_first_pair_index(chars: &Vec<char>) -> Option<usize> {
    for (i, c) in chars.iter().enumerate() {
        // print!("{}", c);
        let pair_c = if c.is_uppercase() { c.to_ascii_lowercase() } else { c.to_ascii_uppercase() };
        if i+1 != chars.len() && chars[i+1] as char == pair_c {
            return Some(i)
        }
    }
    None
}

fn remove_pairs(target: &mut String) {
    let mut chars = target.as_bytes().iter().map(|&b| b as char).collect::<Vec<char>>();
    loop {
        match get_first_pair_index(&chars) {
            Some(pair_index) => {
                chars.remove(pair_index);
                chars.remove(pair_index);
            },
            None => { break; },
        }
    }
    *target = chars.iter().collect::<String>();
}

fn main() {
    let file = File::open("../input").unwrap();
    // let file = File::open("../example").unwrap();
    let reader = BufReader::new(file);
    let target = String::from(reader.lines().next().unwrap().unwrap());

    let mut min_len = target.len();
    let mut min_c = 'a';
    for _c in 'a' as u8 ..'z' as u8 {
        let this_c = _c as char;
        let mut this_target = target.clone();
        this_target.retain(|c| !(c == this_c || c == this_c.to_ascii_uppercase()));
        remove_pairs(&mut this_target);
        let len = this_target.len();
        println!("{} {}", this_c, len);
        if len < min_len {
            min_len = len;
            min_c = this_c;
        }
    }
    println!("");

    // let targets = (a..z).map()
    // remove_pairs(&mut target);

    println!("Result: {} {}", min_c, min_len);
}
