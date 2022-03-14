use std::fs;

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
fn read_in_file(filename: &String) -> Vec<String> {
    //read the file and remove all comments to get it ready for processing
    let prototype_string =
        remove_comments(&fs::read_to_string(filename).expect("Could not read file"));
    //find the data sections, split them apart from each other, and discard the rest
    let data_sections = find_data_sections(&prototype_string);
    println!("{} sections found in file", data_sections.len());
    //read the individual prototypes from each section
    let mut prototypes = Vec::new();
    for data_section in data_sections {
        prototypes.append(&mut read_data_section(&data_section));
    }
    println!("{} prototypes read from file", prototypes.len());
    return prototypes;
}

fn remove_comments(commented_prototype_string: &String) -> String {
    let mut commented_lines = Vec::new();
    let mut lines = Vec::new();

    for line in commented_prototype_string.lines() {
        commented_lines.push(String::from(line));
    }
    //check for block comments, these are a bit difficult
    let mut line_number = 0;
    while line_number != commented_lines.len() {
        let current_line = commented_lines.get(line_number).unwrap();
        //check if the line ends with two hyphens, indicating the end of a block comment
        if current_line.contains("--") && current_line.find("--").unwrap() == current_line.len() - 2
        {
            //find the start of said block comment
            let mut block_open_found = false;
            let mut line_number_to_check = line_number;
            while !block_open_found {
                line_number_to_check -= 1;
                let line_to_check = commented_lines.get(line_number_to_check).unwrap();
                if line_to_check.contains("--") {
                    block_open_found = true;
                }
            }
            //remove the block comment
            while line_number_to_check <= line_number {
                commented_lines.remove(line_number_to_check);
                line_number -= 1;
            }
        }
        line_number += 1;
    }
    line_number = 0;
    //remove normal comments too
    while line_number != commented_lines.len() {
        let mut current_line = commented_lines.get(line_number).unwrap().clone();
        if current_line.contains("--") {
            current_line.truncate(current_line.find("--").unwrap());
        }
        lines.push(current_line);
        line_number += 1;
    }
    //collect all of the lines back into one string and return it
    let prototype_string = lines.join("\n");
    return prototype_string;
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
