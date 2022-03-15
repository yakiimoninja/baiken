use std::fs;
use std::path::Path;
use std::string::String;
use serenity::framework::standard::{macros::command, Args, CommandResult};
use serenity::model::prelude::*;
use serenity::prelude::*;

use crate::{CHARS, MoveAliases, check};

#[command]
#[aliases("a")]
async fn aliases(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {

    // Getting character and move args
    let character_arg = args.single::<String>()?;

    // Checking for correct character argument
    if character_arg.len() < 3 {
        if character_arg.to_lowercase() != "ky" {
            let error_msg = "Invalid character name!";
            msg.channel_id.say(&ctx.http, &error_msg).await?;
            print!("\n");
            panic!("{}", error_msg);
        }            
    }

    // Checking if data folder exists
    if let Some(error_msg) = check::data_folder_exists(false) {
        msg.channel_id.say(&ctx.http, &error_msg.replace("'", "`")).await?;
        print!("\n");
        panic!("{}", error_msg.replace("\n", " "));
    }
    
    // Checking if character folders exist
    if let Some(error_msg) = check::character_folders_exist(false) {
        msg.channel_id.say(&ctx.http, &error_msg.replace("'", "`")).await?;
        print!("\n");
        panic!("{}", error_msg.replace("\n", " "));
    }

    // Checking if character jsons exist
    if let Some(error_msg) = check::character_jsons_exist(false) {
        msg.channel_id.say(&ctx.http, &error_msg.replace("'", "`")).await?;
        print!("\n");
        panic!("{}", error_msg.replace("\n", " "));
    }
 
    for c in 0..CHARS.0.len() {

        // Checking if aliases for this character exist
        let aliases_path = "data/".to_owned() + CHARS.0[c] + "/aliases.json";
        if Path::new(&aliases_path).exists() == false {
            // Error message cause a specific file is missing
            let error_msg = "The `".to_owned() + &aliases_path + "` file was not found.";
            msg.channel_id.say(&ctx.http, &error_msg).await?;
            print!("\n");
            panic!("{}", error_msg.replace("`", "'"));            
        }
    }

    let mut character_found = false;
    for c in 0..CHARS.0.len() {

        // Iterating through the character jsons to find the character requested
        if CHARS.0[c].to_lowercase().replace("-", "").contains(&character_arg.to_lowercase()) == true ||
            CHARS.0[c].to_lowercase().contains(&character_arg.to_lowercase()) == true {          
            
            println!("\nUser input: 'b.a {}'", character_arg);
            println!("Succesfully read '{}.json' file.", &CHARS.0[c]);
            character_found = true;

            // Reading the aliases json
            let aliases_path = "data/".to_owned() + &CHARS.0[c] + "/aliases.json";
            let aliases_data = fs::read_to_string(&aliases_path)
                .expect(&("\nFailed to read '".to_owned() + &aliases_path + "' file."));
            
            // Deserializing the aliases json
            let aliases_data = serde_json::from_str::<Vec<MoveAliases>>(&aliases_data).unwrap();

            // Formatting string for in discord print
            let mut moves_as_msg = "__**".to_string() + &CHARS.0[c].replace("_", " ") + " Move Aliases**__\n```";
            
            // Checks what character info is accessing
            if CHARS.0[c] != "Faust" && CHARS.0[c] != "Goldlewis_Dickinson" && CHARS.0[c] != "Ky_Kiske" {
                
                // Building the message to be sent by the bot
                for m in 0..aliases_data.len() {
                    moves_as_msg = moves_as_msg.to_owned() + "\nMove: "+ &aliases_data[m].aliases[0] 
                        + " -> Input: " + &aliases_data[m].input + "\nAliases: ";
    
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
                msg.channel_id.say(&ctx.http, &moves_as_msg).await?;
            }
            else {
                // Spliting the message that will be sent by the bot
                // Into 2 separate messages cause of the character limit
                for m in 0..aliases_data.len() / 2 {
                    moves_as_msg = moves_as_msg.to_owned() + "\nMove: "+ &aliases_data[m].aliases[0] 
                        + " -> Input: " + &aliases_data[m].input + "\nAliases: ";
    
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
                msg.channel_id.say(&ctx.http, &moves_as_msg).await?;

                // 2nd message builder
                moves_as_msg = "```".to_string();
                for m in aliases_data.len()/2..aliases_data.len() {
                    moves_as_msg = moves_as_msg.to_owned() + "\nMove: "+ &aliases_data[m].aliases[0] 
                        + " -> Input: " + &aliases_data[m].input + "\nAliases: ";
    
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
                msg.channel_id.say(&ctx.http, &moves_as_msg).await?;                
            }
        }
    }

    // Error message cause given characters json was not found
    if character_found == false {
        let error_msg= &("Character `".to_owned() + &character_arg + "` was not found!");
        msg.channel_id.say(&ctx.http, error_msg).await?;
        print!("\n");
        panic!("{}", error_msg.replace("`", "'"));
    }

    Ok(())
}