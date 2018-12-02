use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

fn check_available(count: i32, twice: &mut bool, tird: &mut bool) {
    if count == 2 {
        *twice = true;
    } else if count == 3 {
        *tird = true;
    }
}

fn main() {
    let file = File::open("../input").unwrap();
    // let file = File::open("../example").unwrap();
    let reader = BufReader::new(file);
    let mut twice_count = 0;
    let mut tird_count = 0;

    for _line in reader.lines() {
        let line = _line.unwrap();
        let mut chars = line.into_bytes();
        chars.sort_by(|a, b| a.cmp(b));
        let mut current_char = chars[0];
        let mut current_char_count = 1;
        let mut twice_available = false;
        let mut tird_available = false;
        for c in chars[1..chars.len()].iter() {
            if *c == current_char {
                current_char_count += 1;
            } else {
                current_char = *c;
                check_available(current_char_count, &mut twice_available, &mut tird_available);
                current_char_count = 1;
            }
        }
        check_available(current_char_count, &mut twice_available, &mut tird_available);
        println!("{} {}", twice_available, tird_available);
        if twice_available { twice_count += 1; }
        if tird_available { tird_count += 1; }
    }
    println!("Result: {} * {} = {}", twice_count, tird_count, twice_count * tird_count);
}

// not 6175
// not 12597
