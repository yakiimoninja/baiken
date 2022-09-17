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
 
    for c in 0..CHARS.len() {

        // Checking if aliases for this characters moves exist
        let aliases_path = "data/".to_owned() + CHARS[c] + "/aliases.json";
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
    for x in 0..vec_nicknames.len() {

        // If user input is part of a characters full name or the full name itself
        // Then pass the full name to the new var 'character_arg_altered'
        if vec_nicknames[x].character.to_lowercase().replace("-", "").contains(&character_arg.to_lowercase()) == true ||
        vec_nicknames[x].character.to_lowercase().contains(&character_arg.to_lowercase()) == true {
            
            character_found = true;
            character_arg_altered = vec_nicknames[x].character.to_owned();
            break;
        }

        // Iterating through the nicknames.json nickname entries
        for y in 0..vec_nicknames[x].nicknames.len(){

            // If user input equals a character nickname then pass the full character name
            // To the new variable 'character_arg_altered'
            if vec_nicknames[x].nicknames[y].to_lowercase() == character_arg.to_lowercase().trim() {

                character_found = true;
                character_arg_altered = vec_nicknames[x].character.to_owned();
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
    
    // Checks what character info is accessing
    if character_arg_altered != "Faust" && character_arg_altered != "Goldlewis_Dickinson" && character_arg_altered != "Ky_Kiske" {
        
        // Building the message to be sent by the bot
        for m in 0..aliases_data.len() {
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
    }
    else {
        // Spliting the message that will be sent by the bot
        // Into 3 separate messages cause of the character limit
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
        ctx.say(&moves_as_msg).await?;

        // 3nd message builder
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
        ctx.say(&moves_as_msg).await?;
    }

    ctx.say("You can request the addition of a non-existing alias by executing\nthe `b.r` command followed by the character, then the move and lastly the alias you want added.\nExample: `b.r giovanna 236k arrow`.").await?;

    Ok(())
}