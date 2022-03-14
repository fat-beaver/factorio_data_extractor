use std::fs;
use crate::recipe::Recipe;

mod crafter;
mod recipe;

fn main() {
    let recipes = read_prototype_file(String::from("prototypes/recipe.lua"));
    println!("{}", recipes.len());
}

fn read_prototype_file(filepath: String) -> Vec<Recipe> {
    let mut file_contents = fs::read_to_string(filepath).expect("Could not read file");
    //remove all whitespace
    file_contents.retain(|c| !c.is_whitespace());
    //find where the actual data starts
    let first_open_bracket = file_contents.chars().position(|c| c == '{').unwrap();
    let mut file_data = file_contents.split_off(first_open_bracket + 1);
    //remove the closing brackets of those removed at the start
    file_data.split_off(file_data.len() - 2);

    let mut recipes: Vec<Recipe> = Vec::new();
    let mut bracket_depth = 0;
    //keep track of the last split location so that the recipes before and after the current one can be cut off
    let mut last_split = 0;
    for (i, current_char) in file_data.chars().enumerate() {
        //change the bracket depth if either type is present
        if current_char.eq(&'{') {
            bracket_depth += 1;
        } else if current_char.eq(&'}') {
            bracket_depth -= 1;
        //if the current position is not inside brackets and is a comma which splits recipes, find the recipe string
        } else if bracket_depth == 0 && current_char.eq(&',') {
            let mut data_temp = file_data.clone();
            data_temp.split_off(i);
            let recipe = data_temp.split_off(last_split + 1);
            last_split = i;
            println!("{}", recipe);
            //use the recipe string to make a new recipe
            recipes.push(Recipe::new_from_string(recipe));
        }
    }
    return recipes;
}
