// use std::fs::File;
// use std::io::Write;
use std::fs;
use std::path::Path;
use std::string::String;
use serenity::framework::standard::{macros::command, Args, CommandResult};
use serenity::model::prelude::*;
use serenity::prelude::*;

use crate::{Frames, MoveAliases, ImageLinks, Nicknames, check};

#[command]
#[aliases("f")]
async fn frames(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {

    // This will store the full character name in case user input was an alias
    let mut character_arg_altered = String::new();
    // Flags that will be used for logic to determine output
    let mut character_found = false;
    let mut move_found = false;

    // Getting character and move args
    let character_arg = args.single::<String>()?;
    let mut character_move_arg = args.rest().to_string();

    // Checking if character user argument is correct
    if let Some(error_msg) = check::correct_character_arg(&character_arg){
        msg.channel_id.say(&ctx.http, &error_msg).await?;
        print!("\n");
        panic!("{}", error_msg);
    }

    // Checking if move user argument is correct
    if let Some(error_msg) = check::correct_character_move_arg(&character_move_arg){
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

    // Initializing variables for the embed
    // They must not be empty cause then the embed wont send
    let mut image_embed = "https://raw.githubusercontent.com/yakiimoninja/baiken/main/src/commands/images/no_image.png".to_string();
    let mut damage_embed = "-".to_string();
    let mut guard_embed = "-".to_string();
    let mut invin_embed = "-".to_string();
    let mut startup_embed = "-".to_string();
    let mut hit_embed = "-".to_string();
    let mut block_embed = "-".to_string();
    let mut active_embed = "-".to_string();
    let mut recovery_embed = "-".to_string();
    let mut counter_embed = "-".to_string();

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
        msg.channel_id.say(&ctx.http, error_msg).await?;
        print!("\n");
        panic!("{}", error_msg.replace("`", "'"));
    }

    // Reading the character json
    let char_file_path = "data/".to_owned() + &character_arg_altered + "/" + &character_arg_altered + ".json";
    let char_file_data = fs::read_to_string(char_file_path)
        .expect(&("\nFailed to read '".to_owned() + &character_arg + ".json" + "' file."));
    
    // Deserializing from character json
    let move_frames = serde_json::from_str::<Vec<Frames>>(&char_file_data).unwrap();            
    
    println!("\nUser input: 'b.f {} {}'", character_arg, character_move_arg);
    println!("Succesfully read '{}.json' file.", character_arg_altered);
    

    // Checking if aliases for this character exist
    let aliases_path = "data/".to_owned() + &character_arg_altered + "/aliases.json";
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
    let image_links = fs::read_to_string(&("data/".to_owned() + &character_arg_altered + "/images.json"))
        .expect(&("\nFailed to read 'data/".to_owned() + &character_arg_altered + "'/images.json' file."));

    // Deserializing images.json for this character
    let image_links= serde_json::from_str::<Vec<ImageLinks>>(&image_links).unwrap();
    

    for m in 0..move_frames.len() {
        
        // Iterating through the moves of the json file to find the move requested
        if move_frames[m].input.to_string().to_lowercase().replace(".", "") 
        == character_move_arg.to_string().to_lowercase().replace(".", "")
        || move_frames[m].r#move.to_string().to_lowercase().contains(&character_move_arg.to_string().to_lowercase()) == true {
            
            
            move_found = true;
            println!("Succesfully read move '{}' in '{}.json' file.", &move_frames[m].input.to_string(), character_arg_altered);

            let content_embed = "https://dustloop.com/wiki/index.php?title=GGST/".to_owned() + &character_arg_altered + "/Frame_Data";
            let title_embed = "Move: ".to_owned() + &move_frames[m].input.to_string();

            // Checking if the respective data field in the json file is empty
            // If they aren't empty, the variables initialized above will be replaced
            // With the corresponind data from the json file
            // Otherwise they will remain as '-'
            for y in 0..image_links.len() {
                // Iterating through the image.json to find the move's image links
                if move_frames[m].input == image_links[y].input {
                    if image_links[y].move_img.is_empty() == false {
                        image_embed = image_links[y].move_img.to_string();
                        break;
                    }
                }
            }
            if move_frames[m].damage.is_empty() == false {
                damage_embed = move_frames[m].damage.to_string();
            }
            if move_frames[m].guard.is_empty() == false {
                guard_embed = move_frames[m].guard.to_string();
            }
            if move_frames[m].invincibility.is_empty() == false {
                invin_embed = move_frames[m].invincibility.to_string();
            }
            if move_frames[m].startup.is_empty() == false {
                startup_embed = move_frames[m].startup.to_string();
            }
            if move_frames[m].hit.is_empty() == false {
                hit_embed = move_frames[m].hit.to_string();
            }
            if move_frames[m].block.is_empty() == false {
                block_embed = move_frames[m].block.to_string();
            }
            if move_frames[m].active.is_empty() == false {
                active_embed = move_frames[m].active.to_string();
            }
            if move_frames[m].recovery.is_empty() == false {
                recovery_embed = move_frames[m].recovery.to_string();
            }
            if move_frames[m].counter.is_empty() == false {
            counter_embed = move_frames[m].counter.to_string();
            }

            // Debugging prints
            // println!("{}", content_embed);
            //println!("{}", image_embed);
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

            // New version notification
            //msg.channel_id.say(&ctx.http, r"New version 0.4.0 is out!
//Take a peek at the patch notes here.
//__<https://github.com/yakiimoninja/baiken/blob/main/patch_notes/v0.4.0.md>__").await?;

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

    // Error message cause given move wasnt found in the json
    if character_found == true && move_found == false {
        let error_msg= &("Move `".to_owned() + &character_move_arg + "` was not found!");
        msg.channel_id.say(&ctx.http, error_msg).await?;
        msg.channel_id.say(&ctx.http, "You can request the addition of a non-existing alias by executing\nthe `b.r` command followed by the character then move and lastly alias.\nExample: `b.r giovanna 236k arrow`.").await?;
        print!("\n");
        panic!("{}", error_msg.replace("`", "'"));
    }

    Ok(())
}