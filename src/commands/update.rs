use std::fs;
use crate::{Context, Error, CHARS, CharInfo, check};

pub(crate) mod init_json;
mod char_json;

/// Updates the frame data according to dustloop. Owners only!
#[poise::command(prefix_command, slash_command, aliases("u"), owners_only)]
pub async fn update (
    ctx: Context<'_>,
) -> Result<(), Error> {

    // Checking if images jsons exist
    if let Some(error_msg) = check::character_images_exist(false) {
        ctx.say(&error_msg.replace("'", "`")).await?;
        print!("\n");
        panic!("{}", error_msg.replace("\n", " "));
    }

    // Checking if character folders exist
    if let Some(error_msg) = check::character_folders_exist(false) {
        ctx.say(&error_msg.replace("'", "`")).await?;
        print!("\n");
        panic!("{}", error_msg.replace("\n", " "));
    }

    // Checking if init.json exists
    if let Some(error_msg) = check::init_json_exists(false) {
        ctx.say(&error_msg.replace("'", "`")).await?;
        print!("\n");
        panic!("{}", error_msg.replace("\n", " "));
    }

    // Reading init.json file
    let data_from_file = fs::read_to_string("data/init.json")
        .expect("\nFailed to read 'init.json' file.");

    // Deserializing from init.json
    let file = serde_json::from_str::<Vec<CharInfo>>(&data_from_file)
        .expect("\nFailed to deserialize from 'init.json' file.\nConsider deleting 'init.json' from the 'frame_data' folder.");

    char_json::make_char_json(CHARS, file).await;
    ctx.say("Update succesful!").await?;
    
    return Ok(());
}
