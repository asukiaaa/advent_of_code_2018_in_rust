use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

fn main() -> std::io::Result<()> {
    let mut freq = 0;
    let file = File::open("../input")?;
    let reader = BufReader::new(file);

    for line in reader.lines() {
        freq += line?.parse::<i32>().unwrap();
    }
    print!("Result: {}\n", freq);
    Ok(())
}
