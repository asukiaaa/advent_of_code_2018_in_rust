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

fn main() {
    let target_recipe_num = 380621;
    let mut recipes = vec![3, 7];
    let mut a_index = 0;
    let mut b_index = 1;
    // print_info(&recipes, a_index, b_index);
    loop {
        combine(&mut recipes, a_index, b_index);
        // print_info(&recipes, a_index, b_index);
        if recipes.len() > target_recipe_num + 10 {
            print_recipes(&recipes, target_recipe_num, target_recipe_num + 10);
            break;
        }
        a_index = get_new_index(&recipes, a_index);
        b_index = get_new_index(&recipes, b_index);
    }
}

// not 1032141626
