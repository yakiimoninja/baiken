use crate::{CHARS, Nicknames};
use std::{fs, path::Path};

// Collection of functions that check for stuff

pub fn data_folder_exists(init_check: bool) -> Option<String> {

    // Checking if data folder exists
    if Path::new("data").exists() == true {
        return None;
    }
    else {
        // Error message cause data folder does not exist
        let error_msg = "The 'data' folder does not exist.\nDownload and import the 'data' folder from:\nhttps://github.com/yakiimoninja/baiken-bot.".to_string();

        if init_check == true {
            // Printing the error message in the console
            // If it is the initial check
            print!("\n");
            panic!("{}", error_msg);
        }
        else {
            // Returning the error message for in-discord printing
            return Some(error_msg);
        }   
    }
}

pub fn nicknames_json_exists(init_check: bool) -> Option<String> {

    // Reading nicknames.json file
    let data_from_file = fs::read_to_string("data/nicknames.json")
        .expect("\nFailed to read 'nicknames.json' file.");

    match serde_json::from_str::<Vec<Nicknames>>(&data_from_file) {
        Ok(_) => {
            println!("\nSuccessfully read 'nicknames.json' file.");
            return None;
        },
        Err(_) => {
            let error_msg = "Failed to deserialize 'nicknames.json' file.\nDownload and import the `data` folder from:\nhttps://github.com/yakiimoninja/baiken-bot..".to_string();
            
            if init_check == true {
                // Printing the error message in the console
                // If it is the initial check
                print!("\n");
                panic!("{}", error_msg);
            }
            else {
                // Returning the error message for in-discord printing
                return Some(error_msg);
            }
            
        },
    };
}

pub fn character_folders_exist(init_check: bool) -> Option<String> {

    // Checking if character folders exist
    for char in CHARS {
        
        let character_path = &("data/".to_owned() + &char);
        if Path::new(&character_path).exists() == false {
            
            // Error if character folder doesnt exist
            let error_msg = "Missing '".to_owned() + &character_path +"' folder.\nDownload and import the `data` folder from:\nhttps://github.com/yakiimoninja/baiken-bot.";
            
            if init_check == true {
                // Printing the error message in the console
                // If it is the initial check
                print!("\n");
                panic!("{}", error_msg);
            }
            else {
                // Returning the error message for in-discord printing
                return Some(error_msg);
            }
        }
    }
    return None;
}

pub fn character_jsons_exist(init_check: bool) -> Option<String> {

    // Checking if character jsons exist in their respective folders
    for char in CHARS {

        let character_json = &("data/".to_owned() + &char + "/" + &char +".json");
        if Path::new(&character_json).exists() == false {

            // Error if character json doesnt exist
            let error_msg ="Missing '".to_owned() + &character_json + "' file.\nPlease execute the '/update' command.";

            if init_check == true {
                // Printing the error message in the console
                // If it is the initial check
                print!("\n");
                panic!("{}", error_msg);
            }
            else {
                // Returning the error message for in-discord printing
                return Some(error_msg);
            }
        }
    }
    println!("\nSuccessfully read {} character.json files.", &CHARS.len());
    return None;
}

pub fn character_images_exist(init_check: bool) -> Option<String> {

    // Checking if character images.jsons exist in their respective folders
    for char in CHARS {
        
        let images_json = &("data/".to_owned() + &char + "/images.json");
        if Path::new(&images_json).exists() == false {

            // Error if images json doesnt exist
            let error_msg ="Missing '".to_owned() + &images_json +"' file.\nDownload and import the `data` folder from:\nhttps://github.com/yakiimoninja/baiken-bot.";
            
            if init_check == true {
                // Printing the error message in the console
                // If it is the initial check
                print!("\n");
                panic!("{}", error_msg);
            }
            else {
                // Returning the error message for in-discord printing
                return Some(error_msg);
            }
        }
    }
    return None;
}

pub fn correct_character_arg(character_arg: &String) -> Option<String>{
    // Checking for correct character argument
    if character_arg.len() < 2 {
        let error_msg = "Character name: `".to_owned() + &character_arg + "` is invalid!";
        return Some(error_msg);
    }
    else{
        return None;
    }
}
  
pub fn correct_character_move_arg(character_move_arg: &String) -> Option<String>{

    // Checking for correct move argument
    if character_move_arg.len() < 2 {
        let error_msg = "Move: `".to_owned() + &character_move_arg + "` is invalid!";
        return Some(error_msg);
    }
    else{
        return None;
    }
}
