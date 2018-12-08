use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

#[derive(Debug, Clone)]
struct Coordinate {
    x: usize,
    y: usize,
}

impl Coordinate {
    fn get_distance(&self, x: i32, y: i32) -> usize {
        (self.x as i32 - x).wrapping_abs() as usize + (self.y as i32 - y).wrapping_abs() as usize
    }
}

fn main() {
    let debug = false;
    let less_than;
    let file;
    if debug {
        file = File::open("../example").unwrap();
        less_than = 32;
    } else {
        file = File::open("../input").unwrap();
        less_than = 10000;
    }

    let reader = BufReader::new(file);
    let mut coordinates: Vec<Coordinate> = vec![];
    let mut max_x: Option<usize> = None;
    let mut max_y: Option<usize> = None;

    for _line in reader.lines() {
        let line = _line.unwrap();
        let mut d = line.split(", ");
        let x = d.next().unwrap().parse::<usize>().unwrap();
        let y = d.next().unwrap().parse::<usize>().unwrap();
        if max_x.is_none() || x > max_x.unwrap() {
            max_x = Some(x);
        }
        if max_y.is_none() || y > max_y.unwrap() {
            max_y = Some(y);
        }
        coordinates.push(Coordinate{x, y});
    }

    let origin_x = max_x.unwrap() as i32;
    let origin_y = max_y.unwrap() as i32;
    let area_x_len = origin_x * 3;
    let area_y_len = origin_y * 3;

    let mut less_than_xys = vec![];
    for area_x in 0 .. area_x_len {
        for area_y in 0 .. area_y_len {
            let x = area_x - origin_x;
            let y = area_y - origin_y;
            let mut points = 0;
            for c in coordinates.iter() {
                points += c.get_distance(x, y);
                if points >= less_than { break; }
            }
            if points < less_than {
                less_than_xys.push((x, y));
            }
        }
    }

    println!("Result: {}", less_than_xys.len());
}
