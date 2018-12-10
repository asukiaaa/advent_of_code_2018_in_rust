extern crate regex;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use regex::Regex;

struct Point {
    p_x: i32,
    p_y: i32,
    v_x: i32,
    v_y: i32,
}

fn dec_points(points: &mut Vec<Point>) {
    for point in points {
        point.p_x -= point.v_x;
        point.p_y -= point.v_y;
    }
}

fn inc_points(points: &mut Vec<Point>) {
    for point in points {
        point.p_x += point.v_x;
        point.p_y += point.v_y;
    }
}

fn get_min_max(points: &Vec<Point>) -> (i32, i32, i32, i32) {
    let mut min_x = points[0].p_x;
    let mut max_x = points[0].p_x;
    let mut min_y = points[0].p_y;
    let mut max_y = points[0].p_y;
    for p in points {
        if min_x > p.p_x { min_x = p.p_x }
        if max_x < p.p_x { max_x = p.p_x }
        if min_y > p.p_y { min_y = p.p_y }
        if max_y < p.p_y { max_y = p.p_y }
    }
    (min_x, max_x, min_y, max_y)
}

fn print_points(points: &Vec<Point>) {
    let (min_x, max_x, min_y, max_y) = get_min_max(points);
    let mut canvas = vec![vec![false; (max_x - min_x + 1) as usize]; (max_y - min_y + 1) as usize];
    for p in points {
        let x = (p.p_x - min_x) as usize;
        let y = (p.p_y - min_y) as usize;
        canvas[y][x] = true;
    }
    for row in canvas.iter() {
        for &c in row {
            let w = if c {
                '#'
            } else {
                '.'
            };
            print!("{}", w);
        }
        println!("");
    }
}

fn main() {
    let mut debug = false;
    debug = true;
    let file_name = if debug { "../example" } else { "../input" };
    let file = File::open(file_name).unwrap();
    let reader = BufReader::new(file);
    let mut points = vec![];

    let re = Regex::new(r"position=<\s?(-?\d+?), \s?(-?\d+?)> velocity=<\s?(-?\d+?), \s?(-?\d+?)>").unwrap();
    for line_result in reader.lines() {
        let line = line_result.unwrap();
        // println!("{:?}", line);
        for cap in re.captures_iter(&line) {
            // println!("{} {} {} {}", &cap[1], &cap[2], &cap[3], &cap[4]);
            points.push(Point{
                p_x: cap[1].parse::<i32>().unwrap(),
                p_y: cap[2].parse::<i32>().unwrap(),
                v_x: cap[3].parse::<i32>().unwrap(),
                v_y: cap[4].parse::<i32>().unwrap(),
            })
        }
    }

    let mut prev_diff = None;
    let mut count = 0;
    loop {
        let (min_x, max_x, min_y, max_y) = get_min_max(&points);
        let this_diff = max_x - min_x + max_y - min_y;
        if prev_diff != None && prev_diff.unwrap() < this_diff {
            dec_points(&mut points);
            print_points(&points);
            println!("at {} seconds", count - 1);
            break;
        }
        count += 1;
        prev_diff = Some(this_diff);
        inc_points(&mut points);
    }

    // println!("Result: {}", );
}
