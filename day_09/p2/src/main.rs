use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::process::exit;

const POINT_STEP: usize = 23;

fn get_player_index(marble: usize, player_number: usize) -> usize {
    marble % player_number
}

fn pass_one_step(marbles: &mut Vec<usize>, player_points: &mut Vec<usize>, current_index: &mut usize, marble: &mut usize) {
    let player_index = get_player_index(*marble, player_points.len());
    // print!("{}\r", marble as f32 / worth as f32);
    // print!("{} {}\r", marble, player_index);
    if *marble % POINT_STEP == 0 {
        if *current_index >= 7 {
            *current_index -= 7;
        } else {
            *current_index = marbles.len() + *current_index - 7;
        }
        let removed = marbles.remove(*current_index);
        // println!("point {} + {}", *marble, removed);
        player_points[player_index] += *marble + removed;
        // println!("points {}", points);
    } else {
        *current_index += 2;
        if *current_index > marbles.len() {
            *current_index = *current_index - marbles.len();
        }
        // println!("current_index {}", current_index);
        marbles.insert(*current_index, *marble);
    }
    // println!("{:?}", marbles);
    *marble += 1;
}

/*
  18-23 0-7
      4      9     2              10              5              11
  17, -, 18, x 19, -, 24, 20, 25, --, 26, 21, 27, -, 28, 22, 29, --, 30
  0       1     2      7   3   8       9   4  10     11   5  12      13

  mable % 23 == 17
  [17]zabcd efghijklmnop

  mable % 23 == 17 to marble & 23 == 29 - 23 = 6
  [17]z[18][19]b[24][20][25]c[21][27]d[28][22][29] efghi
  get_point a + [23] for i = (marble + 5) % user_number

  mable % 23 == 17 to marble & 23 == 40 - 23 = 17
  [17]z[18][19]b[24][20][25]c[21][27]d[28][22][29]e[30]f[31]g[32]h[33]i[34]j[35]k[36]l[37]m[38]n[39]o[40]p..
  */

fn get_and_inc(marbles: &mut Vec<usize>, index: &mut usize) -> usize {
    let v = marbles[*index];
    *index += 1;
    v
}

const LOOP_STRT_STEP: usize = 17;
fn pass_one_loop(marbles: &mut Vec<usize>, player_points: &mut Vec<usize>, current_index: &mut usize, marble: &mut usize, worth: usize) -> bool {
    let player_number = player_points.len();
    let mut new_marbles: Vec<usize> = marbles.clone();
    let mut left_marbles = new_marbles.split_off(*current_index+1);
    let left_marbles_len = left_marbles.len();
    let mut left_marbles_index = 0;
    new_marbles = new_marbles.clone();
    left_marbles = left_marbles.clone();
    let mut new_player_points = player_points.clone();
    if left_marbles_len < POINT_STEP || *marble % POINT_STEP != LOOP_STRT_STEP || worth - *marble < POINT_STEP { return false }
    // println!("start process");
    loop {
        if left_marbles_len - left_marbles_index < POINT_STEP || worth - *marble < POINT_STEP{ break }
        // println!("before one loop");
        // println!("marble: {} {}", *marble, *marble % POINT_STEP);
        // println!("old: {:?}", marbles);
        // println!("new: {:?}", new_marbles);
        // println!("left: {:?}", left_marbles);
        if *marble % POINT_STEP == LOOP_STRT_STEP {
            // println!("append");
            let mut a: Vec<usize> = vec![];
            a.push(get_and_inc(&mut left_marbles, &mut left_marbles_index));
            a.push(*marble);
            a.push(get_and_inc(&mut left_marbles, &mut left_marbles_index)); // z
            let removed = get_and_inc(&mut left_marbles, &mut left_marbles_index);
            a.push(*marble + 1);
            a.push(*marble + 2);
            a.push(get_and_inc(&mut left_marbles, &mut left_marbles_index)); // b
            a.push(*marble + 7);
            a.push(*marble + 3);
            a.push(*marble + 8);
            a.push(get_and_inc(&mut left_marbles, &mut left_marbles_index)); // c
            a.push(*marble + 9);
            a.push(*marble + 4);
            a.push(*marble + 10);
            a.push(get_and_inc(&mut left_marbles, &mut left_marbles_index)); // d
            a.push(*marble + 11);
            a.push(*marble + 5);
            a.push(*marble + 12);
            // println!("left: {:?}", left_marbles);

            let marble23 = *marble + 6;
            // println!("point {} + {}", marble23, removed);
            new_player_points[get_player_index(marble23, player_number)] += marble23 + removed;
            *marble += 13;

            // e[30] to n[39]
            while *marble % POINT_STEP != LOOP_STRT_STEP {
                a.push(get_and_inc(&mut left_marbles, &mut left_marbles_index));
                a.push(*marble);
                *marble += 1;
                // println!("marble: {} {}", *marble, *marble % POINT_STEP);
            }
            *current_index += a.len();
            new_marbles.append(&mut a);
            print!("progress: {:?}\r", *marble as f32 / worth as f32);
        } else {
            println!("error exit because not [marble % POINT_STEP == 17]");
            exit(1);
        }
        // println!("marble: {} {}", *marble, *marble % POINT_STEP);
        // println!("old: {:?}", marbles);
        // println!("new: {:?}", new_marbles);
        // println!("left: {:?}", left_marbles);
        // println!("finish one loop");
    }
    // println!("end process");
    loop {
        new_marbles.push(get_and_inc(&mut left_marbles, &mut left_marbles_index));
        if left_marbles_index == left_marbles_len { break; }
    }
    *marbles = new_marbles;
    *player_points = new_player_points;
    true
}

fn get_highest_score(player_number: usize, worth: usize) -> usize {
    let mut marbles = vec![0];
    let mut player_points = vec![0; player_number];
    let mut current_index = 0;
    let mut marble = 1;
    loop {
        if false { // debug true
            pass_one_step(&mut marbles, &mut player_points, &mut current_index, &mut marble);
        } else {
            if pass_one_loop(&mut marbles, &mut player_points, &mut current_index, &mut marble, worth) {
                // println!("progress: {:?}", marble as f32 / worth as f32);
                // println!("test exit");
                // exit(1);
            } else {
                pass_one_step(&mut marbles, &mut player_points, &mut current_index, &mut marble);
            }
        }
        if marble >= worth+1 { break; }

        if marble > 400 {
            // println!("{:?}", marbles);
            // println!("test exit");
            // exit(1);
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
    let times = if debug { 1 } else { 100 };
    let file = File::open(file_name).unwrap();
    let reader = BufReader::new(file);

    for line_result in reader.lines() {
        let line = line_result.unwrap();
        let d = line.split_whitespace().into_iter().collect::<Vec<&str>>();
        let player_number = d[0].parse::<usize>().unwrap();
        let worth = d[6].parse::<usize>().unwrap();
        let result = get_highest_score(player_number, worth * times);
        println!("{} {} Result: {}", player_number, worth, result);
        if d.len() > 8 {
            let answer = d[11].parse::<usize>().unwrap();
            if result != answer {
                println!("The result is wring. Answer is {}", answer);
                exit(1);
            }
        }
        // break;
    }

    // println!("Result: {}", );
}
