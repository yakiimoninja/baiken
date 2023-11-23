use std::fs;
use std::path::Path;
use std::string::String;
use crate::{Context, Error, IMAGE_DEFAULT};
use crate::serenity::futures::{Stream, StreamExt, self};
use crate::{MoveInfo, MoveAliases, ImageLinks, Nicknames, CHARS, check};

const GREEN_CIRCLE: &str = "🟢";
const RED_SQUARE: &str = "🟥";
const BLUE_DIAMOND: &str = "🔷";

// Autocompletes the character name
async fn autocomplete_character<'a>(
    _ctx: Context<'_>,
    partial: &'a str,
) -> impl Stream<Item = String> + 'a {
    futures::stream::iter(&CHARS)
        .filter(move |name| futures::future::ready(name.to_lowercase().contains(&partial.to_lowercase())))
        .map(|name| name.to_string())
}

/// Displays the frame meter of a move.
#[allow(unused_assignments)]
#[poise::command(prefix_command, slash_command, aliases("fm"))]
pub async fn fmeter(
    ctx: Context<'_>,
    #[description = "Character name or nickname."]
    #[autocomplete = "autocomplete_character"] character: String,
    #[description = "Move name, input or alias."] mut character_move: String,
) -> Result<(), Error> {
    // This will store the full character name in case user input was an alias
    let mut character_arg_altered = String::new();
    // Flags that will be used for logic to determine output
    let mut character_found = false;
    let mut move_found = false;

    if let Err(_) = check::adaptive_check(
        ctx,
        (true, &character),
        (true, &character_move),
        true,
        true,
        true,
        true,
        true).await {
        
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
        println!("\nError: {}", error_msg.replace('`', "'"));
        return Ok(())
    }

    // Reading the character json
    let char_file_path = "data/".to_owned() + &character_arg_altered + "/" + &character_arg_altered + ".json";
    let char_file_data = fs::read_to_string(char_file_path)
        .expect(&("\nFailed to read '".to_owned() + &character + ".json" + "' file."));
    
    // Deserializing from character json
    let moves_info = serde_json::from_str::<Vec<MoveInfo>>(&char_file_data).unwrap();            
    
    println!("\nCommand: '{} {} {}'", ctx.command().qualified_name, character, character_move);
    println!("Successfully read '{}.json' file.", character_arg_altered);
    

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
    let image_links = serde_json::from_str::<Vec<ImageLinks>>(&image_links).unwrap();

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
        // Send move image
        for img_links in image_links {
            // Iterating through the image.json to find the move's hitbox links
            if mframes.input == img_links.input {

                println!("Successfully read move '{}' in '{}.json' file.", &mframes.input.to_string(), &character_arg_altered);

                if !img_links.move_img.is_empty() {

                    // Printing image in discord chat
                    let bot_msg = "__**Move: ".to_owned() + &img_links.input + "**__";
                    ctx.say(&bot_msg).await?;
                    ctx.channel_id().say(ctx, &img_links.move_img).await?;
                }
                else{
                    // Printing default fallback image in discord chat
                    let bot_msg = "__**Move: ".to_owned() + &img_links.input + "**__";
                    ctx.say(&bot_msg).await?;
                    ctx.channel_id().say(ctx, IMAGE_DEFAULT).await?;
                }
                
            }
        }
        
        let mut frame_meter_msg = r#"__Startup__: "#.to_owned() + &mframes.startup + " → `";

        // Processing for startup frames

        let startup_vec = sep_frame_vec(&mframes.startup).await;
        //println!("startup_vec: {:?}", startup_vec);
        
        // If vec has only one entry and the entry is empty or -
        // If vec has only one entry and the move has only 1 frame of startup
        if (startup_vec.len() == 1 && startup_vec[0] == "-") ||
        (startup_vec.len() == 1 && startup_vec[0].parse::<i8>().unwrap() == 1) {
            frame_meter_msg += "-";
        }
        // Otherwise execute logic
        else{

            // This bool to determine if bracket was present
            let mut startup_bra = false;
            
            // Making the message
            for x in 0..startup_vec.len() {

                // If vec string entry is a digit
                if let Ok(num) = startup_vec[x].parse::<i8>() {

                    // Iterate up to its numerical value -1
                    for _ in 0..num-1 {

                        // If left bracket was not passed previously
                        if !startup_bra {
                            // Put a GREEN_CIRCLE into the message
                            frame_meter_msg += GREEN_CIRCLE;
                        }
                        // If left bracket was passed
                        else {

                            // The difference between the first possible frame a move can connect
                            // and the latest frame -1 is the times a GREEN_CIRCLE is going to be 
                            // put inside the msg and inside brackets
                            for _ in 0..( (startup_vec[x].parse::<i8>().unwrap()) - (startup_vec[x-2].parse::<i8>()).unwrap()) {
                                frame_meter_msg += GREEN_CIRCLE;
                            }
                            break;
                        }
                    }
                }
                // If vec string entry isnt a digit
                else {
                    // Display a GREEN_CIRCLE if "+" is the last frame of the move
                    if x == startup_vec.len()-2 && startup_vec[x] == "+" {

                        // If entry after + is a digit assert its value
                        if let Ok(num) = startup_vec[x+1].parse::<i8>() {

                            // If value is 1 then print GREEN_CIRCLE instead of "+" 
                            if num == 1 {
                                frame_meter_msg += GREEN_CIRCLE;
                            }
                            // Otherwise put GREEN_CIRCLE and  "+" sign
                            else{
                                frame_meter_msg = frame_meter_msg + GREEN_CIRCLE + &startup_vec[x];
                            }
                        }
                        // If entry after + isnt a number ????
                        else{
                            frame_meter_msg = frame_meter_msg + &startup_vec[x];
                        }
                    }
                    // Otherwise display the symbol
                    else {
                        //println!("Indide else: {:?}", startup_vec);
                        frame_meter_msg = frame_meter_msg + &startup_vec[x];
                    }

                    // Execute same logic for [ and ~
                    if startup_vec[x] == "[" || startup_vec[x] == "~" {
                        startup_bra = true;
                    }
                    else if startup_vec[x] == "]" {
                        startup_bra = false;
                    }
                }
            }
        }

        frame_meter_msg = frame_meter_msg + "`\n__Active__: " + &mframes.active + " → `";
        
        // Processing for active frames
        let active_vec = sep_frame_vec(&mframes.active).await;
        //println!("Active vec: {:?}", active_vec);
        
        if active_vec.len() == 1 && active_vec[0] == "-" {
            frame_meter_msg += "-";
        }
        else {

            let mut hit_recovery = false;
            
            // Making the message
            for active_vec_string in &active_vec {

                // If vec string entry is a digit
                if let Ok(num) = active_vec_string.parse::<i8>() {

                    // Iterate up to its numerical value
                    for _ in 0..num {

                        // If left parenthesis was not passed previously
                        if !hit_recovery {
                            frame_meter_msg += RED_SQUARE;
                        }
                        // If left parenthesis was passed
                        else {
                            frame_meter_msg += BLUE_DIAMOND;
                        }
                    }
                }
                // If vec string entry isnt a digit
                else {
                    frame_meter_msg = frame_meter_msg + &active_vec_string;

                    if active_vec_string == "(" {
                        hit_recovery = true;
                    }
                    else if active_vec_string == ")" {
                        hit_recovery = false;
                    }
                }
            }
        }

        frame_meter_msg = frame_meter_msg + "`\n__Recovery__: " + &mframes.recovery + " → `";

        // Processing for recovery frames
        //println!("Original recovery: {:?}", mframes.recovery);
        let recovery_vec = sep_frame_vec(&mframes.recovery).await;
        
        if recovery_vec.len() == 1 && recovery_vec[0] == "-" {
            frame_meter_msg += "-";
        }
        else {

            let mut recovery_tilde = false;

            // Making the message
            for x in 0..recovery_vec.len() {

                // If vec string entry is a digit
                if let Ok(num) = recovery_vec[x].parse::<i8>() {

                    // Iterate up to its numerical value
                    for _ in 0..num {

                        // If tilde was not passed previously
                        if !recovery_tilde {
                            // Put a BLUE_DIAMOND into the message
                            frame_meter_msg += BLUE_DIAMOND;
                        }
                        // If tilde was passed
                        else {
                            
                            // The difference between the first possible frame a move can connect
                            // and the latest frame -1 is the times a BLUE_DIAMOND is going to be 
                            // put inside the msg
                            for _ in 0..( (recovery_vec[x].parse::<i8>().unwrap()) - (recovery_vec[x-2].parse::<i8>()).unwrap()) {
                                frame_meter_msg += BLUE_DIAMOND;
                            }
                            break;
                        }
                    }
                }
                // If vec string entry isnt a digit
                else {
                    frame_meter_msg = frame_meter_msg + &recovery_vec[x];

                    // Execute same logic for ( and ~
                    recovery_tilde = recovery_vec[x] == "~" || recovery_vec[x] == "(";
                }
            }
        }

        frame_meter_msg += "`";
        ctx.channel_id().say(ctx, frame_meter_msg).await?;

        // println!("Startup: {:?}", startup_vec);
        // println!("Active: {:?}", active_vec);
        // println!("Recovery: {:?}", recovery_vec);
    }
    // Error message cause given move wasnt found in the json
    else {
        let error_msg= &("Move `".to_owned() + &character_move + "` was not found!" + "\nView moves of a character by executing `/moves`.");
        ctx.say(error_msg).await?;
        // Console error print 
        println!("{}", "\nError: Move '".to_owned() + &character_move + "' was not found!");
    }

    Ok(())
}

/// Splits the string into a vec keeping the separators
async fn sep_frame_vec(text: &String) -> Vec<String> {

    // Remove whitespace
    let mut result = Vec::new();
    let mut last = 0;

    // Split to vector keeping the SEPERATORS
    for (index, matched) in text.match_indices(|c: char| !(c.is_alphanumeric())) {
        if last != index {
            result.push(text[last..index].to_string());
        }
        result.push(matched.to_string());
        last = index + matched.len();
    }
    if last < text.len() {
        result.push(text[last..].to_string());
    }

    // Removes empty entries and "total"
    if result.len() > 1 {

        'outer: loop {

            let cur_it_len = result.len();

            //println!("Before loop: {:?}, cur_it_len {}", result, result.len());
            for r in 0..result.len()-1 {

            //println!("In loop: {:?}, index {}, len {}", result, r, result.len());
                if result[r].to_lowercase() == "total" || result[r].is_empty() || result[r] == " " {
                    //println!("Index: {}, Removing total empty space. {:?}, len {}", r, result, result.len());
                    result.remove(r);
                    break;
                }
            }

            if cur_it_len == result.len(){
                break 'outer;
            }
        }
    }

    result
}
