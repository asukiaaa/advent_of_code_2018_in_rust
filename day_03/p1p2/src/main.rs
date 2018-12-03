use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

#[derive(Debug)]
pub struct Area {
    id: i32,
    top: i32,
    left: i32,
    width: i32,
    height: i32,
}

fn overwrap_line(this_p: i32, this_len: i32, other_p: i32, other_len: i32) -> Option<(i32, i32)> {
    if this_p < other_p {
        if this_p + this_len <= other_p {
            return None
        } else {
            return Some((other_p, this_p + this_len - other_p))
        }
    } else {
        if other_p + other_len <= this_p {
            return None
        } else {
            return Some((this_p, other_p + other_len - this_p))
        }
    }
}

impl Area {
    fn overwrap(&self, other: & Area) -> Option<Area> {
        let overwrap_th = overwrap_line(self.top, self.height, other.top, other.height);
        if overwrap_th == None { return None }
        let overwrap_lw = overwrap_line(self.left, self.height, other.left, other.height);
        if overwrap_lw == None { return None }
        let (top, height) = overwrap_th.unwrap();
        let (left, width) = overwrap_lw.unwrap();
        return Some(Area{ id: 0, top, left, height, width })
    }
}

fn main() {
    let file = File::open("../input").unwrap();
    // let file = File::open("../example").unwrap();
    let reader = BufReader::new(file);
    let mut claims:Vec<Area> = vec![];
    let mut overwrap_points:Vec<Vec<i32>> = vec![vec![0]];
    let mut overwrap_size = 0;
    let mut not_overwrap_ids = vec![];

    for _line in reader.lines() {
        let line = _line.unwrap();
        let mut iter = line.split_whitespace();
        let mut id_str = String::from(iter.next().unwrap());
        id_str.remove(0);
        let id = id_str.parse::<i32>().unwrap();
        iter.next(); // skip @
        let mut position = String::from(iter.next().unwrap());
        position.pop(); // remove ':'
        let lt: Vec<i32> = position.split(',').map(|p| p.parse::<i32>().unwrap()).collect();
        let left = lt[0];
        let top = lt[1];
        let mut size = String::from(iter.next().unwrap());
        let wh: Vec<i32> = size.split('x').map(|s| s.parse::<i32>().unwrap()).collect();
        let width = wh[0];
        let height = wh[1];
        // println!("{} {},{} {}x{}", id, top, left, width, height);
        claims.push(Area{id, top, left, width, height});
    }

    for area in claims.iter() {
        let mut overwraps = false;
        // println!("{:?}", area);
        let bottom = area.top + area.height;
        let right = area.left + area.width;
        while overwrap_points[0].len() < right as usize {
            for line in overwrap_points.iter_mut() {
                let len = line.len();
                line.append(&mut vec![0; len]);
            }
        }
        while overwrap_points.len() < bottom as usize {
            let len = overwrap_points[0].len();
            for _ in 0..overwrap_points.len() {
                overwrap_points.push(vec![0; len]);
            }
        }
        for y in area.top..bottom {
            for x in area.left..right {
                // println!("{} {}", x,y);
                let current_value = overwrap_points[y as usize][x as usize];
                overwrap_points[y as usize][x as usize] =
                    if current_value == 0 {
                        area.id
                    } else {
                        match not_overwrap_ids.binary_search(&current_value) {
                            Ok(index) => { not_overwrap_ids.remove(index); },
                            Err(_) => {},
                        }
                        overwraps = true;
                        -1
                    };
            }
        }
        if !overwraps {
            not_overwrap_ids.push(area.id);
        }
    }

    // for i in 0..overwrap_points.len() {
    //     println!("{:?}", overwrap_points[i]);
    // }
    for y in 0..overwrap_points.len() {
        for x in 0..overwrap_points[0].len() {
            if overwrap_points[y][x] == -1 {
                overwrap_size += 1;
            }
        }
    }
    println!("Part1 result: {}", overwrap_size);
    println!("Part2 result: {:?}", not_overwrap_ids);
}

// part1
// not 1273
// not 144178
// not 112000
