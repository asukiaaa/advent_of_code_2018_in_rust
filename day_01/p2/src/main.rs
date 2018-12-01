use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

fn main() -> std::io::Result<()> {
    let mut freq = 0;
    let file = File::open("../input")?;
    let reader = BufReader::new(file);
    let mut reached_numbers = vec![freq];
    let diffs: Vec<i32> = reader.lines().map(|line| line.unwrap().parse::<i32>().unwrap()).collect();
    let mut first_twice_reached_number:Option<i32> = None;

    for diff in diffs.iter() {
        freq += diff;
        {
            let index = reached_numbers.iter().find(|&&n| n == freq);
            match index {
                Some(_v) => {
                    first_twice_reached_number = Some(freq);
                    break;
                },
                None => {},
            }
        }
        reached_numbers.push(freq);
    }

    let mut number_pairs: Vec<Vec<i32>> = vec![];
    if first_twice_reached_number == None {
        for (index_a, number_a) in reached_numbers.iter().enumerate() {
            for (_index_b, number_b) in reached_numbers[index_a + 1..reached_numbers.len()].iter().enumerate() {
                let index_b = index_a + _index_b + 1;
                let diff_i = number_b - number_a;
                let diff_div_freq = diff_i as f32 / freq as f32;
                if diff_i > 0 && diff_i >= freq && diff_div_freq % 1.0 == 0.0 && index_b != reached_numbers.len() - 1 {
                    // print!("{} {} {} {}\n", number_a, number_b, index_b, diff_div_freq);
                    number_pairs.push(vec![*number_a, *number_b, index_a as i32, diff_div_freq as i32]);
                }
            }
        }
    }
    number_pairs.sort_by(|a, b| a[2].cmp(&b[2]));
    number_pairs.sort_by(|a, b| a[3].cmp(&b[3]));
    // print!("{:?}\n", number_pairs);
    print!("Result: {}\n", number_pairs[0][1]);
    // print!("{} {}\n", reached_numbers.len(), freq);

    Ok(())
}
