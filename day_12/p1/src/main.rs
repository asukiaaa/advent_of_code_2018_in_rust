use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

fn main() {
    let mut debug = false;
    // debug = true;
    let file_name = if debug { "../example" } else { "../input" };
    let file = File::open(file_name).unwrap();
    let reader = BufReader::new(file);
    let mut state: String = "".to_string();
    let mut patterns: Vec<(String, char)> = vec![];
    for line_result in reader.lines() {
        let line = line_result.unwrap();
        let data = line.split_whitespace().collect::<Vec<&str>>();
        if state == "" {
            state = data[2].to_string();
        } else if data.len() > 0 {
            patterns.push((data[0].to_string(), data[2].to_string().into_bytes()[0] as char));
        }
    }
    // println!("{:?}", patterns);
    let pattern_len = patterns[0].0.len();
    let blank_pattern = vec!['.'; pattern_len].iter().collect::<String>();
    let mut zero_index = 0;
    for _ in 0..20 {
        if !state.starts_with(&blank_pattern) {
            state.insert_str(0, &blank_pattern);
            zero_index += pattern_len;
            // println!("{}", zero_index);
        }
        if !state.ends_with(&blank_pattern) {
            state.push_str(&blank_pattern);
        }
        println!("{:?}", state);
        let mut new_state = vec!['.'; pattern_len/2];
        for begin_index in 0..state.len()-pattern_len {
            let target = &state[begin_index..begin_index+pattern_len];
            let mut this_pattern = '.';
            for pattern in patterns.iter() {
                if target == pattern.0 {
                    this_pattern = pattern.1;
                    break;
                }
            }
            new_state.push(this_pattern);
        }
        state = new_state.iter().collect::<String>();
        // break;
    }
    let mut amount_points:i32 = 0;
    // println!("{}", zero_index);
    for (i, &c_int) in state.as_bytes().iter().enumerate() {
        if c_int as char == '#' {
            let point = i as i32 - zero_index as i32;
            // println!("{}", point);
            amount_points += point;
        }
    }
    println!("Result: {:?}", amount_points);
}
