use crate::{CHARS, CharInfo, commands, Nicknames};
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

pub fn init_json_exists(init_check: bool) -> Option<String> {

    // If init.json doesnt exist initialize it
    if Path::new("data/init.json").exists() == false {
        commands::update::init_json::init_json();
    }

    // Reading init.json file
    let data_from_file = fs::read_to_string("data/init.json")
        .expect("\nFailed to read 'init.json' file.");

    match serde_json::from_str::<Vec<CharInfo>>(&data_from_file) {
        Ok(_) => {
            println!("\nSuccesfully read 'init.json' file.");
            return None;
        },
        Err(_) => {
            let error_msg = "Failed to deserialize 'init.json' file. Delete 'init.json' from the 'data' folder and rerun the bot.".to_string();
            
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

pub fn nicknames_json_exists(init_check: bool) -> Option<String> {

    // Reading nicknames.json file
    let data_from_file = fs::read_to_string("data/nicknames.json")
        .expect("\nFailed to read 'nicknames.json' file.");

    match serde_json::from_str::<Vec<Nicknames>>(&data_from_file) {
        Ok(_) => {
            println!("\nSuccesfully read 'nicknames.json' file.");
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
    for x in 0..CHARS.len() {
        
        let character_path = &("data/".to_owned() + &CHARS[x]);
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
    for x in 0..CHARS.len() {

        let character_json = &("data/".to_owned() + &CHARS[x] + "/" + &CHARS[x] +".json");
        if Path::new(&character_json).exists() == false {

            // Error if character json doesnt exist
            let error_msg ="Missing '".to_owned() + &character_json + "' file.\nPlease execute the 'b.update' command.";

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
    println!("\nSuccesfully read {} character.json files.", &CHARS.len());
    return None;
}

pub fn character_images_exist(init_check: bool) -> Option<String> {

    // Checking if character images.jsons exist in their respective folders
    for x in 0..CHARS.len() {
        
        let images_json = &("data/".to_owned() + &CHARS[x] + "/images.json");
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