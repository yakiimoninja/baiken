use std::fs;
use std::path::Path;
use std::string::String;
use serenity::framework::standard::{macros::command, Args, CommandResult};
use serenity::model::prelude::*;
use serenity::prelude::*;

use crate::{CHARS, Frames};

#[command]
async fn m (ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {

    // Getting character and move args
    let character = args.single::<String>()?;
    //let mut character_move = args.rest().to_string();
    println!("{}", character);
    // Checking for correct character argument
    if character.len() < 3 {
        if character.to_lowercase() != "ky"{
            let error_msg = "Invalid character name!";
            msg.channel_id.say(&ctx.http, &error_msg).await?;
            print!("\n");
            panic!("{}", error_msg);
        }            
    }

    // Checking if 'frames' folder and character jsons exist
    if Path::new("data/frames").exists() == true{
        for c in 0..CHARS.0.len(){
            let json_path = &("data/frames/".to_owned() + CHARS.0[c] + "/" + CHARS.0[c] + ".json");
            if Path::new(json_path).exists() == true{
                continue;
            } else {
                // Error message cause a specific file is missing
                let error_msg = "The `".to_owned() + json_path + "` file was not found.\nDownload and import the `data` folder from:\nhttps://github.com/yakiimoninja/baiken-bot.";
                msg.channel_id.say(&ctx.http, &error_msg).await?;
                print!("\n");
                panic!("{}", error_msg.replace("`", "'"));
            }
        }
    }
    else{
        // Error message cause 'frames' folder doesnt exist
        let error_msg= "The `data/frames` folder was not found.\nDownload and import the `data` folder from:\nhttps://github.com/yakiimoninja/baiken-bot.";
        msg.channel_id.say(&ctx.http, error_msg).await?;
        print!("\n");
        panic!("{}", error_msg.replace("`", "'"));
    }

    let mut character_found = false;
    for c in 0..CHARS.0.len() {

        // Iterating through the character jsons to find the character requested
        if CHARS.0[c].to_lowercase().replace("-", "").contains(&character.to_lowercase()) == true /*||
            CHARS.0[c].to_lowercase().contains(&character.to_lowercase()) == true */{

            // Reading the character json if found
            let char_file_path = "data/frames/".to_owned() + CHARS.0[c] + "/" + CHARS.0[c] + ".json";
            let char_file_data = fs::read_to_string(char_file_path)
                .expect(&("\nFailed to read '".to_owned() + &CHARS.0[c] + ".json" + "' file."));
            
            //Deserializing from character json
            let move_frames = serde_json::from_str::<Vec<Frames>>(&char_file_data).unwrap();            
            
            println!("\nSuccesfully read '{}.json' file.", &CHARS.0[c]);

            let mut moves_as_msg = "__**".to_string() + &CHARS.0[c].replace("_", " ") + " Moves**__\n```";

            for m in 0..move_frames.len(){
                moves_as_msg = moves_as_msg.to_owned() + "\nMove: "+ &move_frames[m].r#move.to_string() 
                    + " -> Input: " + &move_frames[m].input.to_string() + ".";
            }
            moves_as_msg = moves_as_msg + &"```".to_string();
            msg.channel_id.say(&ctx.http, &moves_as_msg).await?;
            
            character_found = true;
        }
    }

    // Error message cause given characters json was not found
    if character_found == false {
        let error_msg= &("Character `".to_owned() + &character + "` was not found!");
        msg.channel_id.say(&ctx.http, error_msg).await?;
        print!("\n");
        panic!("{}", error_msg.replace("`", "'"));
    }

    Ok(())
}