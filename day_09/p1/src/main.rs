use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

fn get_highest_score(player_number: usize, worth: usize) -> usize {
    let mut marbles = vec![0];
    let mut player_points = vec![0; player_number];
    let mut current_index = 0;
    for marble in 1..worth+1 {
        let player_index = (marble - 1) % player_number;
        // println!("{} {}", marble, player_index);
        if marble % 23 == 0 {
            if current_index >= 7 {
                current_index -= 7;
            } else {
                current_index = marbles.len() + current_index - 7;
            }
            player_points[player_index] += marble;
            player_points[player_index] += marbles.remove(current_index);
            // println!("player_index {}", player_index);
        } else {
            current_index += 2;
            if current_index > marbles.len() {
                current_index = current_index - marbles.len();
            }
            // println!("current_index {}", current_index);
            marbles.insert(current_index, marble);
        }
    }
    // println!("{:?}", marbles);
    // println!("{:?}", player_points);
    player_points.sort();
    *player_points.last().unwrap()
}

fn main() {
    let debug = false;
    let file_name = if debug { "../example" } else { "../input" };
    let file = File::open(file_name).unwrap();
    let reader = BufReader::new(file);

    for line_result in reader.lines() {
        let line = line_result.unwrap();
        let d = line.split_whitespace().into_iter().collect::<Vec<&str>>();
        let player_number = d[0].parse::<usize>().unwrap();
        let worth = d[6].parse::<usize>().unwrap();
        println!("{} {} {}", player_number, worth, get_highest_score(player_number, worth));
    }

    // println!("Result: {}", );
}
