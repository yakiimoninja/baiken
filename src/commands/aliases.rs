use std::fs;
use std::path::Path;
use std::string::String;
use crate::{CHARS, MoveAliases, check, Nicknames, Context, Error};

/// Displays all the aliases for each normal/special/super move of a character.
#[poise::command(prefix_command, slash_command, aliases("a"))]
pub async fn aliases(
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

    // Checking if data folder exists
    if let Some(error_msg) = check::data_folder_exists(false) {
        ctx.say(&error_msg.replace('\'', "`")).await?;
        println!();
        panic!("{}", error_msg.replace('\n', " "));
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
                if y_nicknames.to_lowercase() == character_arg.to_lowercase().trim() {
    
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
            if x_nicknames.character.to_lowercase().replace('-', "").contains(&character_arg.to_lowercase()) ||
            x_nicknames.character.to_lowercase().contains(&character_arg.to_lowercase()) {
                
                character_arg_altered = x_nicknames.character.to_owned();
                character_found = true;
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
        return Ok(())
    }    

    println!("\nCommand: '{} {}'", ctx.command().qualified_name, character_arg);
    println!("Successfully read '{}.json' file.", &character_arg_altered);

    // Reading the aliases json
    let aliases_path = "data/".to_owned() + &character_arg_altered + "/aliases.json";
    let aliases_data = fs::read_to_string(&aliases_path)
        .expect(&("\nFailed to read '".to_owned() + &aliases_path + "' file."));
    
    // Deserializing the aliases json
    let aliases_data = serde_json::from_str::<Vec<MoveAliases>>(&aliases_data).unwrap();

    // Formatting string for in discord print
    let mut moves_as_msg = "__**".to_string() + &character_arg_altered.replace('_', " ") + " Move Aliases**__\n```diff";
    
    // Spliting the message that will be sent by the bot
    // Into 3 separate messages cause of the character limit
    // 1st message builder which is also a reply
    for moves in aliases_data.iter().take(aliases_data.len() / 3) {
        moves_as_msg = moves_as_msg.to_owned() + "\n* Move: "+ &moves.aliases[0] 
            + " -> Input: " + &moves.input + "\n+ Aliases: ";

        for a in 0..moves.aliases.len() {
            if a != moves.aliases.len() - 1 {
                moves_as_msg = moves_as_msg.to_owned() + &moves.aliases[a] + ", ";
            }
            else {
                moves_as_msg = moves_as_msg.to_owned() + &moves.aliases[a];
            }
        }
        moves_as_msg = moves_as_msg.to_owned() + ".\n";
    }
    moves_as_msg += "\n```";
    ctx.say(&moves_as_msg).await?;

    // 2nd message builder
    moves_as_msg = "```diff".to_string();
    for moves in aliases_data.iter().take((aliases_data.len() / 3 ) * 2).skip(aliases_data.len() / 3) {
        moves_as_msg = moves_as_msg.to_owned() + "\n* Move: "+ &moves.aliases[0] 
            + " -> Input: " + &moves.input + "\n+ Aliases: ";

        for a in 0..moves.aliases.len() {
            if a != moves.aliases.len() - 1 {
                moves_as_msg = moves_as_msg.to_owned() + &moves.aliases[a] + ", ";
            }
            else {
                moves_as_msg = moves_as_msg.to_owned() + &moves.aliases[a];
            }
        }
        moves_as_msg = moves_as_msg.to_owned() + ".\n";
    }
    moves_as_msg += "\n```";
    ctx.channel_id().say(ctx, &moves_as_msg).await?;

    // 3rd message builder
    moves_as_msg = "```diff".to_string();
    for moves in aliases_data.iter().skip((aliases_data.len() / 3 ) * 2) {
        moves_as_msg = moves_as_msg.to_owned() + "\n* Move: "+ &moves.aliases[0] 
            + " -> Input: " + &moves.input + "\n+ Aliases: ";

        for a in 0..moves.aliases.len() {
            if a != moves.aliases.len() - 1 {
                moves_as_msg = moves_as_msg.to_owned() + &moves.aliases[a] + ", ";
            }
            else {
                moves_as_msg = moves_as_msg.to_owned() + &moves.aliases[a];
            }
        }
        moves_as_msg = moves_as_msg.to_owned() + ".\n";
    }
    moves_as_msg += "\n```";
    
    ctx.channel_id().say(ctx, &moves_as_msg).await?;

    ctx.channel_id().say(ctx, "You can request the addition of an alias, nickname, feature\nor simply leave feedback by executing the `/request` command.").await?;

    Ok(())
}
