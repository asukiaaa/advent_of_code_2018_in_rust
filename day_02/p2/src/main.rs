use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

fn main() {
    let file = File::open("../input").unwrap();
    // let file = File::open("../example").unwrap();
    let reader = BufReader::new(file);
    let mut result: Option<String> = None;

    let lines: Vec<Vec<u8>> = reader.lines().map(|l| l.unwrap().into_bytes()).collect();
    for (index_a, line_a) in lines.iter().enumerate() {
        for line_b in lines[index_a..lines.len()].iter() {
            let mut diff_count = 0;
            let mut diff_index: Option<usize> = None;
            for i in 0..line_a.len() {
                if line_a[i] != line_b[i] {
                    diff_count += 1;
                    diff_index = Some(i);
                    if diff_count > 1 { break; }
                }
            }
            if diff_count == 1 {
                // println!("{} {} {:?} {}",
                //          String::from_utf8(line_a.clone()).unwrap(),
                //          String::from_utf8(line_b.clone()).unwrap(),
                //          diff_index,
                //          line_a[diff_index.unwrap()] as char);
                let mut result_vec = line_a.clone();
                result_vec.remove(diff_index.unwrap());
                result = Some(String::from_utf8(result_vec).unwrap());
                break;
            }
        }
        if result != None { break; }
    }
    match result {
        Some(v) => println!("Result: {}", v),
        None => {}
    }
}
