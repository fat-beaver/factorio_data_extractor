pub struct Recipe {
    category: String,
    ingredients: Vec<String>,
    counts: Vec<i32>
}

impl Recipe {
    pub fn new(category: String, ingredients: Vec<String>, counts:Vec<i32>) -> Recipe {
        if ingredients.len() != counts.len() {
            println!("recipe makes no sense!");
            std::process::exit(1);
        }
        Recipe {category, ingredients, counts}
    }
}
