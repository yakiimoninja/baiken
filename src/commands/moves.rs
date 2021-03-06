use std::fs;
use std::string::String;
use serenity::framework::standard::{macros::command, Args, CommandResult};
use serenity::model::prelude::*;
use serenity::prelude::*;

use crate::{Frames, check, Nicknames};

#[command]
#[aliases("m")]
async fn moves(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {

    // This will store the full character name in case user input was an alias
    let mut character_arg_altered = String::new();
    // Flag that will be used for logic to determine output
    let mut character_found = false;

    // Getting character arg
    let character_arg = args.single::<String>()?;

    // Checking if character user argument is correct
    if let Some(error_msg) = check::correct_character_arg(&character_arg){
        msg.channel_id.say(&ctx.http, &error_msg).await?;
        print!("\n");
        panic!("{}", error_msg);
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

    // Reading the character json if found
    let char_file_path = "data/".to_owned() + &character_arg_altered + "/" + &character_arg_altered + ".json";
    let char_file_data = fs::read_to_string(char_file_path)
        .expect(&("\nFailed to read '".to_owned() + &character_arg_altered + ".json" + "' file."));
    
    //Deserializing from character json
    let move_frames = serde_json::from_str::<Vec<Frames>>(&char_file_data).unwrap();            
    
    println!("\nUser input: 'b.m {}'", character_arg);
    println!("Succesfully read '{}.json' file.", &character_arg_altered);
    
    // Formatting string for in discord print
    let mut moves_as_msg = "__**".to_string() + &character_arg_altered.replace("_", " ") + " Moves**__\n```diff";

    for m in 0..move_frames.len() {
        moves_as_msg = moves_as_msg.to_owned() + "\n* Move: "+ &move_frames[m].r#move
            + "\n+ Input: " + &move_frames[m].input + "\n";
    }

    moves_as_msg = moves_as_msg + &"```".to_string();
    msg.channel_id.say(&ctx.http, &moves_as_msg).await?;
        
    Ok(())
}