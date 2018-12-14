fn get_level(input: i32, x: usize, y: usize) -> i32 {
    let rack_id = x as i32 + 10;
    let mut p_level = y as i32 * rack_id * rack_id + input * rack_id;
    // println!("{} {} {}", p_level, p_level % 1000, ((p_level % 1000) as f32 / 100.0).floor() as i32);
    // p_level = if p_level < 100 { 0 } else { ((p_level % 1000) as f32 / 100.0).floor() as i32 };
    p_level = if p_level < 100 { 0 } else { p_level % 1000 / 100 };
    p_level - 5
}

fn print_grids_on(grid: &Vec<Vec<i32>>, x: usize, y: usize) {
    for py in y..y+9 {
        for px in x..x+9 {
            print!("{:>3}", grid[py][px]);
        }
        println!("");
    }
}

fn create_grid(input: i32, field_len: usize) -> Vec<Vec<i32>> {
    let mut grid = vec![vec![0; field_len]; field_len];
    for y in 0..300 {
        for x in 0..300 {
            grid[y][x] = get_level(input, x + 1, y + 1);
        }
    }
    grid
}

fn get_max_power(grid: &Vec<Vec<i32>>, square_len: usize) -> (i32, usize, usize) {
    let field_len = grid.len();
    let mut sum_grid = vec![vec![0; field_len + square_len]; field_len + square_len];
    for y in 0..field_len {
        for x in 0..field_len {
            for x_diff in 0..square_len {
                let target_x = x + x_diff;
                if target_x >= field_len { break; }
                for y_diff in 0..square_len {
                    let target_y = y + y_diff;
                    if target_y >= field_len { break; }
                    sum_grid[y][x] += grid[y+y_diff][x+x_diff];
                }
            }
        }
    }

    let mut max_power = sum_grid[0][0];
    let mut max_x = 1;
    let mut max_y = 1;
    for (y, row) in sum_grid.iter().enumerate() {
        if y + square_len > field_len { continue }
        for (x, cell) in row.iter().enumerate() {
            if x + square_len > field_len { continue }
            // println!("{}", cell);
            if max_power < *cell {
                max_power = *cell;
                max_x = x + 1;
                max_y = y + 1;
            }
        }
    }
    (max_power, max_x, max_y)
}

fn main() {
    let mut debug = false;
    // debug = true;
    let input = if debug { 18 } else { 1723 };
    let field_len = 300;

    let grid = create_grid(input, field_len);
    // print_grids_on(&grid, 32, 44);

    let mut max_power:Option<i32> = None;
    let mut max_x = 0;
    let mut max_y = 0;
    let mut max_square_len = 0;
    let mut prev_info_option: Option<(i32, usize, usize)> = None;
    for square_len in 1..field_len {
        let info = get_max_power(&grid, square_len);
        println!("{} {:?}", square_len, info);
        if max_power.is_none() || info.0 > max_power.unwrap() {
            max_power = Some(info.0);
            max_x = info.1;
            max_y = info.2;
            max_square_len = square_len;
        }
        if info.0 < 0 {
        // if prev_info_option.is_some() && prev_info_option.unwrap().0 > info.0 {
            break;
        }
        prev_info_option = Some(info);
    }
    println!("Result: {} on {},{},{}", max_power.unwrap(), max_x, max_y, max_square_len);
}

// not: 234,150,5
