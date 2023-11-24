use std::fs;
use crate::{Context, Error, CHARS, check, Nicknames};
use crate::serenity::futures::{Stream, StreamExt, self};
mod framedata;
mod images; 
mod framedata_json;
mod images_json;

// Autocompletes the character name
async fn autocomplete_character<'a>(
    _ctx: Context<'_>,
    partial: &'a str,
) -> impl Stream<Item = String> + 'a {
    futures::stream::iter(&CHARS)
        .filter(move |name| futures::future::ready(name.to_lowercase().contains(&partial.to_lowercase())))
        .map(|name| name.to_string())
}

// Autocompletes the character name
async fn autocomplete_option<'a>(
    _ctx: Context<'_>,
    partial: &'a str,
) -> impl Stream<Item = String> + 'a {
    futures::stream::iter(&(["all","frames","images"]))
        .filter(move |name| futures::future::ready(name.to_lowercase().contains(&partial.to_lowercase())))
        .map(|name| name.to_string())
}

/// Updates the frame data according to dustloop. Owners only!
#[poise::command(prefix_command, slash_command, aliases("u"), owners_only)]
pub async fn update (
    ctx: Context<'_>,
    #[description = r#"Character name, nickname or "all"."#]
    #[autocomplete = "autocomplete_character"] character: String,
    #[description = r#"Select "frames", "images" or "all"."#]
    #[autocomplete = "autocomplete_option"] option: String,
) -> Result<(), Error> {

    // This will store the full character name in case user input was an alias
    let mut character_arg_altered = String::new();
    // Flag that will be used for logic to determine output
    let mut character_found = false;

    let option = option.trim().to_lowercase();

    if let Err(_) = check::adaptive_check(
        ctx,
        (true, &character),
        (false, &String::new()),
        true,
        true,
        true,
        false,
        false).await {
        
        return Ok(());
    }

    // // Checking if images jsons exist
    // if let Some(error_msg) = check::character_images_exist(false).await {
    //     ctx.say(&error_msg.replace('\'', "`")).await?;
    //     println!();
    //     panic!("{}", error_msg.replace('\n', " "));
    // }

    // Reading the nicknames json
    let data_from_file = fs::read_to_string("data/nicknames.json")
        .expect("\nFailed to read 'nicknames.json' file.");
    
    // Deserializing from nicknames json
    let vec_nicknames = serde_json::from_str::<Vec<Nicknames>>(&data_from_file).unwrap();

    // Iterating through the nicknames.json character entries
    for x_nicknames in vec_nicknames {

        // If user input is part of a characters full name or the full name itself
        // Then pass the full and correct charactet name to the new var 'character_arg_altered'
        if x_nicknames.character.to_lowercase().replace('-', "").contains(&character.to_lowercase()) ||
        x_nicknames.character.to_lowercase().contains(&character.to_lowercase()) {
            
            character_found = true;
            character_arg_altered = x_nicknames.character.to_owned();
            break;
        }

        // Iterating through the nicknames.json nickname entries
        for y_nicknames in x_nicknames.nicknames {

            // If user input equals a character nickname then pass the full character name
            // To the new variable 'character_arg_altered'
            if y_nicknames.to_lowercase() == character.to_lowercase().trim() {

                character_found = true;
                character_arg_altered = x_nicknames.character.to_owned();
                break;
            }   
        }
    }

    // Update frames hand
    if option == "frames" {

        // If character arg is all; update frames for all characters
        if character.trim().to_lowercase() == "all"{
            ctx.say("Update started!").await?; 
            framedata::get_char_data(CHARS, "all").await;
        }
        else {
            
            // Updates images for specific character
            // If user input isnt the full name, part of a full name or a nickname
            // Error out cause requested character was not found in the json
            if !character_found {
    
                let error_msg= &("Character `".to_owned() + &character + "` was not found!");
                ctx.say(error_msg).await?;
                println!("Error: {}", error_msg.replace('`', "'"));
                return Ok(());
            }
            else {
                // Update frames for specific character
                ctx.say("Update started!").await?; 
                framedata::get_char_data(CHARS, &character_arg_altered).await;
            }
        }
    }
    // Update images hand
    else if option == "images"{
        
        // If character arg is all; update images for all characters
        if character.trim().to_lowercase() == "all"{
            ctx.say("Update started!").await?; 
            images::get_char_data(CHARS, "all").await;
        }
        else {
            
            // Updates images for specific character
            // If user input isnt the full name, part of a full name or a nickname
            // Error out cause requested character was not found in the json
            if !character_found {
    
                let error_msg= &("Character `".to_owned() + &character + "` was not found!");
                ctx.say(error_msg).await?;
                println!("Error: {}", error_msg.replace('`', "'"));
                return Ok(());
            }
            else {
                // Update images for specific character
                ctx.say("Update started!").await?; 
                images::get_char_data(CHARS, &character_arg_altered).await;
            }
        }
    }
    // Update both frames and images hand
    else if option == "all"{

        // If character arg is all; update frames and images for all characters
        if character.trim().to_lowercase() == "all"{
            ctx.say("Update started!").await?;
            framedata::get_char_data(CHARS, "all").await;
            images::get_char_data(CHARS, "all").await;
        }
        else {
            
            // If user input isnt the full name, part of a full name or a nickname
            // Error out cause requested character was not found in the json
            if !character_found {
    
                let error_msg= &("Character `".to_owned() + &character + "` was not found!");
                ctx.say(error_msg).await?;
                println!("Error: {}", error_msg.replace('`', "'"));
                return Ok(());
            }
            else {
                // Update frames and images for specific character
                ctx.say("Update started!").await?; 
                framedata::get_char_data(CHARS, &character_arg_altered).await;
                images::get_char_data(CHARS, &character_arg_altered).await;
            }
        }
    }
    // If none
    else {
        let error_msg= &("Selection `".to_owned() + &option + "` is invalid!");
        ctx.say(error_msg).await?;
        println!("{}", "Error: Selection '".to_owned() + &option + "' is invalid!");
        return Ok(());
    }

    ctx.channel_id().say(ctx, "Update succesful!").await?;

    Ok(())
}
