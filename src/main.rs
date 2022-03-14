use std::{error::Error, fs};

use regex::Regex;

fn main() {
    //list of all files to open
    let files = vec![
        String::from("prototypes/entity/entities.lua"),
        String::from("prototypes/recipe.lua"),
    ];
    let mut prototypes = Vec::new();
    //get all of the prototypes from each file
    for file in files {
        prototypes.append(&mut read_in_file(&file));
    }
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

fn read_in_file(filename: &String) -> Result<Vec<String>, Box<dyn Error>> {
    //read the file and remove all comments to get it ready for processing
    let prototype_string = remove_comments(&fs::read_to_string(filename)?);
    //find the data sections, split them apart from each other, and discard the rest
    let data_sections = find_data_sections(&prototype_string);
    println!("{} sections found in file", data_sections.len());
    //read the individual prototypes from each section
    let mut prototypes = Vec::new();
    for data_section in data_sections {
        prototypes.append(&mut read_data_section(&data_section));
    }
    println!("{} prototypes read from file", prototypes.len());
    Ok(prototypes)
}

fn remove_comments(string: &str) -> String {
    // fuck yeah turbofish
    let mut contents: String = string.lines().map(remove_line_comment).collect::<Vec<String>>().join("\n");
    loop {
        let edited_contents = remove_first_block_comment(&contents);
        if contents == edited_contents {
            break edited_contents
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

fn find_data_sections(file_contents: &String) -> Vec<String> {
    let mut file_contents_raw = file_contents.clone();
    //remove all whitespace
    file_contents_raw.retain(|c| !c.is_whitespace());
    let mut sections: Vec<String> = Vec::new();
    while file_contents_raw.contains("data:extend(") {
        let file_contents = file_contents_raw
            .split_off(file_contents_raw.find("data:extend(").unwrap() + "data:extend".len());
        //keep track of how deep we are in brackets to find the end of the data:extend section
        let mut bracket_depth = 0;
        for (i, current_char) in file_contents.chars().enumerate() {
            //change the bracket depth if either type is present
            if current_char.eq(&'(') {
                bracket_depth += 1;
            } else if current_char.eq(&')') {
                bracket_depth -= 1;
            }
            //if not inside any brackets this must be the end of a section, so split it out
            if bracket_depth == 0 {
                let mut data_section = file_contents.clone();
                file_contents_raw = data_section.split_off(i);
                sections.push(data_section);
                break;
            }
        }
    }
    return sections;
}

fn read_data_section(data_section: &String) -> Vec<String> {
    let mut prototype_data_raw = data_section.clone();
    let mut prototypes = Vec::new();
    //remove the starting and ending normal brackets
    prototype_data_raw.truncate(prototype_data_raw.len() - 1);
    let prototype_data = prototype_data_raw.split_off(2);
    let mut bracket_depth = 0;
    //keep track of the last split location so that the recipes before and after the current one can be cut off
    let mut last_split = 0;
    for (i, current_char) in prototype_data.chars().enumerate() {
        //change the bracket depth if either type is present
        if current_char.eq(&'{') {
            bracket_depth += 1;
        } else if current_char.eq(&'}') {
            bracket_depth -= 1;
            //if the current position is not inside brackets and is a comma which splits recipes, find the recipe string
        }
        if bracket_depth == 0 {
            if last_split < i {
                let mut data_temp = prototype_data.clone();
                data_temp.truncate(i);
                let data_section = data_temp.split_off(last_split);
                prototypes.push(data_section);
            }
            last_split = i + 1;
        }
    }
    return prototypes;
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
