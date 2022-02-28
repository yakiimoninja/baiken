use crate::{CHARS, CharInfo, commands};
use std::{fs, path::Path};

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

pub fn frames_folder_exists(init_check: bool) -> Option<String> {

    // Checking if the frames folder exists
    if Path::new("data/frames").exists() == true {
        return None;
    }
    else {
        // Error if frames folder doesnt exist
        let error_msg = "The 'data/frames' folder was not found.\nDownload and import the `data` folder from:\nhttps://github.com/yakiimoninja/baiken-bot.".to_string();
        
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

pub fn character_folders_exist(init_check: bool) -> Option<String> {

    // Checking if character folders exist
    for x in 0..CHARS.0.len(){
        
        let character_path = &("data/frames/".to_owned() + &CHARS.0[x]);
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

pub fn character_folder_contents_exist(init_check: bool) -> Option<String> {

    // Checking if character jsons exist in their respective folder
    for x in 0..CHARS.0.len(){
        let character_json = &("data/frames/".to_owned() + &CHARS.0[x] + "/" + &CHARS.0[x] +".json");
        if Path::new(&character_json).exists() == false {

            // Error if character json doesnt exist
            let error_msg ="Missing '".to_owned() + &character_json + "'.\nPlease execute the 'b.update' command.";

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
    println!("\nSuccesfully read {} character.json files from 'data/frames' folder.", &CHARS.0.len());
    return None;
}

pub fn image_folder_exists(init_check: bool) -> Option<String> {

    // Checking if image folder exist
    if Path::new("data/images").exists() == true {
        return None;
    }
    else{
        // Error message if images folder doesnt exist
        let error_msg= "The 'data/images' folder was not found.\nDownload and import the `data` folder from:\nhttps://github.com/yakiimoninja/baiken-bot.".to_string();
        
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

pub fn image_folder_contents_exist(init_check: bool) -> Option<String> {
    
    // Checking if character txts exist
    for c in 0..CHARS.0.len(){
        if Path::new(&("data/images/".to_owned()+ CHARS.0[c]+".txt")).exists() == false {

            // Error message if a specific character txt is missing
            let error_msg = "The '".to_owned() + &("data/images/".to_owned()+ CHARS.0[c]+".txt") + "' file was not found.\nDownload and import the `data` folder from:\nhttps://github.com/yakiimoninja/baiken-bot.";
            
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
    println!("\nSuccesfully read {} character.txt files from 'data/images' folder.", CHARS.0.len());
    return None;
}