use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

#[derive(Debug)]
struct Coordinate {
    x: usize,
    y: usize,
}

#[derive(Clone, Debug)]
struct CoordInfo {
    nearests: Vec<usize>,
    distance: usize,
}

fn print_area(area: &Vec<Vec<Option<CoordInfo>>>) {
    for row in area {
        for c_option in row {
            let c_char = match c_option {
                Some(c) => {
                    if c.nearests.len() == 1 {
                        let c_char = ('a' as u8 + c.nearests[0] as u8) as char;
                        if c.distance == 0 {
                            c_char.to_ascii_uppercase()
                        } else {
                            c_char
                        }
                        // c.distance.to_string().as_bytes()[0] as char
                    } else {
                        '.'
                    }
                },
                None => { '.' }
            };
            print!("{}", c_char);
        }
        println!("");
    }
}

fn update_distance_area(area: &mut Vec<Vec<Option<CoordInfo>>>, id: usize, coordinate: &Coordinate, distance: usize, origin_x: usize, origin_y: usize) -> bool {
    let mut expanded = false;
    for d in 0..distance+1 {
        // println!("{}", d);
        let d_diff = distance - d;
        for (x, y) in vec![(coordinate.y + origin_x + d,
                            coordinate.x + origin_y + d_diff),
                           (coordinate.y + origin_x - d,
                            coordinate.x + origin_y + d_diff),
                           (coordinate.y + origin_x + d,
                            coordinate.x + origin_y - d_diff),
                           (coordinate.y + origin_x - d,
                            coordinate.x + origin_y - d_diff)] {
            let mut c_option = &mut area[y][x];
            match c_option.clone() {
                Some(mut c) => {
                    if c.distance > distance {
                        c.distance = distance;
                        c.nearests = vec![id];
                        *c_option = Some(c);
                        expanded = true;
                    } else if c.distance == distance && !c.nearests.contains(&id) {
                        c.nearests.push(id);
                        *c_option = Some(c);
                    }
                },
                None => {
                    *c_option = Some(CoordInfo{ nearests: vec![id], distance: distance });
                    expanded = true;
                },
            }
        }
    }
    expanded
}

fn last_or_first_area_filled(area: &Vec<Vec<Option<CoordInfo>>>) -> bool {
    let len_y = area.len();
    let len_x = area.first().unwrap().len();
    for (row_i, row) in area.iter().enumerate() {
        for (c_i, c) in row.iter().enumerate() {
            if !(c_i == 0 || c_i == len_x - 1 || row_i == 0 || row_i == len_y - 1) {
                continue;
            }
            if c.is_some() {
                return true;
            }
        }
    }
    false
}

fn main() {
    let file = File::open("../input").unwrap();
    // let file = File::open("../example").unwrap();
    let reader = BufReader::new(file);
    let mut coordinates: Vec<Coordinate> = vec![];
    let mut max_x = 0;
    let mut max_y = 0;

    for _line in reader.lines() {
        let line = _line.unwrap();
        let mut d = line.split(", ");
        let x = d.next().unwrap().parse::<usize>().unwrap();
        let y = d.next().unwrap().parse::<usize>().unwrap();
        if x > max_x { max_x = x; }
        if y > max_y { max_y = y; }
        coordinates.push(Coordinate{x, y});
    }

    let origin_x = max_x;
    let origin_y = max_y;
    let mut area: Vec<Vec<Option<CoordInfo>>> = vec![vec![None; origin_x * 3]; origin_y * 3];
    let mut distance = 0;
    let mut expanded_ids = vec![];
    loop {
        if last_or_first_area_filled(&area) {
            // expand_area(&mut area, &mut origin_x, &mut origin_y);
            break;
        }
        expanded_ids = vec![];
        for (id, coordinate) in coordinates.iter().enumerate() {
            let expanded = update_distance_area(&mut area, id, coordinate, distance, origin_x, origin_y);
            if expanded { expanded_ids.push(id); }
        }
        distance += 1;
        // print_area(&area);
    }
    let infinite_ids = expanded_ids;

    let mut coords_size = vec![0; coordinates.len()];
    for row in area {
        for c_option in row {
            match c_option {
                Some(c) => {
                    // println!("{:?}", c);
                    if c.nearests.len() == 1 {
                        let id = c.nearests[0];
                        if !infinite_ids.contains(&id) {
                            coords_size[id] += 1;
                        }
                    }
                },
                None => {},
            }
        }
    }
    let mut max_size = 0;
    for size in coords_size {
        if size > max_size {
            max_size = size;
        }
    }

    println!("Result: {}", max_size);
}
