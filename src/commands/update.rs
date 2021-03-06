use std::fs;
use serde_json;
use serenity::framework::standard::{macros::command, CommandResult};
use serenity::model::prelude::*;
use serenity::prelude::*;

use crate::{CHARS, CharInfo, check};

pub(crate) mod init_json;
mod char_json;

const SITE_LINK: &str = "https://dustloop.com/wiki/api.php?action=parse&page=";
const SITE_HALF: &str = "&prop=text&formatversion=2";

#[command]
#[aliases("u")]
#[owners_only]
pub async fn update(ctx: &Context, msg: &Message) -> CommandResult {

    // Checking if images jsons exist
    if let Some(error_msg) = check::character_images_exist(false) {
        msg.channel_id.say(&ctx.http, &error_msg.replace("'", "`")).await?;
        print!("\n");
        panic!("{}", error_msg.replace("\n", " "));
    }

    // Checking if character folders exist
    if let Some(error_msg) = check::character_folders_exist(false) {
        msg.channel_id.say(&ctx.http, &error_msg.replace("'", "`")).await?;
        print!("\n");
        panic!("{}", error_msg.replace("\n", " "));
    }

    // Checking if init.json exists
    if let Some(error_msg) = check::init_json_exists(false) {
        msg.channel_id.say(&ctx.http, &error_msg.replace("'", "`")).await?;
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
    msg.channel_id.say(&ctx.http, "Update succesful!").await?;
    
    return Ok(());
}
