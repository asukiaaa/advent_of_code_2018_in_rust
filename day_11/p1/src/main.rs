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

fn main() {
    let mut debug = false;
    debug = false;
    let input = if debug { 42 } else { 1723 };
    let field_len = 300;

    let mut grid = vec![vec![0; field_len]; field_len];
    for y in 0..300 {
        for x in 0..300 {
            grid[y][x] = get_level(input, x + 1, y + 1);
        }
    }

    print_grids_on(&grid, 32, 44);
    let square_len = 3;
    let mut sum_grid = vec![vec![0; field_len + square_len]; field_len + square_len];
    for (y, row) in grid.iter().enumerate() {
        for (x, cell) in row.iter().enumerate() {
            for x_diff in 0..square_len {
                for y_diff in 0..square_len {
                    sum_grid[y + y_diff][x + x_diff] += cell;
                }
            }
        }
    }

    let mut max_power = sum_grid[2][2];
    let mut max_x = 0;
    let mut max_y = 0;
    for (y, row) in sum_grid.iter().enumerate() {
        if y < 2 || y >= field_len { continue }
        for (x, cell) in row.iter().enumerate() {
            if x < 2 || x >= field_len { continue }
            // println!("{}", cell);
            if max_power < *cell {
                max_power = *cell;
                max_x = x - square_len / 2;
                max_y = y - square_len / 2;
            }
        }
    }

    println!("Result: {} on ({},{})", max_power, max_x, max_y);
    println!("{} == 4", get_level(8, 3, 5));
    println!("{} == -5", get_level(57, 122, 79));
    println!("{} == 0", get_level(39, 217, 196));
    println!("{} == 4", get_level(71, 101, 153));
}
