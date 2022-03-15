use std::fs;
use std::string::String;
use serenity::framework::standard::{macros::command, Args, CommandResult};
use serenity::model::prelude::*;
use serenity::prelude::*;

use crate::{CHARS, Frames, check};

#[command]
#[aliases("m")]
async fn moves(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {

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

    let mut character_found = false;
    for c in 0..CHARS.0.len() {

        // Iterating through the character jsons to find the character requested
        if CHARS.0[c].to_lowercase().replace("-", "").contains(&character_arg.to_lowercase()) == true ||
            CHARS.0[c].to_lowercase().contains(&character_arg.to_lowercase()) == true {

            // Reading the character json if found
            let char_file_path = "data/".to_owned() + CHARS.0[c] + "/" + CHARS.0[c] + ".json";
            let char_file_data = fs::read_to_string(char_file_path)
                .expect(&("\nFailed to read '".to_owned() + &CHARS.0[c] + ".json" + "' file."));
            
            //Deserializing from character json
            let move_frames = serde_json::from_str::<Vec<Frames>>(&char_file_data).unwrap();            
            
            println!("\nUser input: 'b.m {}'", character_arg);
            println!("Succesfully read '{}.json' file.", &CHARS.0[c]);
            character_found = true;
            
            // Formatting string for in discord print
            let mut moves_as_msg = "__**".to_string() + &CHARS.0[c].replace("_", " ") + " Moves**__\n```";

            for m in 0..move_frames.len() {
                moves_as_msg = moves_as_msg.to_owned() + "\nMove: "+ &move_frames[m].r#move
                    + "\nInput: " + &move_frames[m].input + "\n";
            }
            moves_as_msg = moves_as_msg + &"```".to_string();
            msg.channel_id.say(&ctx.http, &moves_as_msg).await?;
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