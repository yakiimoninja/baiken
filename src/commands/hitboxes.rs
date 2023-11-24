use std::fs;
use std::path::Path;
use std::string::String;
use crate::serenity::futures::{Stream, StreamExt, self};
use crate::{MoveInfo, MoveAliases, ImageLinks, Nicknames, Context, Error, HITBOX_DEFAULT, CHARS, check};

// Autocompletes the character name
async fn autocomplete_character<'a>(
    _ctx: Context<'_>,
    partial: &'a str,
) -> impl Stream<Item = String> + 'a {
    futures::stream::iter(&CHARS)
        .filter(move |name| futures::future::ready(name.to_lowercase().contains(&partial.to_lowercase())))
        .map(|name| name.to_string())
}

/// Displays the hitbox images of a character's move.
#[allow(unused_assignments)]
#[poise::command(prefix_command, slash_command, aliases("h"))]
pub async fn hitboxes(
    ctx: Context<'_>,
    #[description = "Character name or nickname."]
    #[autocomplete = "autocomplete_character"] character: String,
    #[description = "Move name, input or alias."] mut character_move: String,
) -> Result<(), Error> {

    println!("Command Args: '{}, {}'", character, character_move);

    // This will store the full character name in case user input was an alias
    let mut character_arg_altered = String::new();
    // Flags that will be used for logic to determine output
    let mut character_found = false;
    let mut move_found = false;

    if (check::adaptive_check(
        ctx,
        (true, &character),
        (true, &character_move),
        true,
        true,
        true,
        true,
        true).await).is_err() {
        
        return Ok(());
    }

    // Reading the nicknames json
    let data_from_file = fs::read_to_string("data/nicknames.json")
    .expect("\nFailed to read 'nicknames.json' file.");

    // Deserializing from nicknames json
    let vec_nicknames = serde_json::from_str::<Vec<Nicknames>>(&data_from_file).unwrap();

    // Iterating through the nicknames.json character entries
    if !character_found {
        
        'outer: for x_nicknames in &vec_nicknames {
        
            // Iterating through the nicknames.json nickname entries
            for y_nicknames in &x_nicknames.nicknames {
    
                // If user input equals a character nickname then pass the full character name
                // To the new variable 'character_arg_altered'
                if y_nicknames.to_lowercase() == character.to_lowercase().trim() {
    
                    character_found = true;
                    character_arg_altered = x_nicknames.character.to_owned();
                    break 'outer;
                }   
            }
        }
    }

    if !character_found {
        
        // Iterating through the nicknames.json character entries
        for x_nicknames in &vec_nicknames {

            // If user input is part of a characters full name or the full name itself
            // Then pass the full and correct charactet name to the new var 'character_arg_altered'
            if x_nicknames.character.to_lowercase().replace('-', "").contains(&character.to_lowercase()) ||
            x_nicknames.character.to_lowercase().contains(&character.to_lowercase()) {
                
                character_found = true;
                character_arg_altered = x_nicknames.character.to_owned();
                break;
            }
        }
    }

    // If user input isnt the full name, part of a full name or a nickname
    // Error out cause requested character was not found in the json
    if !character_found {
        let error_msg= &("Character `".to_owned() + &character + "` was not found!");
        ctx.say(error_msg).await?;
        println!("Error: {}", error_msg.replace('`', "'"));
        return Ok(());
    }

    // Reading the character json if found
    let char_file_path = "data/".to_owned() + &character_arg_altered + "/" + &character_arg_altered + ".json";
    let char_file_data = fs::read_to_string(char_file_path)
        .expect(&("\nFailed to read '".to_owned() + &character_arg_altered + ".json" + "' file."));
    
    //Deserializing from character json
    let moves_info = serde_json::from_str::<Vec<MoveInfo>>(&char_file_data).unwrap();            
    
    println!("Successfully read '{}.json' file.", &character_arg_altered);
    
    character_found = true;

    // Checking if aliases for this characters moves exist
    let aliases_path = "data/".to_owned() + &character_arg_altered + "/aliases.json";
    if Path::new(&aliases_path).exists() {
        
        // Reading the aliases json
        let aliases_data = fs::read_to_string(&aliases_path)
            .expect(&("\nFailed to read '".to_owned() + &aliases_path + "' file."));
        
        // Deserializing the aliases json
        let aliases_data = serde_json::from_str::<Vec<MoveAliases>>(&aliases_data).unwrap();

        'outer: for alias_data in aliases_data {
            for x_aliases in alias_data.aliases {
                
                // If the requested argument (character_move) is an alias for any of the moves listed in aliases.json
                // Change the given argument (character_move) to the actual move name instead of the alias
                if x_aliases.to_lowercase().trim().replace('.', "") 
                == character_move.to_lowercase().trim().replace('.', "") {
                    character_move = alias_data.input.to_string();
                    break 'outer;
                }
            }
        }
    }

    // Reading images.json for this character
    let image_links = fs::read_to_string("data/".to_owned() + &character_arg_altered + "/images.json")
        .expect(&("\nFailed to read 'data/".to_owned() + &character_arg_altered + "'/images.json' file."));

    // Deserializing images.json for this character
    let image_links= serde_json::from_str::<Vec<ImageLinks>>(&image_links).unwrap();
    
    // Default vaule never used
    let mut mframes = &moves_info[0];

    for moves in &moves_info {
        // Iterating through the moves of the json file to find the move requested
        // Specifically if user arg is exactly move input
        if moves.input.to_string().to_lowercase().replace('.', "") 
        == character_move.to_string().to_lowercase().replace('.', "") {
            mframes = &moves;
            move_found = true;
            break;
        }        
    }

    if !move_found {
        for moves in &moves_info {
            // Iterating through the moves of the json file to find the move requested
            // Specifically if user arg is contained in move name
            if moves.name.to_string().to_lowercase().contains(&character_move.to_string().to_lowercase()) {
                mframes = &moves;
                move_found = true;
                break;
            }
        }
    }

    if move_found {
            
        for img_links in image_links {
            // Iterating through the image.json to find the move's hitbox links
            if mframes.input == img_links.input {

                move_found = true;
                println!("Successfully read move '{}' in '{}.json' file.", &mframes.input.to_string(), &character_arg_altered);

                
                if !img_links.hitbox_img[0].is_empty() {

                    // Priting hitboxes in discord chat
                    let bot_msg = "__**Move: ".to_owned() + &img_links.input + "**__";
                    ctx.say(&bot_msg).await?;

                    for htbx_img in img_links.hitbox_img {                        
                        ctx.channel_id().say(ctx, &htbx_img).await?;
                    }
                }
                else{
                    // Priting hitboxes in discord chat
                    let bot_msg = "__**Move: ".to_owned() + &img_links.input + "**__";
                    ctx.say(&bot_msg).await?;
                    ctx.channel_id().say(ctx, HITBOX_DEFAULT).await?;
                }
                
            }
        }
    }
    // Error message cause given move wasnt found in the json
    else {
        let error_msg= &("Move `".to_owned() + &character_move + "` was not found!" + "\nYou can request, report broken stuff or leave feedback by executing the `/feedback` command.");
        ctx.say(error_msg).await?;
        // Console error print
        println!("{}", "Error: Move '".to_owned() + &character_move + "' was not found!");
    }

    Ok(())
}