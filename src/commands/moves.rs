use std::{fs, string::String};
use colored::Colorize;
use crate::{Context, Error, MoveInfo, MoveAliases };
use crate::{find, check};

/// Displays all the moves and inputs of a character.
#[poise::command(prefix_command, slash_command, aliases("m"))]
pub async fn moves(
    ctx: Context<'_>,
    #[description = "Character name or nickname."] character: String,
) -> Result<(), Error> {

    println!("{}", ("Command Args: '".to_owned() + &character + "'").purple());

    if (check::adaptive_check(
        ctx,
        (true, &character),
        (false, &String::new()),
        true,
        true,
        true,
        true,
        false).await).is_err() {
        
        return Ok(());
    }

   // Finding character
   let character_arg_altered = match find::find_character(&character).await {
        Ok(character_arg_altered) => character_arg_altered,
        Err(err) => {
            ctx.say(err.to_string()).await?;
            println!("{}", ("Error: ".to_owned() + &err.to_string()).red());
            return Ok(()) }
    };

    // Reading the character json
    let char_file_path = "data/".to_owned() + &character_arg_altered + "/" + &character_arg_altered + ".json";
    let char_file_data = fs::read_to_string(char_file_path)
        .expect(&("\nFailed to read '".to_owned() + &character + ".json" + "' file."));

    // Deserializing from character json
    let moves_info = serde_json::from_str::<Vec<MoveInfo>>(&char_file_data).unwrap();            

    println!("{}", ("Successfully read '".to_owned() + &character_arg_altered + ".json' file.").green());

    // Reading the aliases json
    let aliases_path = "data/".to_owned() + &character_arg_altered + "/aliases.json";
    let aliases_data = fs::read_to_string(&aliases_path)
        .expect(&("\nFailed to read '".to_owned() + &aliases_path + "' file."));

    // Deserializing the aliases json
    let aliases_data = serde_json::from_str::<Vec<MoveAliases>>(&aliases_data).unwrap();

    // Formatting string for in discord print
    let mut moves_as_msg = "__**".to_string() + &character_arg_altered.replace('_', " ") + " Moves / Aliases**__\n```diff";

    // Message split due to discord character limit
    // 1st message builder which is also a reply
    for moves in moves_info.iter().take(moves_info.len() / 4) {
        
        moves_as_msg = moves_as_msg.to_owned() + "\n* Move: "+ &moves.name 
        + " -> Input: " + &moves.input;

        for moves_aliases in aliases_data.iter() {

            // If move input exists in aliases.json
            if moves.input == moves_aliases.input {

                moves_as_msg += "\n+ Aliases: ";

                // Format message if there is only one alias or multiple
                for a in 0..moves_aliases.aliases.len() {
                    if a != moves_aliases.aliases.len() - 1 {
                        moves_as_msg = moves_as_msg.to_owned() + &moves_aliases.aliases[a] + ", ";
                    
                    }
                    else {
                        moves_as_msg = moves_as_msg.to_owned() + &moves_aliases.aliases[a];

                    }
                }
            }
            else {
                continue;
            }
        }
        moves_as_msg = moves_as_msg.to_owned() + ".";
    }
    moves_as_msg += "\n```";
    ctx.say(&moves_as_msg).await?;

    // 2nd message builder
    moves_as_msg = "```diff".to_string();
    for moves in moves_info.iter().take((moves_info.len() / 4) * 2).skip(moves_info.len() / 4) {
        
        moves_as_msg = moves_as_msg.to_owned() + "\n* Move: "+ &moves.name 
        + " -> Input: " + &moves.input;

        for moves_aliases in aliases_data.iter() {

            // If move input exists in aliases.json
            if moves.input == moves_aliases.input {
                
                moves_as_msg += "\n+ Aliases: ";
                
                // Format message if there is only one alias or multiple
                for a in 0..moves_aliases.aliases.len() {
                    if a != moves_aliases.aliases.len() - 1 {
                        moves_as_msg = moves_as_msg.to_owned() + &moves_aliases.aliases[a] + ", ";

                    }
                    else {
                        moves_as_msg = moves_as_msg.to_owned() + &moves_aliases.aliases[a];
                        
                    }
                }
            }
            else {
                continue;
            }
        }
        moves_as_msg = moves_as_msg.to_owned() + ".";
    }
    moves_as_msg += "\n```";
    ctx.channel_id().say(ctx, &moves_as_msg).await?;

    // 3rd message builder
    moves_as_msg = "```diff".to_string();
    for moves in moves_info.iter().take((moves_info.len() / 4 ) * 3).skip((moves_info.len() / 4) * 2) {
        
        moves_as_msg = moves_as_msg.to_owned() + "\n* Move: "+ &moves.name 
        + " -> Input: " + &moves.input;

        for moves_aliases in aliases_data.iter() {

            // If move input exists in aliases.json
            if moves.input == moves_aliases.input {
                
                moves_as_msg += "\n+ Aliases: ";

                // Format message if there is only one alias or multiple
                for a in 0..moves_aliases.aliases.len() {
                    if a != moves_aliases.aliases.len() - 1 {
                        moves_as_msg = moves_as_msg.to_owned() + &moves_aliases.aliases[a] + ", ";
                    
                    }
                    else {
                        moves_as_msg = moves_as_msg.to_owned() + &moves_aliases.aliases[a];

                    }
                }
            }
            else {
                continue;
            }
        }
        moves_as_msg = moves_as_msg.to_owned() + ".";
    }
    moves_as_msg += "\n```";
    ctx.channel_id().say(ctx, &moves_as_msg).await?;

    // 4th message builder
    moves_as_msg = "```diff".to_string();
    for moves in moves_info.iter().skip((moves_info.len() / 4) * 3) {
        
        moves_as_msg = moves_as_msg.to_owned() + "\n* Move: "+ &moves.name 
        + " -> Input: " + &moves.input;

        for moves_aliases in aliases_data.iter() {

            // If move input exists in aliases.json
            if moves.input == moves_aliases.input {
                
                moves_as_msg += "\n+ Aliases: ";

                // Format message if there is only one alias or multiple
                for a in 0..moves_aliases.aliases.len() {
                    if a != moves_aliases.aliases.len() - 1 {
                        moves_as_msg = moves_as_msg.to_owned() + &moves_aliases.aliases[a] + ", ";

                    }
                    else {
                        moves_as_msg = moves_as_msg.to_owned() + &moves_aliases.aliases[a];
                        
                    }
                }
            }
            else {
                continue;
            }
        }
        moves_as_msg = moves_as_msg.to_owned() + ".";
    }
    moves_as_msg += "\n```";

    ctx.channel_id().say(ctx, &moves_as_msg).await?;

    Ok(())
}