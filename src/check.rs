use std::{fs, path::Path};
use colored::Colorize;
use crate::{
    CHARS,
    Nicknames,
    Gids,
    Context,
    Error,
};

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

pub async fn gids_json_exists(init_check: bool) -> Option<String> {
    let data_from_file = fs::read_to_string("data/gids.json")
        .expect("Failed to read 'gids.json' file.");

    match serde_json::from_str::<Gids>(&data_from_file) {
        Ok(_) => {
            println!("{}", "Successfully read 'gids.json' file.".green());
            None
        },
        Err(_) => {
            let error_msg = "Error: Failed to deserialize 'gids.json' file.\nDownload and import the `data` folder from:\nhttps://github.com/yakiimoninja/baiken.".to_string();

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
            let error_msg = "Error: Missing '".to_owned() + character_path +"' folder.\nDownload and import the `data` folder from:\nhttps://github.com/yakiimoninja/baiken.";
            
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
            let error_msg ="Error: Missing '".to_owned() + character_json + "' file.\nPlease execute the '/update' command.";

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
            let error_msg ="Error: Missing '".to_owned() + images_json +"' file.\nDownload and import the `data` folder from:\nhttps://github.com/yakiimoninja/baiken.";
            
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

pub async fn character_info_exist(init_check: bool) -> Option<String> {
    
    // Checking if character images.jsons exist in their respective folders
    for char in CHARS {
        
        let info_json = &("data/".to_owned() + char + "/info.json");
        if !Path::new(&info_json).exists() {

            // Error if images json doesnt exist
            let error_msg ="Error: Missing '".to_owned() + info_json +"' file.\nDownload and import the `data` folder from:\nhttps://github.com/yakiimoninja/baiken.";
            
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

/// Runs checks depening on the arguments given
/// Takes multiple tuples as input with the first item of the tuple
/// being a bool to see if a check::function to will execute
/// and the second being if its the initial check or not.
#[allow(clippy::too_many_arguments)]
pub async fn adaptive_check(
    ctx: Context<'_>,
    data_folder_check: bool,
    nicknames_json_check: bool,
    character_folders_check: bool,
    character_jsons_check: bool,
    character_images_check: bool,
    character_info_check: bool,
    gids_json_check: bool,
) -> Result<(), Error> {
    
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
        if let Some(error_msg) = character_images_exist(false).await {
            ctx.say(&error_msg.replace('\'', "`")).await?;
            println!();
            panic!("{}", error_msg.replace('\n', " ").red());
        }
    }
    if character_info_check {        
        // Checking if info jsons exist
        if let Some(error_msg) = character_info_exist(false).await {
            ctx.say(&error_msg.replace('\'', "`")).await?;
            println!();
            panic!("{}", error_msg.replace('\n', " ").red());
        }
    }
    if gids_json_check {
        // Checking if gids json exists
        if let Some(error_msg) = gids_json_exists(false).await {
            ctx.say(&error_msg.replace('\'', "`")).await?;
            println!();
            panic!("{}", error_msg.replace('\n', " ").red());
        }
    }
    
    Ok(())
}
