mod framedata;
mod framedata_db;
mod images;
mod images_db;
mod info;
mod info_db;
use crate::{CHARS, Context, Error, check, find};

#[derive(Debug, poise::ChoiceParameter)]
pub enum UpdateChoice{
    #[name = "all"]
    All,
    #[name = "frames"]
    Frames,
    #[name = "images"]
    Images,
    #[name = "info"]
    Info,
}

/// Update data according to dustloop. Owners only.
#[poise::command(prefix_command, slash_command, owners_only, ephemeral)]
pub async fn update (
    ctx: Context<'_>,
    #[min_length = 2]
    #[description = r#"Character name, nickname or "all"."#] character: String,
    #[description = r#"Select "frames", "info", "images" or "all"."#] option: UpdateChoice,
) -> Result<(), Error> {

    if (check::adaptive_check(ctx, true, false).await).is_err() {
        return Ok(());
    }

    // Finding character
    let character_arg_altered = match find::find_character(&character).await {
        Ok(character_arg_altered) => character_arg_altered,
        Err(err) => {
            ctx.say(err.to_string()).await?;
            return Ok(()) }
    };

    match option {
        UpdateChoice::All => {
            // If character arg is all; update frames, images and info for all characters
            if character.trim().to_lowercase() == "all"{
                ctx.say("Update started!").await?;
                update_all_char_data().await;
            }
            else {
                // If user input isnt the full name, part of a full name or a nickname
                // Update frames, images and info for specific character
                ctx.say("Update started!").await?; 
                framedata::get_char_data(CHARS, &character_arg_altered).await;
                images::get_char_images(CHARS, &character_arg_altered).await;
                info::get_char_info(CHARS, &character_arg_altered).await;
            }
        }
        UpdateChoice::Frames => {
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
        UpdateChoice::Images => {
            // If character arg is all; update images for all characters
            if character.trim().to_lowercase() == "all"{
                ctx.say("Update started!").await?; 
                images::get_char_images(CHARS, "all").await;
            }
            else {
                // Updates images for specific character
                // If user input isnt the full name, part of a full name or a nickname
                // Update images for specific character
                ctx.say("Update started!").await?; 
                images::get_char_images(CHARS, &character_arg_altered).await;
            }
        }
        UpdateChoice::Info => {
            // If character arg is all; update info for all characters
            if character.trim().to_lowercase() == "all"{
                ctx.say("Update started!").await?; 
                info::get_char_info(CHARS, "all").await;
            }
            else {
                // Updates info for specific character
                // If user input isnt the full name, part of a full name or a nickname
                // Update info for specific character
                ctx.say("Update started!").await?; 
                info::get_char_info(CHARS, &character_arg_altered).await;
            }
        }
    }

    ctx.say("Update succesful!").await?;

    Ok(())
}

pub async fn update_all_char_data(){
    // 24 hour character data auto update function
    framedata::get_char_data(CHARS, "all").await;
    images::get_char_images(CHARS, "all").await;
    info::get_char_info(CHARS, "all").await;
}
