// use std::fs::File;
// use std::io::Write;
use std::fs;
use std::path::Path;
use std::string::String;
use serenity::framework::standard::{macros::command, Args, CommandResult};
use serenity::model::prelude::*;
use serenity::prelude::*;

use crate::{CHARS, Frames, MoveAliases};
// use crate::CharFrames;

#[command]
async fn f (ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {

    // Getting character and move args
    let character = args.single::<String>()?;
    let mut character_move = args.rest().to_string();

    // Checking for correct character argument
    if character.len() < 3 {
        if character.to_lowercase() != "ky"{
            let error_msg = "Invalid character name!";
            msg.channel_id.say(&ctx.http, &error_msg).await?;
            print!("\n");
            panic!("{}", error_msg);
        }            
    }

    // Checking for correct move argument
    if character_move.len() < 2 {
        let error_msg = "Invalid move!";
        msg.channel_id.say(&ctx.http, &error_msg).await?;
        print!("\n");
        panic!("{}", error_msg);
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

    // Initializing variables for the embed
    // They must not be empty cause then the embed wont send
    let mut image_embed = "-".to_string();
    let mut damage_embed = "-".to_string();
    let mut guard_embed = "-".to_string();
    let mut invin_embed = "-".to_string();
    let mut startup_embed = "-".to_string();
    let mut hit_embed = "-".to_string();
    let mut block_embed = "-".to_string();
    let mut active_embed = "-".to_string();
    let mut recovery_embed = "-".to_string();
    let mut counter_embed = "-".to_string();

    let mut character_found = false;
    let mut move_found = false;

    for c in 0..CHARS.0.len() {

        // Iterating through the character jsons to find the character requested
        if CHARS.0[c].to_lowercase().replace("-", "").contains(&character.to_lowercase()) == true ||
            CHARS.0[c].to_lowercase().contains(&character.to_lowercase()) == true{

            // Reading the character json if found
            let char_file_path = "data/frames/".to_owned() + CHARS.0[c] + "/" + CHARS.0[c] + ".json";
            let char_file_data = fs::read_to_string(char_file_path)
                .expect(&("\nFailed to read '".to_owned() + &CHARS.0[c] + ".json" + "' file."));
            
            //Deserializing from character json
            let move_frames = serde_json::from_str::<Vec<Frames>>(&char_file_data).unwrap();            
            
            println!("\nSuccesfully read '{}.json' file.", &CHARS.0[c]);
            
            character_found = true;

            // Checking if aliases for this character exist
            let aliases_path = "data/frames/".to_owned() + CHARS.0[c] + "/aliases.json";
            if Path::new(&aliases_path).exists() == true{
                
                // Reading the aliases json
                let aliases_data = fs::read_to_string(&aliases_path)
                    .expect(&("\nFailed to read '".to_owned() + &aliases_path + "' file."));
                
                // Deserializing the aliases json
                let aliases_data = serde_json::from_str::<Vec<MoveAliases>>(&aliases_data).unwrap();

                for a in 0..aliases_data.len(){
                    for b in 0..aliases_data[a].aliases.len(){
                        
                        // If the requested argument (character_move) is an alias for any of the moves listed in aliases.json
                        // Change the given argument (character_move) to the actual move name instead of the alias
                        if aliases_data[a].aliases[b].to_lowercase().trim() == character_move.to_lowercase().trim() {
                            character_move = aliases_data[a].input.to_string();
                        }
                    }
                }
            }
            

            for m in 0..move_frames.len(){
                // Iterating through the moves of the json file to find the move requested
                if move_frames[m].input.to_string().to_lowercase().replace(".", "") 
                == character_move.to_string().to_lowercase().replace(".", "")
                || move_frames[m].r#move.to_string().to_lowercase().contains(&character_move.to_string().to_lowercase()) == true{
                    
                    println!("Succesfully read move '{}' in '{}.json' file.", &move_frames[m].input.to_string(), &CHARS.0[c]);

                    move_found = true;

                    let content_embed = "https://dustloop.com/wiki/index.php?title=GGST/".to_owned() + &CHARS.0[c] + "/Frame_Data";
                    let title_embed = "Move: ".to_owned() + &move_frames[m].input.to_string();

                    // Checking if the respective data field in the json file is empty
                    // If they aren't empty, the variables initialized above will be replaced
                    // With the corresponind data from the json file
                    // Otherwise they will remain as '-'
                    if move_frames[m].img_link.to_string().is_empty() == false{
                        image_embed = move_frames[m].img_link.to_string();
                    }
                    if move_frames[m].damage.to_string().is_empty() == false{
                        damage_embed = move_frames[m].damage.to_string();
                    }
                    if move_frames[m].guard.to_string().is_empty() == false {
                        guard_embed = move_frames[m].guard.to_string();
                    }
                    if move_frames[m].invincibility.to_string().is_empty() == false {
                        invin_embed = move_frames[m].invincibility.to_string();
                    }
                    if move_frames[m].startup.to_string().is_empty() == false {
                        startup_embed = move_frames[m].startup.to_string();
                    }
                    if move_frames[m].hit.to_string().is_empty() == false {
                        hit_embed = move_frames[m].hit.to_string();
                    }
                    if move_frames[m].block.to_string().is_empty() == false {
                        block_embed = move_frames[m].block.to_string();
                    }
                    if move_frames[m].active.to_string().is_empty() == false {
                        active_embed = move_frames[m].active.to_string();
                    }
                    if move_frames[m].recovery.to_string().is_empty() == false {
                        recovery_embed = move_frames[m].recovery.to_string();
                    }
                    if move_frames[m].counter.to_string().is_empty() == false {
                    counter_embed = move_frames[m].counter.to_string();
                    }

                    // Debugging prints
                    // println!("{}", content_embed);
                    // println!("{}", image_embed);
                    // println!("{}", title_embed);
                    // println!("{}", damage_embed);
                    // println!("{}", guard_embed);
                    // println!("{}", invin_embed);
                    // println!("{}", startup_embed);
                    // println!("{}", hit_embed);
                    // println!("{}", block_embed);
                    // println!("{}", active_embed);
                    // println!("{}", recovery_embed);
                    // println!("{}", counter_embed);

                    // Sending the data as an embed
                    let _msg = msg.channel_id.send_message(&ctx.http, |m| {
                        m.content(&content_embed);
                        m.embed(|e| {
                            e.title(&title_embed);
                            //e.description("This is a description");
                            e.image(&image_embed);
                            e.fields(vec![("Damage", &damage_embed, true),
                                        ("Guard", &guard_embed, true),
                                        ("Invinciblity", &invin_embed, true),
                                        ("Startup", &startup_embed, true),
                                        ("On Hit", &hit_embed, true),
                                        ("On Block", &block_embed, true),
                                        ("Active", &active_embed, true),
                                        ("Recovery", &recovery_embed, true),
                                        ("Counter", &counter_embed, true)]);
                            //e.field("This is the third field", "This is not an inline field", false);
                            e
                        });
                        m
                    }).await;

                    break;
                }
            }
        }
    }

    // Error message cause given characters json was not found
    if character_found == false {
        let error_msg= &("Character `".to_owned() + &character + "` was not found!");
        msg.channel_id.say(&ctx.http, error_msg).await?;
        print!("\n");
        panic!("{}", error_msg.replace("`", "'"));
    }
    // Error message cause given move wasnt found in the json
    if character_found == true && move_found == false {
        let error_msg= &("Move `".to_owned() + &character_move + "` was not found!");
        msg.channel_id.say(&ctx.http, error_msg).await?;
        print!("\n");
        panic!("{}", error_msg.replace("`", "'"));
    }

    Ok(())
}

