use std::{fs, path::Path};
use colored::Colorize;
use crate::{CHARS, Nicknames, Context, Error};

// Collection of functions that check for stuff

pub async fn data_folder_exists(init_check: bool) -> Option<String> {

    // Checking if data folder exists
    if Path::new("data").exists() {
        None
    }
    else {
        // Error message cause data folder does not exist
        let error_msg = "Error: The 'data' folder does not exist.\nDownload and import the 'data' folder from:\nhttps://github.com/yakiimoninja/baiken.".to_string();

        if init_check {
            // Printing the error message in the console
            // If it is the initial check
            println!();
            panic!("{}", error_msg.red());
        }
        else {
            // Returning the error message for in-discord printing
            Some(error_msg)
        }   
    }
}

pub async fn nicknames_json_exists(init_check: bool) -> Option<String> {

    // Reading nicknames.json file
    let data_from_file = fs::read_to_string("data/nicknames.json")
        .expect("\nFailed to read 'nicknames.json' file.");

    match serde_json::from_str::<Vec<Nicknames>>(&data_from_file) {
        Ok(_) => {
            println!("{}", "Successfully read 'nicknames.json' file.".green());
            None
        },
        Err(_) => {
            let error_msg = "Error: Failed to deserialize 'nicknames.json' file.\nDownload and import the `data` folder from:\nhttps://github.com/yakiimoninja/baiken.".to_string();
            
            if init_check {
                // Printing the error message in the console
                // If it is the initial check
                println!();
                panic!("{}", error_msg.red());
            }
            else {
                // Returning the error message for in-discord printing
                Some(error_msg)
            }
            
        },
    }
}

pub async fn character_folders_exist(init_check: bool) -> Option<String> {

    // Checking if character folders exist
    for char in CHARS {
        
        let character_path = &("data/".to_owned() + char);
        if !Path::new(&character_path).exists() {
            
            // Error if character folder doesnt exist
            let error_msg = "Error: Missing '".to_owned() + &character_path +"' folder.\nDownload and import the `data` folder from:\nhttps://github.com/yakiimoninja/baiken.";
            
            if init_check {
                // Printing the error message in the console
                // If it is the initial check
                println!();
                panic!("{}", error_msg.red());
            }
            else {
                // Returning the error message for in-discord printing
                return Some(error_msg);
            }
        }
    }
    
    None
}

pub async fn character_jsons_exist(init_check: bool) -> Option<String> {

    // Checking if character jsons exist in their respective folders
    for char in CHARS {

        let character_json = &("data/".to_owned() + char + "/" + char +".json");
        if !Path::new(&character_json).exists() {

            // Error if character json doesnt exist
            let error_msg ="Error: Missing '".to_owned() + &character_json + "' file.\nPlease execute the '/update' command.";

            if init_check {
                // Printing the error message in the console
                // If it is the initial check
                println!();
                panic!("{}", error_msg.red());
            }
            else {
                // Returning the error message for in-discord printing
                return Some(error_msg);
            }
        }
    }
    println!("{}", ("Successfully read ".to_owned() + &CHARS.len().to_string() + " character.json files.").green());

    None
}

pub async fn character_images_exist(init_check: bool) -> Option<String> {

    // Checking if character images.jsons exist in their respective folders
    for char in CHARS {
        
        let images_json = &("data/".to_owned() + char + "/images.json");
        if !Path::new(&images_json).exists() {

            // Error if images json doesnt exist
            let error_msg ="Error: Missing '".to_owned() + &images_json +"' file.\nDownload and import the `data` folder from:\nhttps://github.com/yakiimoninja/baiken.";
            
            if init_check {
                // Printing the error message in the console
                // If it is the initial check
                println!();
                panic!("{}", error_msg.red());
            }
            else {
                // Returning the error message for in-discord printing
                return Some(error_msg);
            }
        }
    }
    None
}

pub async fn correct_character_arg(character_arg: &String) -> Option<String>{
    
    // Checking for correct character argument
    if character_arg.len() < 2 {
        let error_msg = "Character name `".to_owned() + &character_arg + "` is invalid!";
        Some(error_msg)
    }
    else{
        None
    }
}
  
pub async fn correct_character_move_arg(character_move_arg: &String) -> Option<String>{

    // Checking for correct move argument
    if character_move_arg.len() < 2 {
        let error_msg = "Move `".to_owned() + &character_move_arg + "` is invalid!";
        Some(error_msg)
    }
    else{
        None
    }
}

/// Runs checks depening on the arguments given
/// Takes multiple tuples as input with the first item of the tuple
/// being a bool to see if a check::function to will execute
/// and the second being if its the initial check or not.
#[allow(clippy::too_many_arguments)]
pub async fn adaptive_check(
    ctx: Context<'_>,
    correct_character_check: (bool, &String),
    correct_character_move_check: (bool, &String),
    data_folder_check: bool,
    nicknames_json_check: bool,
    character_folders_check: bool,
    character_jsons_check: bool,
    character_images_check: bool,
) -> Result<(), Error> {
    
    let mut checks_passed = true;

    if correct_character_check.0 {
        // Checking if character user argument is correct
        if let Some(error_msg) = correct_character_arg(correct_character_check.1).await {
            ctx.say(&error_msg).await?;
            println!("{}", ("Error: ".to_owned() + &error_msg.to_string()).red());
            checks_passed = false;
        }
    }
    if correct_character_move_check.0 {
        // Checking if move user argument is correct
        if let Some(error_msg) = correct_character_move_arg(correct_character_move_check.1).await {
            ctx.say(&error_msg).await?;
            println!("{}", ("Error: ".to_owned() + &error_msg.to_string()).red());
            checks_passed = false;
        }
    }
    if data_folder_check {
        // Checking if data folder exists
        if let Some(error_msg) = data_folder_exists(false).await {
            ctx.say(&error_msg.replace('\'', "`")).await?;
            println!();
            panic!("{}", error_msg.replace('\n', " ").red());
        }
    }
    if nicknames_json_check {        
        // Checking if nicknames.json exists
        if let Some(error_msg) = nicknames_json_exists(false).await {
            ctx.say(&error_msg.replace('\'', "`")).await?;
            println!();
            panic!("{}", error_msg.replace('\n', " ").red());
        }
    }
    if character_folders_check {       
        // Checking if character folders exist
        if let Some(error_msg) = character_folders_exist(false).await {
            ctx.say(&error_msg.replace('\'', "`")).await?;
            println!();
            panic!("{}", error_msg.replace('\n', " ").red());
        }
    }
    if character_jsons_check {        
        // Checking if character jsons exist
        if let Some(error_msg) = character_jsons_exist(false).await {
            ctx.say(&error_msg.replace('\'', "`")).await?;
            println!();
            panic!("{}", error_msg.replace('\n', " ").red());
        }
    }
    if character_images_check {        
        // Checking if image jsons exist
        if let Some(error_msg) = data_folder_exists(false).await {
            ctx.say(&error_msg.replace('\'', "`")).await?;
            println!();
            panic!("{}", error_msg.replace('\n', " ").red());
        }
    }
    
    if checks_passed {
        Ok(())
    }
    else {
        Err("Failed adaptive_check".into())
    }
}