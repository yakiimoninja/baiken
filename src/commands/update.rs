use std::fs;
use crate::{Context, Error, CHARS, check, Nicknames};
mod framedata;
mod images; 
mod framedata_json;
mod images_json; 

/// Updates the frame data according to dustloop. Owners only!
#[poise::command(prefix_command, slash_command, aliases("u"), owners_only)]
pub async fn update (
    ctx: Context<'_>,
    #[description = r#"Select "frames", "images" or "all"."#] frames_or_images: String,
    #[description = r#"Character name or nickname or "all"."#] character_arg: String,
) -> Result<(), Error> {

    // This will store the full character name in case user input was an alias
    let mut character_arg_altered = String::new();
    // Flag that will be used for logic to determine output
    let mut character_found = false;

    let frames_or_images = frames_or_images.trim().to_lowercase();

    // Checking if character user argument is correct
    if let Some(error_msg) = check::correct_character_arg(&character_arg){
        ctx.say(&error_msg).await?;
        println!();
        panic!("{}", error_msg);
    }
    
    // Checking if images jsons exist
    if let Some(error_msg) = check::character_images_exist(false) {
        ctx.say(&error_msg.replace('\'', "`")).await?;
        println!();
        panic!("{}", error_msg.replace('\n', " "));
    }

    // Checking if character folders exist
    if let Some(error_msg) = check::character_folders_exist(false) {
        ctx.say(&error_msg.replace('\'', "`")).await?;
        println!();
        panic!("{}", error_msg.replace('\n', " "));
    }

    // Reading the nicknames json
    let data_from_file = fs::read_to_string("data/nicknames.json")
        .expect("\nFailed to read 'nicknames.json' file.");
    
    // Deserializing from nicknames json
    let vec_nicknames = serde_json::from_str::<Vec<Nicknames>>(&data_from_file).unwrap();

    // Iterating through the nicknames.json character entries
    for x_nicknames in vec_nicknames {

        // If user input is part of a characters full name or the full name itself
        // Then pass the full and correct charactet name to the new var 'character_arg_altered'
        if x_nicknames.character.to_lowercase().replace('-', "").contains(&character_arg.to_lowercase()) ||
        x_nicknames.character.to_lowercase().contains(&character_arg.to_lowercase()) {
            
            character_found = true;
            character_arg_altered = x_nicknames.character.to_owned();
            break;
        }

        // Iterating through the nicknames.json nickname entries
        for y_nicknames in x_nicknames.nicknames {

            // If user input equals a character nickname then pass the full character name
            // To the new variable 'character_arg_altered'
            if y_nicknames.to_lowercase() == character_arg.to_lowercase().trim() {

                character_found = true;
                character_arg_altered = x_nicknames.character.to_owned();
                break;
            }   
        }
    }

    // Update frames hand
    if frames_or_images == "frames" {

        // If character arg is all; update frames for all characters
        if character_arg.trim().to_lowercase() == "all"{
            ctx.say("Update started!").await?; 
            framedata::get_char_data(CHARS, "all").await;
        }
        else {
            
            // Updates images for specific character
            // If user input isnt the full name, part of a full name or a nickname
            // Error out cause requested character was not found in the json
            if !character_found {
    
                let error_msg= &("Character `".to_owned() + &character_arg + "` was not found!");
                ctx.say(error_msg).await?;
                println!();
                panic!("{}", error_msg.replace('`', "'"));
            }
            else {
                // Update frames for specific character
                ctx.say("Update started!").await?; 
                println!();
                framedata::get_char_data(CHARS, &character_arg_altered).await;
            }
        }
    }
    // Update images hand
    else if frames_or_images == "images"{
        
        // If character arg is all; update images for all characters
        if character_arg.trim().to_lowercase() == "all"{
            ctx.say("Update started!").await?; 
            images::get_char_data(CHARS, "all").await;
        }
        else {
            
            // Updates images for specific character
            // If user input isnt the full name, part of a full name or a nickname
            // Error out cause requested character was not found in the json
            if !character_found {
    
                let error_msg= &("Character `".to_owned() + &character_arg + "` was not found!");
                ctx.say(error_msg).await?;
                println!();
                panic!("{}", error_msg.replace('`', "'"));
            }
            else {
                // Update images for specific character
                ctx.say("Update started!").await?; 
                println!();
                images::get_char_data(CHARS, &character_arg_altered).await;
            }
        }
    }
    // Update both frames and images hand
    else if frames_or_images == "all"{

        // If character arg is all; update frames and images for all characters
        if character_arg.trim().to_lowercase() == "all"{
            ctx.say("Update started!").await?;
            framedata::get_char_data(CHARS, "all").await;
            images::get_char_data(CHARS, "all").await;
        }
        else {
            
            // If user input isnt the full name, part of a full name or a nickname
            // Error out cause requested character was not found in the json
            if !character_found {
    
                let error_msg= &("Character `".to_owned() + &character_arg + "` was not found!");
                ctx.say(error_msg).await?;
                println!();
                panic!("{}", error_msg.replace('`', "'"));
            }
            else {
                // Update frames and images for specific character
                ctx.say("Update started!").await?; 
                println!();
                framedata::get_char_data(CHARS, &character_arg_altered).await;
                images::get_char_data(CHARS, &character_arg_altered).await;
            }
        }
    }
    // If none
    else {
        let error_msg= &("Selection `".to_owned() + &frames_or_images + "` is invalid!");
        ctx.say(error_msg).await?;
        println!();
        panic!("{}", error_msg.replace('`', "'"));
    }

    ctx.channel_id().say(ctx, "Update succesful!").await?;

    Ok(())
}
