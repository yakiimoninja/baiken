use std::fs;
use std::string::String;
use crate::{MoveInfo, check, Nicknames, Context, Error};

/// Displays all the moves and inputs of a character.
#[poise::command(prefix_command, slash_command, aliases("m"))]
pub async fn moves(
    ctx: Context<'_>,
    #[description = "Character name or nickname."] character_arg: String,
) -> Result<(), Error> {

    // This will store the full character name in case user input was an alias
    let mut character_arg_altered = String::new();
    // Flag that will be used for logic to determine output
    let mut character_found = false;

    // Checking if character user argument is correct
    if let Some(error_msg) = check::correct_character_arg(&character_arg){
        ctx.say(&error_msg).await?;
        println!("\nError: {}", error_msg);
        return Ok(());
    }

    // Checking if character folders exist
    if let Some(error_msg) = check::character_folders_exist(false) {
        ctx.say(&error_msg.replace('\'', "`")).await?;
        println!();
        panic!("{}", error_msg.replace('\n', " "));
    }
    
    // Checking if character jsons exist
    if let Some(error_msg) = check::character_jsons_exist(false) {
        ctx.say(&error_msg.replace('\'', "`")).await?;
        println!();
        panic!("{}", error_msg.replace('\n', " "));
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
                if y_nicknames.to_lowercase() == character_arg.to_lowercase().trim() {
    
                    character_found = true;
                    character_arg_altered = x_nicknames.character.to_owned();
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
            if x_nicknames.character.to_lowercase().replace('-', "").contains(&character_arg.to_lowercase()) ||
            x_nicknames.character.to_lowercase().contains(&character_arg.to_lowercase()) {
                
                character_found = true;
                character_arg_altered = x_nicknames.character.to_owned();
                break;
            }
        }
    }
    
    // If user input isnt the full name, part of a full name or a nickname
    // Error out cause requested character was not found in the json
    if !character_found {
        let error_msg= &("Character `".to_owned() + &character_arg + "` was not found!");
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
    
    println!("\nCommand: '{} {}'", ctx.command().qualified_name, character_arg);
    println!("Successfully read '{}.json' file.", &character_arg_altered);
    
    // Formatting string for in discord print
    let mut moves_as_msg = "__**".to_string() + &character_arg_altered.replace('_', " ") + " Moves**__\n```diff";

    // Message split due to discord character limit
    // 1st message builder which is also a reply
    for moves in moves_info.iter().take(moves_info.len() / 2) {
        moves_as_msg = moves_as_msg.to_owned() + "\n* Move: "+ &moves.name
            + "\n+ Input: " + &moves.input + "\n";
    }
    moves_as_msg += "```";

    ctx.say(&moves_as_msg).await?;

    // 2nd message builder
    moves_as_msg = "```diff".to_string();
    for moves in moves_info.iter().skip(moves_info.len() / 2) {
        moves_as_msg = moves_as_msg.to_owned() + "\n* Move: "+ &moves.name
            + "\n+ Input: " + &moves.input + "\n";
    }
    moves_as_msg += "```";
    
    ctx.channel_id().say(ctx, &moves_as_msg).await?;
    
    Ok(())
}