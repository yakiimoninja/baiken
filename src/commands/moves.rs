use std::fs::{self} ;
use std::path::Path;
use std::string::String;
use crate::serenity::futures::{Stream, StreamExt, self};
use crate::{MoveInfo, check, Nicknames, Context, Error, CHARS, MoveAliases};

// Autocompletes the character name
async fn autocomplete_character<'a>(
    _ctx: Context<'_>,
    partial: &'a str,
) -> impl Stream<Item = String> + 'a {
    futures::stream::iter(&CHARS)
        .filter(move |name| futures::future::ready(name.to_lowercase().contains(&partial.to_lowercase())))
        .map(|name| name.to_string())
}

/// Displays all the moves and inputs of a character.
#[poise::command(prefix_command, slash_command, aliases("m"))]
pub async fn moves(
    ctx: Context<'_>,
    #[description = "Character name or nickname."]
    #[autocomplete = "autocomplete_character"] character: String,
) -> Result<(), Error> {

    // This will store the full character name in case user input was an alias
    let mut character_arg_altered = String::new();
    // Flag that will be used for logic to determine output
    let mut character_found = false;

    if let Err(_) = check::adaptive_check(
        ctx,
        (true, &character),
        (false, &String::new()),
        true,
        true,
        true,
        true,
        false).await {
        
        return Ok(());
    }

    for char in CHARS {

        // Checking if aliases for this characters moves exist
        let aliases_path = "data/".to_owned() + char + "/aliases.json";
        if !Path::new(&aliases_path).exists() {
            // Error message cause a specific file is missing
            let error_msg = "The `".to_owned() + &aliases_path + "` file was not found.";
            ctx.say(&error_msg).await?;
            println!();
            panic!("{}", error_msg.replace('`', "'"));            
        }
    }

    // Reading the nicknames json
    let data_from_file = fs::read_to_string("data/nicknames.json")
        .expect("\nFailed to read 'nicknames.json' file.");
    
    // Deserializing from nicknames json
    let vec_nicknames = serde_json::from_str::<Vec<Nicknames>>(&data_from_file).unwrap();

    // Iterating through the nicknames.json character entries
    if !character_found {
        
        'outer: for x_nicknames in &vec_nicknames {
        
            // Iterating through the nicknames.json nickname entries
            for y_nicknames in &x_nicknames.nicknames {
    
                // If user input equals a character nickname then pass the full character name
                // To the new variable 'character_arg_altered'
                if y_nicknames.to_lowercase() == character.to_lowercase().trim() {
    
                    character_arg_altered = x_nicknames.character.to_owned();
                    character_found = true;
                    break 'outer;
                }   
            }
        }
    }

    if !character_found {
        
        // Iterating through the nicknames.json character entries
        for x_nicknames in &vec_nicknames {

            // If user input is part of a characters full name or the full name itself
            // Then pass the full and correct charactet name to the new var 'character_arg_altered'
            if x_nicknames.character.to_lowercase().replace('-', "").contains(&character.to_lowercase()) ||
            x_nicknames.character.to_lowercase().contains(&character.to_lowercase()) {
                
                character_arg_altered = x_nicknames.character.to_owned();
                character_found = true;
                break;
            }
        }
    }
    
    // If user input isnt the full name, part of a full name or a nickname
    // Error out cause requested character was not found in the json
    if !character_found {
        let error_msg= &("Character `".to_owned() + &character + "` was not found!");
        ctx.say(error_msg).await?;
        println!("\nError: {}", error_msg.replace('`', "'"));
        return Ok(());
    }

    // Reading the character json if found
    let char_file_path = "data/".to_owned() + &character_arg_altered + "/" + &character_arg_altered + ".json";
    let char_file_data = fs::read_to_string(char_file_path)
        .expect(&("\nFailed to read '".to_owned() + &character_arg_altered + ".json" + "' file."));
    
    //Deserializing from character json
    let moves_info = serde_json::from_str::<Vec<MoveInfo>>(&char_file_data).unwrap();

    // Reading the aliases json
    let aliases_path = "data/".to_owned() + &character_arg_altered + "/aliases.json";
    let aliases_data = fs::read_to_string(&aliases_path)
        .expect(&("\nFailed to read '".to_owned() + &aliases_path + "' file."));
    
    // Deserializing the aliases json
    let aliases_data = serde_json::from_str::<Vec<MoveAliases>>(&aliases_data).unwrap();         
    
    println!("\nCommand: '{} {}'", ctx.command().qualified_name, character);
    println!("Successfully read '{}.json' and 'aliases.json' file.", &character_arg_altered);
    
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