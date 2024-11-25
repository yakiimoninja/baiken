use std::{fs, path::Path};
use colored::Colorize;
use poise::reply::CreateReply;
use crate::{
    Context, Error, Gids, Nicknames, CHARS
};

// Collection of functions that check for stuff

pub async fn data_folder_exists() -> Result<(), Error> {

    // Checking if data folder exists
    if Path::new("data").exists() {
        Ok(())
    }
    else {
        // Error message cause data folder does not exist
        let error_msg = "Failed to open 'data' folder.";
        Err(error_msg.into())
    }
}

pub async fn gids_json_exists() -> Result<(), Error> {

    // Reading gids.json file
    let data_from_file = match fs::read_to_string("data/gids.json") {
        Ok(data_from_file) =>  data_from_file,
        Err(_) => {
            let error_msg = "Failed to open 'data/gids.json' file.";
            println!("{}", error_msg.red());
            return Err(error_msg.into());
        }
    };

    match serde_json::from_str::<Gids>(&data_from_file) {
        Ok(_) => {
            println!("{}", "Successfully read 'gids.json' file.".green());
            Ok(())
        },
        Err(_) => {
            let error_msg = "Failed to deserialize 'gids.json' file.";
            // Returning the error message for in-discord printing
            Err(error_msg.into())
        }
    }
}

pub async fn nicknames_json_exists() -> Result<(), Error> {

    // Reading nicknames.json file
    let data_from_file = match fs::read_to_string("data/nicknames.json") {
        Ok(data_from_file) =>  data_from_file,
        Err(_) => {
            let error_msg = "Failed to open 'data/nicknames.json' file.";
            return Err(error_msg.into());
        }
    };

    match serde_json::from_str::<Vec<Nicknames>>(&data_from_file) {
        Ok(_) => {
            println!("{}", "Successfully read 'nicknames.json' file.".green());
            Ok(())
        },
        Err(_) => {
            let error_msg = "Failed to deserialize 'nicknames.json' file.";
            Err(error_msg.into())
        }
    }
}

pub async fn character_folders_exist() -> Result<(), Error> {

    // Checking if character folders exist
    for char in CHARS {
        
        let character_path = &("data/".to_owned() + char);
        if !Path::new(&character_path).exists() {
            // Error if character folder doesnt exist
            let error_msg = "Failed to open '".to_owned() + character_path +"' folder.";
            return Err(error_msg.into());
        }
    }
    Ok(())
}

pub async fn character_jsons_exist() -> Result<(), Error> {

    // Checking if character jsons exist in their respective folders
    for char in CHARS {

        let character_json = &("data/".to_owned() + char + "/" + char +".json");
        if !Path::new(&character_json).exists() {
            // Error if character json doesnt exist
            let error_msg ="Failed to open '".to_owned() + character_json + "' file.";
            return Err(error_msg.into());
        }
    }
    println!("{}", ("Successfully read ".to_owned() + &CHARS.len().to_string() + " character.json files.").green());
    Ok(())
}

pub async fn character_images_exist() -> Result<(), Error> {

    // Checking if character images.jsons exist in their respective folders
    for char in CHARS {
        
        let images_json = &("data/".to_owned() + char + "/images.json");
        if !Path::new(&images_json).exists() {
            // Error if images json doesnt exist
            let error_msg ="Failed to open '".to_owned() + images_json +"' file.";
            return Err(error_msg.into());
        }
    }
    Ok(())
}

pub async fn character_info_exist() -> Result<(), Error> {
    
    // Checking if character images.jsons exist in their respective folders
    for char in CHARS {
        
        let info_json = &("data/".to_owned() + char + "/info.json");
        if !Path::new(&info_json).exists() {
            // Error if images json doesnt exist
            let error_msg ="Failed to open '".to_owned() + info_json +"' file.";
            return Err(error_msg.into());
        }
    }
    Ok(())
}

/// Runs checks depening on the arguments given
/// bool to see if a check::function to will execute
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
        if let Err(error_msg) = data_folder_exists().await {
            ctx.send(CreateReply::default()
                .content(&error_msg.to_string().replace('\'', "`"))
                .ephemeral(true))
            .await?;
            println!("{}", error_msg.to_string().replace('\n', " ").red());
            return Err(error_msg);
        }
    }
    if nicknames_json_check {        
        // Checking if nicknames.json exists
        if let Err(error_msg) = nicknames_json_exists().await {
            ctx.send(CreateReply::default()
                .content(&error_msg.to_string().replace('\'', "`"))
                .ephemeral(true))
            .await?;
            println!("{}", error_msg.to_string().replace('\n', " ").red());
            return Err(error_msg);
        }
    }
    if character_folders_check {       
        // Checking if character folders exist
        if let Err(error_msg) = character_folders_exist().await {
            ctx.send(CreateReply::default()
                .content(&error_msg.to_string().replace('\'', "`"))
                .ephemeral(true))
            .await?;
            println!("{}", error_msg.to_string().replace('\n', " ").red());
            return Err(error_msg);
        }
    }
    if character_jsons_check {        
        // Checking if character jsons exist
        if let Err(error_msg) = character_jsons_exist().await {
            ctx.send(CreateReply::default()
                .content(&error_msg.to_string().replace('\'', "`"))
                .ephemeral(true))
            .await?;
            println!("{}", error_msg.to_string().replace('\n', " ").red());
            return Err(error_msg);
        }
    }
    if character_images_check {        
        // Checking if image jsons exist
        if let Err(error_msg) = character_images_exist().await {
            ctx.send(CreateReply::default()
                .content(&error_msg.to_string().replace('\'', "`"))
                .ephemeral(true))
            .await?;
            println!("{}", error_msg.to_string().replace('\n', " ").red());
            return Err(error_msg);
        }
    }
    if character_info_check {        
        // Checking if info jsons exist
        if let Err(error_msg) = character_info_exist().await {
            ctx.send(CreateReply::default()
                .content(&error_msg.to_string().replace('\'', "`"))
                .ephemeral(true))
            .await?;
            println!("{}", error_msg.to_string().replace('\n', " ").red());
            return Err(error_msg);
        }
    }
    if gids_json_check {
        // Checking if gids json exists
        if let Err(error_msg) = gids_json_exists().await {
            ctx.send(CreateReply::default()
                .content(&error_msg.to_string().replace('\'', "`"))
                .ephemeral(true))
            .await?;
            println!("{}", error_msg.to_string().replace('\n', " ").red());
            return Err(error_msg);
        }
    }
    Ok(())
}
