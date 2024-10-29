use std::{fs, string::String};
use crate::{check, Nicknames, Context, Error};

/// Displays all the nicknames for all characters.
#[poise::command(prefix_command, slash_command)]
pub async fn nicknames(
    ctx: Context<'_>,
) -> Result<(), Error> {

    if (check::adaptive_check(
        ctx,
        (false, &String::new()),
        (false, &String::new()),
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
    let mut nicks_as_msg = "## __**Character Nicknames**__\n".to_string();

    for nicknames in vec_nicknames {
        // Character portion
        nicks_as_msg = nicks_as_msg.to_owned() + "\n- **" + &nicknames.character.to_string() + "**";
        
        // Nickname portion
        nicks_as_msg += "\n\t\t→ `";
        
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
    
    nicks_as_msg += "";
    ctx.say(&nicks_as_msg).await?;
    ctx.channel_id().say(ctx, "Try the `/help notes` command for usage notes and specifics.").await?;
    Ok(())
}
