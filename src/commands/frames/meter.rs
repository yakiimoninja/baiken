use std::string::String;
use poise::serenity_prelude::CreateEmbed;
use crate::{check, find, Context, Error, EMBED_COLOR, IMAGE_DEFAULT};
use crate::structs::MoveInfo;

const GREEN_CIRCLE: &str = "🟢\u{200b}";
const RED_SQUARE: &str = "🟥\u{200b}";
const BLUE_DIAMOND: &str = "🔷\u{200b}";

// Returns a String of symbols representing startup frames
async fn startup_frames(move_data: &MoveInfo) -> String {
    let startup_vec = sep_frame_vec(&move_data.startup).await;
    let mut meter_msg = String::new();
    //println!("startup_vec: {:?}", startup_vec);
    
    // If vec has only one entry and the entry is empty or -
    // If vec has only one entry and the move has only 1 frame of startup
    if (startup_vec.len() == 1 && startup_vec[0] == "-") ||
    (startup_vec.len() == 1 && startup_vec[0].parse::<u16>().unwrap() == 1) {
        meter_msg += "-";
    }
    // Otherwise execute logic
    else{

        // This bool to determine if bracket was present
        let mut startup_bra = false;
        
        // Making the message
        for x in 0..startup_vec.len() {

            // If vec string entry is a digit
            if let Ok(num) = startup_vec[x].parse::<u16>() {

                // Iterate up to its numerical value -1
                for _ in 0..num-1 {

                    // If left bracket was not passed previously
                    if !startup_bra {
                        // Put a GREEN_CIRCLE into the message
                        meter_msg += GREEN_CIRCLE;
                    }
                    // If left bracket was passed
                    else {

                        // The difference between the first possible frame a move can connect
                        // and the latest frame -1 is the times a GREEN_CIRCLE is going to be 
                        // put inside the msg and inside brackets
                        for _ in 0..( (startup_vec[x].parse::<u16>().unwrap()) - (startup_vec[x-2].parse::<u16>()).unwrap()) {
                            meter_msg += GREEN_CIRCLE;
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
                    if let Ok(num) = startup_vec[x+1].parse::<u16>() {

                        // If value is 1 then print GREEN_CIRCLE instead of "+" 
                        if num == 1 {
                            meter_msg += GREEN_CIRCLE;
                        }
                        // Otherwise put GREEN_CIRCLE and  "+" sign
                        else{
                            meter_msg = meter_msg + GREEN_CIRCLE + &startup_vec[x];
                        }
                    }
                    // If entry after + isnt a number ????
                    else{
                        meter_msg = meter_msg + &startup_vec[x];
                    }
                }
                // Otherwise display the symbol
                else {
                    //println!("Indide else: {:?}", startup_vec);
                    meter_msg = meter_msg + &startup_vec[x];
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
    meter_msg
}


// Returns a String of symbols representing active frames
async fn active_frames(move_data: &MoveInfo) -> String {
    // Processing for active frames
    let active_vec = sep_frame_vec(&move_data.active).await;
    let mut meter_msg = String::new();
    //println!("Active vec: {:?}", active_vec);
    
    if active_vec.len() == 1 && active_vec[0] == "-" {
        meter_msg += "-";
    }
    else {
        let mut hit_recovery = false;

        for active_vec_string in &active_vec {

            // If vec string entry is a digit
            if let Ok(num) = active_vec_string.parse::<u16>() {

                // Iterate up to its numerical value
                for _ in 0..num {

                    // If left parenthesis was not passed when iterating
                    if !hit_recovery {
                        meter_msg += RED_SQUARE;
                    }
                    // If left parenthesis was passed when iterating
                    else {
                        meter_msg += BLUE_DIAMOND;
                    }
                }
            }
            // If vec string entry isnt a digit
            else {
                meter_msg = meter_msg + active_vec_string;

                if active_vec_string == "(" {
                    hit_recovery = true;
                }
                else if active_vec_string == ")" {
                    hit_recovery = false;
                }
            }
        }
    }
    meter_msg
}


// Returns a String of symbols representing recovery frames
async fn recovery_frames(move_data: &MoveInfo) -> String {
    // Processing for recovery frames
    let recovery_vec = sep_frame_vec(&move_data.recovery).await;
    let mut meter_msg = String::new();
    
    if recovery_vec.len() == 1 && recovery_vec[0] == "-" {
        meter_msg += "-";
    }
    else {

        let mut recovery_tilde = false;

        for x in 0..recovery_vec.len() {

            // If vec string entry is a digit
            if let Ok(num) = recovery_vec[x].parse::<u16>() {

                // Iterate up to its numerical value
                for _ in 0..num {

                    // If tilde was not passed previously
                    if !recovery_tilde {
                        // Put a BLUE_DIAMOND into the message
                        meter_msg += BLUE_DIAMOND;
                    }
                    else {
                        // If tilde was passed
                        // The difference between the first possible frame a move can connect
                        // and the latest frame -1 is the times a BLUE_DIAMOND is going to be 
                        // put inside the msg
                        for _ in 0..((recovery_vec[x].parse::<u16>().unwrap()) - (recovery_vec[x-2].parse::<u16>()).unwrap()) {
                            meter_msg += BLUE_DIAMOND;
                        }
                        break;
                    }
                }
            }
            // If vec string entry isnt a digit
            else {
                meter_msg = meter_msg + &recovery_vec[x];
                // Execute same logic for ( and ~
                recovery_tilde = recovery_vec[x] == "~" || recovery_vec[x] == "(";
            }
        }
    }
    meter_msg
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


/// Display visually, a move's frame meter.
#[poise::command(prefix_command, slash_command)]
pub async fn meter(
    ctx: Context<'_>,
    #[min_length = 2]
    #[description = "Character name or nickname."] character: String,
    #[min_length = 2]
    #[rename = "move"]
    #[description = "Move name, input or alias."] character_move: String,
) -> Result<(), Error> {

    // Initializing variables for the embed
    // They must not be empty cause then the embed wont be sent
    let mut embed_image = IMAGE_DEFAULT.to_string();

    if (check::adaptive_check(ctx, true, false).await).is_err() {
        return Ok(());
    }

    let (character, char_id) = match find::find_character(&character, ctx.data().db.clone()).await {
        Ok(character) => character,
        Err(err) => {
            ctx.say(err.to_string()).await?;
            return Ok(()) }
    };

    // Finding move and move id
    let (move_data, _) = match find::find_move(char_id, &character_move, ctx.data().db.clone()).await {
        Ok(move_data) => move_data,
        Err(err) => {
            ctx.say(err.to_string() + "\nView the moves of a character by executing `/moves`.").await?;
            return Ok(()) }
    };

    if !move_data.image.is_empty() {
        embed_image = move_data.image.to_string();
    }

    // Processing for startup frames
    let mut meter_msg = String::from("`");
    //let mut meter_msg = String::new();
    meter_msg += &startup_frames(&move_data).await;
    meter_msg += &active_frames(&move_data).await;
    meter_msg += &recovery_frames(&move_data).await;
    meter_msg += "`";

    let mut embed_title = "__**".to_owned()
        + &character.replace('_', " ") + " "
        + &move_data.input + " / " + &move_data.name + "**__";

    if move_data.input == move_data.name {
        embed_title = "__**".to_owned()
            + &character.replace('_', " ") + " "
            + &move_data.input + "**__";
    }

    let embed_url = "https://dustloop.com/w/GGST/".to_owned() + &character.replace(" ", "_") + "#Overview";

    let embed = CreateEmbed::new()
        .color(EMBED_COLOR)
        .title(embed_title)
        .url(embed_url)
        .fields(vec![
            ("Startup", &move_data.startup.to_string(), true),
            ("Active", &move_data.active.to_string(), true),
            ("Recovery", &move_data.recovery.to_string(), true)])
        .image(embed_image);

    let embed2 = CreateEmbed::new()
        .color(EMBED_COLOR)
        .description(&meter_msg);

    let vec_embeds = vec![embed, embed2];
    let mut reply = poise::CreateReply::default();
    reply.embeds.extend(vec_embeds);
    ctx.send(reply).await?;

    Ok(())
}
