use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::collections::HashMap;

fn get_nearby_positions(x: &usize, y: &usize, x_max: &usize, y_max: &usize) -> Vec<(usize, usize)> {
    let mut nearby_positions = vec![];
    if *x > 0 { nearby_positions.push((*x-1, *y)); }
    if *y > 0 { nearby_positions.push((*x, *y-1)); }
    if *x < *x_max { nearby_positions.push((*x+1, *y)); }
    if *y < *y_max { nearby_positions.push((*x, *y+1)); }
    nearby_positions
}

fn get_highest_priority_step(steps: &Vec<char>) -> Option<char> {
    for c in vec!['t', 'l', 'r', 'b'].iter() {
        if steps.iter().find(|&&s| s == *c).is_some() {
            return Some(*c);
        }
    }
    None
}

fn next_position(x: &usize, y: &usize, step: &char) -> (usize, usize) {
    if *step == 't' {
        (*x, *y-1)
    } else if *step == 'l' {
        (*x-1, *y)
    } else if *step == 'r' {
        (*x+1, *y)
    } else {
        (*x, *y+1)
    }
}

#[derive(Debug, Clone)]
struct Unit {
    role: char,
    hp: usize,
}

impl Unit {
    fn new (role: char) -> Unit {
        Unit { role: role, hp: 200 }
    }
    fn is_elve(&self) -> bool {
        self.role == 'E'
    }
}

#[derive(Debug, Clone)]
struct FieldCell {
    is_wall: bool,
    unit_key: Option<usize>,
}

impl FieldCell {
    fn new () -> FieldCell {
        FieldCell { is_wall: false, unit_key: None }
    }
}

#[derive(Clone)]
struct Field {
    units: HashMap<usize, Unit>,
    field_cells: Vec<Vec<FieldCell>>,
    x_len: usize,
    y_len: usize,
}

impl Field {
    fn new(x_len: usize, y_len: usize) -> Field {
        Field{ units: HashMap::new(), field_cells: vec![vec![FieldCell::new(); x_len]; y_len], x_len, y_len }
    }
    fn print(&self) {
        for y in 0..self.y_len {
            for x in 0..self.x_len {
                let fc = &self.field_cells[y][x];
                if fc.is_wall {
                    print!("#     ");
                } else if fc.unit_key.is_some() {
                    let key = &fc.unit_key.unwrap();
                    let u = self.units.get(key).unwrap();
                    print!("{}{}:{:3}", u.role, key, u.hp);
                } else {
                    print!(".     ");
                }
            }
            println!("");
        }
    }
    fn set_as_wall(&mut self, x: usize, y: usize) {
        self.field_cells[y][x].is_wall = true;
    }
    fn set_as_unit(&mut self, x: usize, y: usize, unit: Unit) {
        let key = self.units.len();
        self.units.insert(key, unit);
        self.field_cells[y][x].unit_key = Some(key);
    }
    fn get_unit_position(&self, unit_key: &usize) -> Option<(usize, usize)> {
        for y in 0..self.y_len {
            for x in 0..self.x_len {
                let c = &self.field_cells[y][x];
                if c.unit_key.is_some() && c.unit_key.unwrap() == *unit_key {
                    return Some((x, y))
                }
            }
        }
        None
    }
    fn unit_facing_enemies(&self, unit_key: &usize) -> Vec<usize> {
        let mut enemies = vec![];
        let this_unit = self.units.get(unit_key).unwrap();
        let (this_x, this_y) = self.get_unit_position(unit_key).unwrap();
        for (enemy_key, unit) in self.units.iter() {
            if enemy_key == unit_key || unit.role == this_unit.role || unit.hp == 0 { continue; }
            let (enemy_x, enemy_y) = self.get_unit_position(enemy_key).unwrap();
            let mut diff = 0;
            if this_x > enemy_x {
                diff += this_x - enemy_x;
            } else {
                diff += enemy_x - this_x;
            }
            if this_y > enemy_y {
                diff += this_y - enemy_y;
            } else {
                diff += enemy_y - this_y;
            }
            if diff == 1 {
                enemies.push(*enemy_key);
            }
        }
        enemies
    }
    fn get_unit_reachable_distances(&self, unit_key: &usize) -> Vec<Vec<Option<usize>>> {
        let (this_x, this_y) = self.get_unit_position(unit_key).unwrap();
        let mut distances: Vec<Vec<Option<usize>>> = vec![vec![None; self.x_len]; self.y_len];
        distances[this_y][this_x] = Some(0);
        let mut current_distance = 0;
        let x_max = self.x_len - 1;
        let y_max = self.y_len - 1;
        loop {
            let mut stept_cells = 0;
            for y in 0..self.y_len {
                for x in 0..self.x_len {
                    let c = &self.field_cells[y][x];
                    if distances[y][x].is_some() || c.is_wall || c.unit_key.is_some() { continue }
                    // println!("x_len-1 y_len-1: {} {}", x_len-1, y_len-1);
                    // println!("x y: {} {}", x, y);
                    for (n_x, n_y) in get_nearby_positions(&x, &y, &x_max, &y_max) {
                        // println!("nx ny: {} {}", n_x, n_y);
                        // let c = &self.field_cells[n_y][n_x];
                        // if c.is_wall || c.unit_key.is_some() { continue }
                        match distances[n_y][n_x] {
                            Some(d) => {
                                if d == current_distance {
                                    distances[y][x] = Some(d+1);
                                    stept_cells += 1;
                                    break
                                }
                            },
                            None => {},
                        }
                    }
                }
            }
            current_distance += 1;
            // println!("stept_cells: {}", stept_cells);
            if stept_cells == 0 { break }
        }
        distances
    }
    fn step_for_position(&mut self, from_x: &usize, from_y: &usize, dest_x: &usize, dest_y: &usize, distances: &Vec<Vec<Option<usize>>>) -> char {
        let mut positions = vec![(*dest_x, *dest_y)];
        let mut dist = distances[*dest_y][*dest_x].unwrap();
        let x_max = self.x_len - 1;
        let y_max = self.y_len - 1;
        loop {
            if dist == 1 { break; }
            dist -= 1;
            let mut next_positions: Vec<(usize, usize)> = vec![];
            for (x, y) in positions {
                for (n_x, n_y) in get_nearby_positions(&x, &y, &x_max, &y_max) {
                    let this_d = distances[n_y][n_x];
                    if this_d.is_some() && this_d.unwrap() == dist && next_positions.iter().find(|&&p| p.0 == n_x && p.1 == n_y).is_none() {
                        next_positions.push((n_x, n_y));
                    }
                }
            }
            positions = next_positions;
        }
        // println!("positions: {:?}", positions);
        // println!("from: {} {}", from_x, from_y);
        let next_steps = positions.iter().map(
            |(x, y)|
            if *y < *from_y {
                't'
            } else if *x < *from_x {
                'l'
            } else if *x > *from_x {
                'r'
            } else {
                'b'
            }
        ).collect::<Vec<char>>();
        // println!("next steps: {:?}", next_steps);
        get_highest_priority_step(&next_steps).unwrap()
    }
    fn unit_move_step(&mut self, key: &usize, step: &char) {
        // println!("move {} as {}", key, step);
        let (this_x, this_y) = self.get_unit_position(key).unwrap();
        let (next_x, next_y) = next_position(&this_x, &this_y, step);
        self.field_cells[this_y][this_x].unit_key = None;
        self.field_cells[next_y][next_x].unit_key = Some(*key);
    }
    fn unit_move_to_enemy(&mut self, key: &usize) {
        let x_max = self.x_len - 1;
        let y_max = self.y_len - 1;
        let this_unit = self.units.get(key).unwrap().clone();
        let (this_x, this_y) = self.get_unit_position(key).unwrap();
        // println!("moving {} {:?}", key, this_unit);
        let enemy_positions = self.units.iter().filter_map(
            |(key, u)|
            if u.role != this_unit.role && u.hp != 0 {
                self.get_unit_position(key)
            } else {
                None
            }
        ).collect::<Vec<(usize, usize)>>();
        let distances = self.get_unit_reachable_distances(key);
        // for dist_row in distances.iter() {
        //     for d in dist_row {
        //         let d_str = match d {
        //             Some(v) => {v.to_string()},
        //             None => {".".to_string()},
        //         };
        //         print!("{} ", d_str);
        //     }
        //     println!("");
        // }
        let mut nearest_destinations = vec![];
        let mut nearest_distance = 0;
        for (x, y) in enemy_positions {
            for (dx, dy) in get_nearby_positions(&x, &y, &x_max, &y_max) {
                match distances[dy][dx] {
                    Some(distance) => {
                        if nearest_destinations.is_empty() {
                            nearest_destinations.push((dx, dy));
                            nearest_distance = distance;
                        } else if nearest_distance == distance {
                            nearest_destinations.push((dx, dy));
                        } else if nearest_distance > distance {
                            nearest_destinations = vec![(dx, dy)];
                            nearest_distance = distance;
                        }
                    },
                    None => {}
                }
            }
        }
        if nearest_destinations.is_empty() { return }
        // println!("nearest destinations {:?}", nearest_destinations);
        let mut nearest_levels: Vec<_> = nearest_destinations.iter().map(
            |(nx, ny)| ((nx, ny), self.get_position_level(nx, ny))
        ).collect();
        nearest_levels.sort_by(|(_, a), (_, b)| a.cmp(b));
        let (target_x, target_y) = nearest_levels[0].0;
        let step = self.step_for_position(&this_x, &this_y, &target_x, &target_y, &distances);
        // println!("step: {}", step);
        self.unit_move_step(&key, &step);
    }
    fn get_position_level(&self, x: &usize, y: &usize) -> usize {
        *x + *y * self.x_len
    }
    fn get_unit_position_level(&self, key: &usize) -> Option<usize> {
        let position = self.get_unit_position(key);
        if position.is_none() {
            None
        } else {
            let (x, y) = position.unwrap();
            Some(self.get_position_level(&x, &y))
        }
    }
    fn ordered_unit_keys(&self) -> Vec<usize> {
        let mut keys = self.units.iter().filter_map(
            |(k, _)| {
                let level = self.get_unit_position_level(k);
                if level.is_some() {
                    Some((k, level))
                } else {
                    None
                }
            }
        ).collect::<Vec<_>>();
        keys.sort_by(|(_, a_d), (_, b_d)| a_d.cmp(b_d));
        keys.iter().map(|(&k, _d)| k).collect::<Vec<_>>()
        // let mut keys = self.units.keys().map(|&k| k).collect::<Vec<_>>();
        // keys.sort();
        // keys
    }
    fn unit_attack(&mut self, key: &usize, elve_power: usize) {
        let this_unit = self.units.get(key).unwrap().clone();
        let enemies = self.unit_facing_enemies(key);
        if enemies.is_empty() { return }
        // println!("enemies: {:?}", enemies);
        let mut enemy_k_hp: Vec<(&usize, usize)> = enemies.iter().map(
            |e_key| (e_key, self.units.get(e_key).unwrap().hp)
        ).collect();
        enemy_k_hp.sort_by(|(_, a_hp), (_, b_hp)| a_hp.cmp(b_hp));
        // println!("enemy_k_hp: {:?}", enemy_k_hp);
        let target_key = if enemy_k_hp.len() == 1 || enemy_k_hp[0].1 < enemy_k_hp[1].1 {
            enemy_k_hp[0].0
        } else {
            let min_hp = enemy_k_hp[0].1;
            let mut enemy_k_l: Vec<_> = enemy_k_hp.iter().filter_map(
                |(k, hp)|
                if *hp == min_hp {
                    Some((k, self.get_unit_position_level(k)))
                } else { None }
            ).collect();
            enemy_k_l.sort_by(|(_, a_l), (_, b_l)| a_l.cmp(b_l));
            enemy_k_l[0].0
        };
        let damage = if this_unit.is_elve() { elve_power } else { 3 };
        let hp = self.units.get(&target_key).unwrap().hp.clone().saturating_sub(damage);
        self.units.get_mut(&target_key).unwrap().hp = hp;
        if hp == 0 {
            let (x, y) = self.get_unit_position(&target_key).unwrap();
            self.field_cells[y][x].unit_key = None;
        }
    }
    fn finished(&self) -> bool {
        let mut e_count = 0;
        let mut g_count = 0;
        for (_, u) in self.units.iter() {
            if u.hp == 0 { continue }
            if u.is_elve() { e_count += 1 }
            else { g_count += 1 }
        }
        e_count == 0 || g_count == 0
    }
    fn get_total_hp(&self) -> usize {
        self.units.iter().filter_map(
            |(_, u)|
            if u.hp == 0 { None } else { Some(u.hp) }
        ).sum()
    }
    fn left_elves_number(&self) -> usize {
        self.units.iter().filter_map(|(_, u)| if u.is_elve() && u.hp != 0 { Some(true) } else { None }).collect::<Vec<_>>().len()
    }
}

fn main() {
    let mut debug = false;
    // debug = true;
    let file_name = if debug { "../example" } else { "../input" };
    let file = File::open(file_name).unwrap();
    let reader = BufReader::new(file);
    let mut lines = reader.lines().into_iter().map(|l| l.unwrap()).collect::<Vec<String>>();
    lines.remove(0);
    lines.pop();

    let mut original_field = Field::new(lines.len(), lines[0].len() - 2);
    for (y, line) in lines.iter_mut().enumerate() {
        line.remove(0);
        line.pop();
        // println!("{:?}", line);
        for (x, cell_u8) in line.as_bytes().iter().enumerate() {
            let cell = *cell_u8 as char;
            if cell == 'G' || cell == 'E' {
                original_field.set_as_unit(x, y, Unit::new(cell));
            } else if cell == '#' {
                original_field.set_as_wall(x, y);
            }
        }
    }
    // original_field.print();

    let mut failed_max_power = 4;
    let mut succeeded_min_power: Option<usize> = None;
    let mut elve_power = 4;
    let mut succeeded_loop_count = 0;
    let mut succeeded_hp = 0;
    let elves_number = original_field.left_elves_number();
    loop {
        let mut field = original_field.clone();
        let mut loop_count = 0;
        // field.print();
        loop {
            // println!("\n---- loop: {} ----", loop_count+1);
            let mut action_count = 0;
            let keys = field.ordered_unit_keys();
            let keys_len = keys.len();
            for key in keys {
                if field.finished() { break };
                if field.units.get(&key).unwrap().hp != 0 {
                    if field.unit_facing_enemies(&key).is_empty() {
                        field.unit_move_to_enemy(&key);
                    }
                    field.unit_attack(&key, elve_power);
                }
                action_count += 1;
            }
            if action_count != keys_len { break }
            // field.print();
            loop_count += 1;
        }
        // println!("finished");
        // field.print();

        let total_hp = field.get_total_hp();
        println!("Buttle score: {} * {} = {}", total_hp, loop_count, total_hp * loop_count);
        let left_number = field.left_elves_number();
        println!("Left elves: {}", left_number);
        println!("Elve power: {}", elve_power);
        println!("failed: {} succeeded: {:?}", failed_max_power, succeeded_min_power);
        if left_number != elves_number {
            failed_max_power = elve_power;
            match succeeded_min_power {
                Some(succeeded_power) => {
                    elve_power += (succeeded_power - elve_power) / 2;
                },
                None => {
                    elve_power *= 2;
                }
            }
        } else {
            succeeded_min_power = Some(elve_power);
            succeeded_loop_count = loop_count;
            succeeded_hp = total_hp;
            elve_power -= (elve_power - failed_max_power) / 2;
        }
        if succeeded_min_power.is_some() && succeeded_min_power.unwrap() - 1 == failed_max_power { break }
        // if elve_power > 30 { break }
    }
    println!("Elve power: {:?}", succeeded_min_power);
    println!("Result: {} * {} = {}", succeeded_hp, succeeded_loop_count, succeeded_hp * succeeded_loop_count);
}
