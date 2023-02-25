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
        print!("\n");
        panic!("{}", error_msg);
    }

    // Checking if data folder exists
    if let Some(error_msg) = check::data_folder_exists(false) {
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

    // Checking if character jsons exist
    if let Some(error_msg) = check::character_jsons_exist(false) {
        ctx.say(&error_msg.replace("'", "`")).await?;
        print!("\n");
        panic!("{}", error_msg.replace("\n", " "));
    }
 
    for char in CHARS {

        // Checking if aliases for this characters moves exist
        let aliases_path = "data/".to_owned() + char + "/aliases.json";
        if Path::new(&aliases_path).exists() == false {
            // Error message cause a specific file is missing
            let error_msg = "The `".to_owned() + &aliases_path + "` file was not found.";
            ctx.say(&error_msg).await?;
            print!("\n");
            panic!("{}", error_msg.replace("`", "'"));            
        }
    }

    // Reading the nicknames json
    let data_from_file = fs::read_to_string("data/nicknames.json")
        .expect("\nFailed to read 'nicknames.json' file.");
    
    // Deserializing from nicknames json
    let vec_nicknames = serde_json::from_str::<Vec<Nicknames>>(&data_from_file).unwrap();

    // Iterating through the nicknames.json character entries
    if character_found == false {
    
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

    if character_found == false {
        
        // Iterating through the nicknames.json character entries
        for x_nicknames in &vec_nicknames {

            // If user input is part of a characters full name or the full name itself
            // Then pass the full and correct charactet name to the new var 'character_arg_altered'
            if x_nicknames.character.to_lowercase().replace("-", "").contains(&character_arg.to_lowercase()) == true ||
            x_nicknames.character.to_lowercase().contains(&character_arg.to_lowercase()) == true {
                
                character_found = true;
                character_arg_altered = x_nicknames.character.to_owned();
                break;
            }
        }
    }

    // If user input isnt the full name, part of a full name or a nickname
    // Error out cause requested character was not found in the json
    if character_found == false {
        let error_msg= &("Character `".to_owned() + &character_arg + "` was not found!");
        ctx.say(error_msg).await?;
        print!("\n");
        panic!("{}", error_msg.replace("`", "'"));
    }    

    println!("\nCommand: '{} {}'", ctx.command().qualified_name, character_arg);
    println!("Succesfully read '{}.json' file.", &character_arg_altered);

    // Reading the aliases json
    let aliases_path = "data/".to_owned() + &character_arg_altered + "/aliases.json";
    let aliases_data = fs::read_to_string(&aliases_path)
        .expect(&("\nFailed to read '".to_owned() + &aliases_path + "' file."));
    
    // Deserializing the aliases json
    let aliases_data = serde_json::from_str::<Vec<MoveAliases>>(&aliases_data).unwrap();

    // Formatting string for in discord print
    let mut moves_as_msg = "__**".to_string() + &character_arg_altered.replace("_", " ") + " Move Aliases**__\n```diff";
    
    // Checks what character info is accessing to check later cause of discord message character limit
    if character_arg_altered != "Faust" 
        && character_arg_altered != "Goldlewis_Dickinson" 
        && character_arg_altered != "Ky_Kiske" 
        && character_arg_altered != "Leo_Whitefang" 
        && character_arg_altered != "Nagoriyuki" {
        
        // Building the message to be sent by the bot
        for alias_data in aliases_data {
            moves_as_msg = moves_as_msg.to_owned() + "\n* Move: "+ &alias_data.aliases[0] 
                + " -> Input: " + &alias_data.input + "\n+ Aliases: ";

            for a in 0..alias_data.aliases.len() {
                if a != alias_data.aliases.len() - 1 {
                    moves_as_msg = moves_as_msg.to_owned() + &alias_data.aliases[a] + ", ";
                }
                else {
                    moves_as_msg = moves_as_msg.to_owned() + &alias_data.aliases[a];
                }
            }
            moves_as_msg = moves_as_msg.to_owned() + ".\n";
        }
        moves_as_msg = moves_as_msg + &"\n```".to_string();
        ctx.say(&moves_as_msg).await?;
    }
    else {
        // Spliting the message that will be sent by the bot
        // Into 3 separate messages cause of the character limit
        // 1st message builder which is also a reply
        for m in 0..aliases_data.len() / 3 {
            moves_as_msg = moves_as_msg.to_owned() + "\n* Move: "+ &aliases_data[m].aliases[0] 
                + " -> Input: " + &aliases_data[m].input + "\n+ Aliases: ";

            for a in 0..aliases_data[m].aliases.len() {
                if a != aliases_data[m].aliases.len() - 1 {
                    moves_as_msg = moves_as_msg.to_owned() + &aliases_data[m].aliases[a] + ", ";
                }
                else {
                    moves_as_msg = moves_as_msg.to_owned() + &aliases_data[m].aliases[a];
                }
            }
            moves_as_msg = moves_as_msg.to_owned() + ".\n";
        }
        moves_as_msg = moves_as_msg + &"\n```".to_string();
        ctx.say(&moves_as_msg).await?;

        // 2nd message builder
        moves_as_msg = "```diff".to_string();
        for m in aliases_data.len() / 3..(aliases_data.len() /3 ) * 2{
            moves_as_msg = moves_as_msg.to_owned() + "\n* Move: "+ &aliases_data[m].aliases[0] 
                + " -> Input: " + &aliases_data[m].input + "\n+ Aliases: ";

            for a in 0..aliases_data[m].aliases.len() {
                if a != aliases_data[m].aliases.len() - 1 {
                    moves_as_msg = moves_as_msg.to_owned() + &aliases_data[m].aliases[a] + ", ";
                }
                else {
                    moves_as_msg = moves_as_msg.to_owned() + &aliases_data[m].aliases[a];
                }
            }
            moves_as_msg = moves_as_msg.to_owned() + ".\n";
        }
        moves_as_msg = moves_as_msg + &"\n```".to_string();
        ctx.channel_id().say(ctx.discord(), &moves_as_msg).await?;

        // 3rd message builder
        moves_as_msg = "```diff".to_string();
        for m in (aliases_data.len() / 3) * 2..aliases_data.len() {
            moves_as_msg = moves_as_msg.to_owned() + "\n* Move: "+ &aliases_data[m].aliases[0] 
                + " -> Input: " + &aliases_data[m].input + "\n+ Aliases: ";

            for a in 0..aliases_data[m].aliases.len() {
                if a != aliases_data[m].aliases.len() - 1 {
                    moves_as_msg = moves_as_msg.to_owned() + &aliases_data[m].aliases[a] + ", ";
                }
                else {
                    moves_as_msg = moves_as_msg.to_owned() + &aliases_data[m].aliases[a];
                }
            }
            moves_as_msg = moves_as_msg.to_owned() + ".\n";
        }
        moves_as_msg = moves_as_msg + &"\n```".to_string();
        ctx.channel_id().say(ctx.discord(), &moves_as_msg).await?;
    }

    ctx.channel_id().say(ctx.discord(), "You can request the addition of an alias, nickname, feature\nor simply leave feedback by executing the `/request` command.").await?;

    Ok(())
}
