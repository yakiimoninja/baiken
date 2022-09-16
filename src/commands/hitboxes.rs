use std::fs;
use std::path::Path;
use std::string::String;
use crate::{Frames, MoveAliases, ImageLinks, check, Nicknames, Context, Error};

const IMAGE_DEFAULT: &str = "https://raw.githubusercontent.com/yakiimoninja/baiken/main/data/images/no_hitbox.png";

/// Prints the hitbox images for a character's move.
#[poise::command(prefix_command, slash_command, aliases("h"))]
pub async fn hitboxes(
    ctx: Context<'_>,
    #[description = "Character name or nickname."] character_arg: String,
    #[description = "Move name, input or alias."] mut character_move_arg: String,
) -> Result<(), Error> {

    // This will store the full character name in case user input was an alias
    let mut character_arg_altered = String::new();
    // Flags that will be used for logic to determine output
    let mut character_found = false;
    let mut move_found = false;

    // Checking if character user argument is correct
    if let Some(error_msg) = check::correct_character_arg(&character_arg){
        ctx.say(&error_msg).await?;
        print!("\n");
        panic!("{}", error_msg);
    }

    // Checking if move user argument is correct
    if let Some(error_msg) = check::correct_character_move_arg(&character_move_arg){
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
   

    // Reading the character json if found
    let char_file_path = "data/".to_owned() + &character_arg_altered + "/" + &character_arg_altered + ".json";
    let char_file_data = fs::read_to_string(char_file_path)
        .expect(&("\nFailed to read '".to_owned() + &character_arg_altered + ".json" + "' file."));
    
    //Deserializing from character json
    let move_frames = serde_json::from_str::<Vec<Frames>>(&char_file_data).unwrap();            
    
    println!("\nUser input: 'b.h {} {}'", character_arg, character_move_arg);
    println!("Succesfully read '{}.json' file.", &character_arg_altered);
    
    character_found = true;

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
            
            for y in 0..image_links.len() {
                // Iterating through the image.json to find the move's hitbox links
                if move_frames[m].input == image_links[y].input {

                    move_found = true;
                    println!("Succesfully read move '{}' in '{}.json' file.", &move_frames[m].input.to_string(), &character_arg_altered);

                    
                    if image_links[y].hitbox_img[0].is_empty() == false {

                        // Priting hitboxes in discord chat
                        let bot_msg = "__**Move: ".to_owned() + &image_links[y].input + "**__";
                        ctx.say(&bot_msg).await?;

                        for i in 0..image_links[y].hitbox_img.len() {                        
                            ctx.say(&image_links[y].hitbox_img[i]).await?;
                        }
                    }
                    else{

                        // Priting hitboxes in discord chat
                        let bot_msg = "__**Move: ".to_owned() + &image_links[m].input + "**__";
                        ctx.say(&bot_msg).await?;
                        ctx.say(&*IMAGE_DEFAULT).await?;
                    }
                    
                }
            }
            break;
        }
    }



    // Error message cause given characters json was not found
    if character_found == false {
        let error_msg= &("Character `".to_owned() + &character_arg + "` was not found!");
        ctx.say(error_msg).await?;
        print!("\n");
        panic!("{}", error_msg.replace("`", "'"));
    }
    // Error message cause given move wasnt found in the json
    if character_found == true && move_found == false {
        let error_msg= &("Move `".to_owned() + &character_move_arg + "` was not found!");
        ctx.say(error_msg).await?;
        ctx.say("You can request something or leave feedback by executing the `/request` command.").await?;
        print!("\n");
        panic!("{}", error_msg.replace("`", "'"));
    }

    Ok(())
}