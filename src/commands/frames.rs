// use std::fs::File;
// use std::io::Write;
use std::fs;
use std::path::Path;
use std::string::String;
use serenity::framework::standard::{macros::command, Args, CommandResult};
use serenity::model::prelude::*;
use serenity::prelude::*;

use crate::{CHARS, Frames};
// use crate::CharFrames;

#[command]
async fn f (ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {

    // Getting character and move args
    let character = args.single::<String>()?;
    let character_move = args.single::<String>()?;


    // Checking for correct character argument
    if character.len() < 3 {
        if character.to_lowercase() == "ky"{}
        else{
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
            if Path::new(&("data/frames/".to_owned()+ CHARS.0[c]+".json")).exists() == true{
                continue;
            } else {
                // Error message cause a specific file is missing
                let error_msg = "The '".to_owned() + &("data/frames/".to_owned()+ CHARS.0[c]+".json") + "' file was not found.\nPlease import all character '.txt' files to 'data/images' folder.";
                msg.channel_id.say(&ctx.http, &error_msg).await?;
                print!("\n");
                panic!("{}", error_msg);
            }
        }
    }
    else{
        // Error message cause 'frames' folder doesnt exist
        let error_msg= "The 'data/frames' folder was not found.\nPlease execute the `b.update` command.";
        msg.channel_id.say(&ctx.http, error_msg).await?;
        print!("\n");
        panic!("{}", error_msg);
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

    for c in 0..CHARS.0.len() {

        // Iterating through the character jsons to find the character requested
        if CHARS.0[c].to_lowercase().replace("-", "").contains(&character.to_lowercase()) == true{

            // Reading the character json if found
            let char_file_path = "data/frames/".to_owned() + &CHARS.0[c].to_string() + ".json";
            let char_file_data = fs::read_to_string(char_file_path)
                .expect(&("\nFailed to read '".to_owned() + &CHARS.0[c].to_string() + ".json" + "' file."));
            
            //Deserializing from character json
            let move_frames = serde_json::from_str::<Vec<Frames>>(&char_file_data).unwrap();            
            
            println!("\nSuccesfully read '{}.json' file.", &CHARS.0[c].to_string());

            for m in 0..move_frames.len(){
                // Iterating through the moves of the json file to find the move requested
                if move_frames[m].r#move.to_string().to_lowercase().replace(".", "") 
                == character_move.to_string().to_lowercase().replace(".", ""){
                    
                    let content_embed = "https://dustloop.com/wiki/index.php?title=GGST/".to_owned() + &CHARS.0[c].to_string() + "/Frame_Data";
                    let title_embed = "Move: ".to_owned() + &move_frames[m].r#move.to_string();

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
    Ok(())
}
