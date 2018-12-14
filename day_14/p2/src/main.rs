fn get_new_index(recipes: &Vec<usize>, index: usize) -> usize {
    let mut new_index = index + recipes[index] + 1;
    let recipe_len = recipes.len();
    // println!("{}", new_index);
    loop {
        if new_index < recipe_len { break; }
        new_index -= recipe_len;
        // println!("{}", new_index);
    }
    // println!("update {} to {}", index, new_index);
    new_index
}

fn combine(recipes: &mut Vec<usize>, a_index: usize, b_index: usize) {
    // println!("{} {}", a_index, b_index);
    let sum = recipes[a_index] + recipes[b_index];
    // println!("sum: {}", sum);
    if sum >= 10 {
        recipes.push(sum / 10);
    }
    recipes.push(sum % 10);
}

fn print_info(recipes: &Vec<usize>, a_index: usize, b_index: usize) {
    let recipe_len = recipes.len();
    for index in 0..recipe_len {
        let v = recipes[index];
        if index == a_index {
            print!("({})", v);
        } else if index == b_index {
            print!("[{}]", v);
        } else {
            print!(" {} ", v);
        }
    }
    println!("");
}

fn print_recipes(recipes: &Vec<usize>, from: usize, to: usize) {
    for index in from..to {
        print!("{}", recipes[index]);
    }
    println!("");
}

fn print_lasts(recipes: &Vec<usize>) {
    let print_len = 10;
    let recipe_len = recipes.len();
    let min_index = if recipe_len < print_len { 0 } else { recipe_len - print_len };
    print_recipes(&recipes, min_index, recipe_len);
}

fn match_recipe(recipes: &Vec<usize>, index: usize, match_target: &Vec<usize>) -> bool {
    // if index + match_target.len() > recipes.len() { return false; }
    let match_len = match_target.len();
    let mut matched = true;
    // println!("index: {}", index);
    for i in 0 .. match_len {
        // println!("compare {} {}", recipes[index + i], match_target[i]);
        if recipes[index + i] != match_target[i] {
            return false;
        }
    }
    true
}

fn main() {
    let mut debug = false;
    // debug = true;
    let match_target = if debug {
        // vec![5,1,5,8,9]
        vec![5,9,4,1,4]
    } else {
        vec![3,8,0,6,2,1]
    };
    let match_len = match_target.len();
    let mut recipes = vec![3, 7];
    let mut a_index = 0;
    let mut b_index = 1;
    let mut check_index = 0;
    let mut matched_index: Option<usize> = None;
    // print_info(&recipes, a_index, b_index);
    loop {
        // if check_index > 20 { break } // debug
        combine(&mut recipes, a_index, b_index);
        // print_info(&recipes, a_index, b_index);
        if recipes.len() > check_index + match_len {
            print!("{}\r", check_index);
            loop {
                if match_recipe(&recipes, check_index, &match_target) {
                    matched_index = Some(check_index);
                    break;
                }
                check_index += 1;
                if check_index + match_len == recipes.len() { break; }
            }
            // print_recipes(&recipes, target_recipe_num, target_recipe_num + 10);
        }
        if matched_index.is_some() {
            break;
        }
        a_index = get_new_index(&recipes, a_index);
        b_index = get_new_index(&recipes, b_index);
    }
    println!("check index: {}", check_index);
    println!("{:?}", matched_index);
}

// not 1032141626
