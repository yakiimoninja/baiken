use crate::{Context, Error, CHARS, check};

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

    ctx.say("Update started!").await?;
    
    char_json::make_char_json(CHARS).await;
    ctx.channel_id().say(ctx.discord(), "Update succesful!").await?;
    
    return Ok(());
}
