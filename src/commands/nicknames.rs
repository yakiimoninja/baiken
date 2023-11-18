use std::fs;
use std::string::String;
use crate::{check, Nicknames, Context, Error};

/// Displays all the nicknames for all characters.
#[poise::command(prefix_command, slash_command, aliases("n"))]
pub async fn nicknames(
    ctx: Context<'_>,
) -> Result<(), Error> {

    // Checking if nicknames.json exist
    if let Some(error_msg) = check::nicknames_json_exists(false) {
        ctx.say(&error_msg.replace('\'', "`")).await?;
        println!();
        panic!("{}", error_msg.replace('\n', " "));
    }

    // Reading the nicknames json
    let data_from_file = fs::read_to_string("data/nicknames.json")
        .expect("\nFailed to read 'nicknames.json' file.");

    // Deserializing from nicknames json
    let vec_nicknames = serde_json::from_str::<Vec<Nicknames>>(&data_from_file).unwrap();
    
    println!("\nCommand: '{}'", ctx.command().qualified_name);
    println!("Successfully read 'nicknames.json' file.");
    
    // Formatting string for in discord print
    let mut nicks_as_msg = "__**Character Nicknames**__\n```diff".to_string();

    for nicknames in vec_nicknames {
        // Character portion
        nicks_as_msg = nicks_as_msg.to_owned() + "\n* Character: " + &nicknames.character.to_string();
        
        // Nickname portion
        nicks_as_msg += "\n+ Nicknames: ";
        
        for x in 0..nicknames.nicknames.len() {
            if x != nicknames.nicknames.len() - 1 {
                // Taking into account the lack of nicknames for some characters
                if !nicknames.nicknames[x].is_empty() {
                    nicks_as_msg = nicks_as_msg + &nicknames.nicknames[x] + ", ";
                }
                else {
                    nicks_as_msg = nicks_as_msg + &nicknames.nicknames[x];
                }
            }
            else {
                nicks_as_msg = nicks_as_msg + &nicknames.nicknames[x];
            }
        }
        nicks_as_msg = nicks_as_msg.to_owned() + ".\n";
    }
    
    nicks_as_msg += "```";
    nicks_as_msg += "";
    ctx.say(&nicks_as_msg).await?;
        
    Ok(())
}