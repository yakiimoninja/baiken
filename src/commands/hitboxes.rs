use std::fs;
use std::path::Path;
use std::string::String;
use serenity::framework::standard::{macros::command, Args, CommandResult};
use serenity::model::prelude::*;
use serenity::prelude::*;

use crate::{CHARS, Frames, MoveAliases, ImageLinks, check};

#[command]
#[aliases("hitbox","hit", "h")]
async fn hitboxes(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {

    // Getting character and move args
    let character_arg = args.single::<String>()?;
    let mut character_move_arg = args.rest().to_string();

    // Checking for correct character argument
    if character_arg.len() < 3 {
        if character_arg.to_lowercase() != "ky" {
            let error_msg = "Character name: `".to_owned() + &character_arg + "` is invalid!";
            msg.channel_id.say(&ctx.http, &error_msg).await?;
            print!("\n");
            panic!("{}", error_msg);
        }    
    }

    // Checking for correct move argument
    if character_move_arg.len() < 2 {
        let error_msg = "Move: `".to_owned() + &character_move_arg + "` is invalid!";
        msg.channel_id.say(&ctx.http, &error_msg).await?;
        print!("\n");
        panic!("{}", error_msg);
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

    let mut character_found = false;
    let mut move_found = false;

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
            
            println!("\nSuccesfully read '{}.json' file.", &CHARS.0[c]);
            
            character_found = true;

            // Checking if aliases for this character exist
            let aliases_path = "data/".to_owned() + CHARS.0[c] + "/aliases.json";
            if Path::new(&aliases_path).exists() == true {
                
                // Reading the aliases json
                let aliases_data = fs::read_to_string(&aliases_path)
                    .expect(&("\nFailed to read '".to_owned() + &aliases_path + "' file."));
                
                // Deserializing the aliases json
                let aliases_data = serde_json::from_str::<Vec<MoveAliases>>(&aliases_data).unwrap();

                for a in 0..aliases_data.len() {
                    for b in 0..aliases_data[a].aliases.len() {
                        
                        // If the requested argument (character_move) is an alias for any of the moves listed in aliases.json
                        // Change the given argument (character_move) to the actual move name instead of the alias
                        if aliases_data[a].aliases[b].to_lowercase().trim() == character_move_arg.to_lowercase().trim() {
                            character_move_arg = aliases_data[a].input.to_string();
                        }
                    }
                }
            }

            // Reading images.json for this character
            let image_links = fs::read_to_string(&("data/".to_owned() + CHARS.0[c] + "/images.json"))
                .expect(&("\nFailed to read 'data/".to_owned() + CHARS.0[c] + "'/images.json' file."));

            // Deserializing images.json for this character
            let image_links= serde_json::from_str::<Vec<ImageLinks>>(&image_links).unwrap();
            

            for m in 0..move_frames.len() {
                // Iterating through the moves of the json file to find the move requested
                if move_frames[m].input.to_string().to_lowercase().replace(".", "") 
                == character_move_arg.to_string().to_lowercase().replace(".", "")
                || move_frames[m].r#move.to_string().to_lowercase().contains(&character_move_arg.to_string().to_lowercase()) == true {
                    
                    for y in 0..image_links.len() {
                        // Iterating through the image.json to find the move's hitbox links
                        if move_frames[m].input == image_links[y].r#move {

                            move_found = true;
                            println!("Succesfully read move '{}' in '{}.json' file.", &move_frames[m].input.to_string(), &CHARS.0[c]);

                            // Priting hitboxes in discord chat
                            let bot_msg = "__**Move: ".to_owned() + &image_links[m].r#move + "**__";
                            msg.channel_id.say(&ctx.http, &bot_msg).await?;

                            for i in 0..image_links[y].hitbox_img.len() {                        
                                msg.channel_id.say(&ctx.http, &image_links[y].hitbox_img[i]).await?;
                            }
                        }
                    }
                    break;
                }
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
    // Error message cause given move wasnt found in the json
    if character_found == true && move_found == false {
        let error_msg= &("Move `".to_owned() + &character_move_arg + "` was not found!");
        msg.channel_id.say(&ctx.http, error_msg).await?;
        print!("\n");
        panic!("{}", error_msg.replace("`", "'"));
    }

    Ok(())
}