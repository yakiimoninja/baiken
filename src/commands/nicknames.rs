use std::{fs, string::String};
use poise::serenity_prelude::CreateEmbed;
use crate::{
    Nicknames,
    Context,
    Error,
    check,
};

/// Display all character nicknames.
#[poise::command(prefix_command, slash_command)]
pub async fn nicknames(
    ctx: Context<'_>,
) -> Result<(), Error> {

    if (check::adaptive_check(
        ctx,
        true,
        true,
        false,
        false,
        false,
        false,
        false).await).is_err() {
        
        return Ok(());
    }

    // Reading the nicknames json
    let data_from_file = fs::read_to_string("data/nicknames.json")
        .expect("\nFailed to read 'nicknames.json' file.");

    // Deserializing from nicknames json
    let vec_nicknames = serde_json::from_str::<Vec<Nicknames>>(&data_from_file).unwrap();
    
    // Formatting string for in discord print
    let mut nicks_as_msg = "".to_string();

    for nicknames in vec_nicknames {
        // Character portion
        nicks_as_msg = nicks_as_msg.to_owned() + "\n- **"
            + &nicknames.character.to_string().replace("_", " ") + "**";
        
        // Nickname portion
        // THE SPACE BEFORE THE ARROW
        // IS A SPECIAL CHARACTER
        // DISCORD WONT RENDER EMPTY PRECEEDING SPACE
        // SO THE EMBED ISNT FORMATTED PROPERLY
        // 3x U+2000 &#8192 En Quad
        // DONT USE TABS OR END OF EMBED
        // WILL HAVE EMPTY PADDING
        nicks_as_msg += "\n\t→ `";
        
        for x in 0..nicknames.nicknames.len() {
            if x != nicknames.nicknames.len() - 1 {
                // Taking into account the lack of nicknames for some characters
                if !nicknames.nicknames[x].is_empty() {
                    nicks_as_msg = nicks_as_msg + &nicknames.nicknames[x] + "`, `";
                }
                else {
                    nicks_as_msg = nicks_as_msg + &nicknames.nicknames[x];
                }
            }
            else {
                nicks_as_msg = nicks_as_msg + &nicknames.nicknames[x];
            }
        }
        nicks_as_msg = nicks_as_msg.to_owned() + "`";
    }

    // Sending the data as an embed
    let embed = CreateEmbed::new()
        .title("__**Character Nicknames**__")
        .url("https://github.com/yakiimoninja/baiken/blob/main/data/nicknames.json")
        .description(nicks_as_msg)
        .color((140,75,64));

    ctx.send(poise::CreateReply::default().embed(embed)).await?;

    Ok(())
}
