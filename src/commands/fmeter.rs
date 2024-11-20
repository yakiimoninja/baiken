use std::{fs, string::String};
use colored::Colorize;
use crate::{
    IMAGE_DEFAULT,
    MoveInfo,
    ImageLinks,
    Context,
    Error,
    check,
    find,
};

const GREEN_CIRCLE: &str = "🟢";
const RED_SQUARE: &str = "🟥";
const BLUE_DIAMOND: &str = "🔷";

/// Display a move's frame meter.
#[allow(unused_assignments)]
#[poise::command(prefix_command, slash_command)]
pub async fn fmeter(
    ctx: Context<'_>,
    #[min_length = 2]
    #[description = "Character name or nickname."] character: String,
    #[min_length = 2]
    #[rename = "move"]
    #[description = "Move name, input or alias."] mut character_move: String,
) -> Result<(), Error> {

    println!("{}", ("Command Args: '".to_owned() + &character + ", " + &character_move + "'").purple());

    // This will store the full character name in case user input was an alias
    let mut character_arg_altered = String::new();

    if (check::adaptive_check(
        ctx,
        true,
        true,
        true,
        true,
        true,
        false,
        false).await).is_err() {
        
        return Ok(());
    }

    // Finding character
    character_arg_altered = match find::find_character(&character).await {
        Ok(character_arg_altered) => character_arg_altered,
        Err(err) => {
            ctx.say(err.to_string()).await?;
            println!("{}", ("Error: ".to_owned() + &err.to_string()).red());
            return Ok(()) }
    };

    // Reading the character json
    let char_file_path = "data/".to_owned() + &character_arg_altered + "/" + &character_arg_altered + ".json";
    let char_file_data = fs::read_to_string(char_file_path)
        .expect(&("\nFailed to read '".to_owned() + &character + ".json" + "' file."));
    
    // Deserializing from character json
    let moves_info = serde_json::from_str::<Vec<MoveInfo>>(&char_file_data).unwrap();            
    
    println!("{}", ("Successfully read '".to_owned() + &character_arg_altered + ".json' file.").green());
    
    // Finding move index and input
    let index_and_move = match find::find_index_and_move(&character_arg_altered, character_move, &moves_info).await {
        Ok(index_and_input) => index_and_input,
        Err(err) => {
            ctx.say(err.to_string() + "\nView the moves of a character by executing `/moves`.").await?;
            println!("{}", ("Error: ".to_owned() + &err.to_string()).red());
            return Ok(()) }
    };

    // TODO find a fix for this
    character_move = index_and_move.1;

    // Reading images.json for this character
    let image_links = fs::read_to_string("data/".to_owned() + &character_arg_altered + "/images.json")
        .expect(&("\nFailed to read 'data/".to_owned() + &character_arg_altered + "'/images.json' file."));

    // Deserializing images.json for this character
    let image_links= serde_json::from_str::<Vec<ImageLinks>>(&image_links).unwrap();

    let move_info = &moves_info[index_and_move.0];

    // Send move image
    for img_links in image_links {
        // Iterating through the image.json to find the move's hitbox links
        if move_info.input == img_links.input {

            println!("{}", ("Successfully read move '".to_owned() + &move_info.input.to_string() + "' in '" + &character_arg_altered + ".json' file.").green());
            
            // Masked dustloop link
            let bot_msg = "## **[__".to_owned()
            + &character_arg_altered.replace('_', " ") + " "
            + &img_links.input
                .replace("]P[","|P|")
                .replace("]K[","|K|")
                .replace("]S[","|S|")
                .replace("]H[","|H|")
            + "__](<https://dustloop.com/wiki/index.php?title=GGST/" 
            + &character_arg_altered + "#Overview>)**";

            if !img_links.move_img.is_empty() {

                // Printing image in discord chat
                ctx.say(&bot_msg).await?;
                ctx.channel_id().say(ctx, &img_links.move_img).await?;
            }
            else{
                // Printing default fallback image in discord chat
                ctx.say(&bot_msg).await?;
                ctx.channel_id().say(ctx, IMAGE_DEFAULT).await?;
            }
            
        }
    }
    
    let mut frame_meter_msg = r#"- **Startup**: "#.to_owned() + &move_info.startup + " → `";

    // Processing for startup frames

    let startup_vec = sep_frame_vec(&move_info.startup).await;
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

    frame_meter_msg = frame_meter_msg + "`\n- **Active**: " + &move_info.active + " → `";
    
    // Processing for active frames
    let active_vec = sep_frame_vec(&move_info.active).await;
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
                frame_meter_msg = frame_meter_msg + active_vec_string;

                if active_vec_string == "(" {
                    hit_recovery = true;
                }
                else if active_vec_string == ")" {
                    hit_recovery = false;
                }
            }
        }
    }

    frame_meter_msg = frame_meter_msg + "`\n- **Recovery**: " + &move_info.recovery + " → `";

    // Processing for recovery frames
    //println!("Original recovery: {:?}", move_info.recovery);
    let recovery_vec = sep_frame_vec(&move_info.recovery).await;
    
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

    Ok(())
}

/// Splits the string into a vec keeping the separators
async fn sep_frame_vec(text: &str) -> Vec<String> {

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
