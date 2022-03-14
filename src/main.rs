use std::{error::Error, fs};

use regex::Regex;

fn main() -> Result<(), Box<dyn Error>> {
    let file_contents = read_files()?;
    let mut prototypes: Vec<String> = file_contents.iter().map(|file| process_file(file)).flatten().collect();
    //add recipes and assembling machines to their own lists and discard the other prototypes
    let mut recipes = Vec::new();
    let mut assemblers = Vec::new();
    for prototype in prototypes {
        let prototype_type = find_prototype_type(&prototype);
        if prototype_type == "recipe" {
            recipes.push(prototype);
        } else if prototype_type == "assembling-machine" {
            assemblers.push(prototype);
        }
    }
    println!(
        "found {} recipes and {} assembling machines",
        recipes.len(),
        assemblers.len()
    );

    Ok(())
}

fn read_files() -> Result<Vec<String>, Box<dyn Error>> {
    let files = vec![
        String::from("prototypes/entity/entities.lua"),
        String::from("prototypes/recipe.lua"),
    ];

    let contents: Result<Vec<_>, _> = files
        .iter()
        .map(|filename| fs::read_to_string(filename))
        .collect();
    Ok(contents?)
}

fn process_file(contents: &str) -> Vec<String> {
    let stripped_contents = remove_comments(contents);
    let data_sections: Vec<String> = extract_data_sections(stripped_contents);
    println!("{} sections found in file", data_sections.len());
    let prototypes: Vec<String> = data_sections
        .into_iter()
        .map(process_data_section)
        .flatten()
        .collect();
    println!("{} prototypes read from file", prototypes.len());
    prototypes
}

fn remove_comments(string: &str) -> String {
    // fuck yeah turbofish
    let mut contents: String = string
        .lines()
        .map(remove_line_comment)
        .collect::<Vec<String>>()
        .join("\n");
    loop {
        let edited_contents = remove_first_block_comment(&contents);
        if contents == edited_contents {
            break edited_contents;
        }
        contents = edited_contents;
    }
}

fn remove_line_comment(string: &str) -> String {
    let mut line_start = None;

    for (i, _) in string.match_indices("--") {
        if i > string.len() - 3 {
            line_start = Some(i);
            break;
        }

        if &string[i + 2..i + 4] != "[[" {
            line_start = Some(i);
            break;
        }
    }

    match line_start {
        Some(i) => string.split_at(i).0.to_owned(),
        None => string.to_owned(),
    }
}

fn remove_first_block_comment(string: &str) -> String {
    let re = Regex::new(r"--\[\[.*?]]").unwrap();

    match re.find(string) {
        Some(m) => {
            let mut string = string.to_owned();
            string.replace_range(m.range(), "");
            string
        }
        None => string.to_owned(),
    }
}

// assumes that string[start] == "("
fn get_matching_bracket((open, close): (char, char), string: &str, start: usize) -> Option<usize> {
    let mut depth = 1;
    let mut i = start;
    let chars: Vec<char> = string.chars().collect();

    loop {
        i += 1;

        if i >= chars.len() {
            return None;
        }

        match chars[i] {
            open => depth += 1,
            close => depth -= 1,
            _ => (),
        }

        if depth == 0 {
            return Some(i);
        }
    }
}

fn find_data_sections(file_contents: &str) -> Result<Vec<String>, Box<dyn Error>> {
    let mut file_contents = file_contents.replace(char::is_whitespace, "");
    let mut sections: Vec<String> = vec![];

    while let Some(start) = file_contents.find("data:extend(") {
        let paren_start = start + "data:extend".len();

        let end = get_matching_bracket(('(', ')'), &file_contents, paren_start)
            .ok_or("Unmatched parens")?;
        sections.push(file_contents[paren_start..end].to_owned());
        file_contents.replace_range(start..end, "");
    }

    Ok(sections)
}

fn process_data_section(section: &str) -> Vec<String> {
    let mut section = section[1..section.len() - 1].to_owned();
    let mut prototypes: Vec<String> = vec![];

    while let Some(start) = section.find("{") {
        let end = match get_matching_bracket(('{', '}'), &section, start) {
            Some(i) => i,
            None => {
                section = section[1..].to_owned();
                continue;
            }
        };

        prototypes.push(section[start..end].to_owned());
        section.replace_range(start..end, "");
    }

    prototypes
}

fn find_prototype_type(prototype: &String) -> String {
    let mut prototype_type = prototype.clone();
    for (i, section_char) in prototype.chars().enumerate() {
        //find the first comma and split the string to remove the second quote, the comma, and everything after it
        if section_char.eq(&',') {
            prototype_type.truncate(i - 1);
            break;
        }
    }
    for (i, section_char) in prototype_type.chars().enumerate() {
        //get rid of the first quote and everything before it
        if section_char.eq(&'"') {
            prototype_type = prototype_type.split_off(i + 1);
            break;
        }
    }
    return prototype_type;
}
