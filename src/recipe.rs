pub struct Recipe {
    category: String,
    ingredients: Vec<String>,
    counts: Vec<i32>,
    enabled: bool
}

impl Recipe {
    pub fn new(category: String, ingredients: Vec<String>, counts: Vec<i32>, enabled: bool) -> Recipe {
        if ingredients.len() != counts.len() {
            panic!("recipe makes no sense!");
        }
        Recipe {category, ingredients, counts, enabled}
    }
    pub fn new_from_string(string: String) -> Recipe{
        Recipe::new(String::from("test"),vec![String::from("test"), String::from("test2")], vec![1, 2], true)
    }
}
