mod framedata;
mod images; 
mod framedata_json;
mod images_json;
use colored::Colorize;
use crate::{Context, Error, CHARS, check, find};
use crate::serenity::futures::{Stream, StreamExt, self};

// Autocompletes the update option
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
    #[description = r#"Character name, nickname or "all"."#] character: String,
    #[description = r#"Select "frames", "images" or "all"."#]
    #[autocomplete = "autocomplete_option"] option: String,
) -> Result<(), Error> {

    let option = option.trim().to_lowercase();

    if (check::adaptive_check(
        ctx,
        (true, &character),
        (false, &String::new()),
        true,
        true,
        true,
        false,
        false).await).is_err() {
        
        return Ok(());
    }

    // // Checking if images jsons exist
    // if let Some(error_msg) = check::character_images_exist(false).await {
    //     ctx.say(&error_msg.replace('\'', "`")).await?;
    //     println!();
    //     panic!("{}", error_msg.replace('\n', " "));
    // }

    // Finding character
    let character_arg_altered = match find::find_character(&character).await {
        Ok(character_arg_altered) => character_arg_altered,
        Err(err) => {
            ctx.say(err.to_string()).await?;
            println!("{}", ("Error: ".to_owned() + &err.to_string()).red());
            return Ok(()) }
    };

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
            // Update frames for specific character
            ctx.say("Update started!").await?; 
            framedata::get_char_data(CHARS, &character_arg_altered).await;
        
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
            // Update images for specific character
            ctx.say("Update started!").await?; 
            images::get_char_data(CHARS, &character_arg_altered).await;
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
            // Update frames and images for specific character
            ctx.say("Update started!").await?; 
            framedata::get_char_data(CHARS, &character_arg_altered).await;
            images::get_char_data(CHARS, &character_arg_altered).await;
        }
    }
    // If none
    else {
        let error_msg= &("Selection `".to_owned() + &option + "` is invalid!");
        ctx.say(error_msg).await?;
        println!("{}", ("Error: Selection '".to_owned() + &option + "' is invalid!").red());
        return Ok(());
    }

    ctx.channel_id().say(ctx, "Update succesful!").await?;

    Ok(())
}
