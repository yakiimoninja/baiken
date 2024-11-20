use colored::Colorize;
use std::{fs, string::String};
use crate::{
    HITBOX_DEFAULT,
    MoveInfo,
    ImageLinks,
    Context,
    Error,
    check,
    find,
};

/// Display a move's hitbox images.
#[allow(unused_assignments)]
#[poise::command(prefix_command, slash_command)]
pub async fn hitboxes(
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
            .expect(&("\nFailed to read '".to_owned() + &character_arg_altered + ".json" + "' file."));

    // Deserializing from character json
    let moves_info = serde_json::from_str::<Vec<MoveInfo>>(&char_file_data).unwrap();
           
    println!("{}", ("Successfully read '".to_owned() + &character_arg_altered + ".json' file.").green());

    // Finding move struct index 
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

            if !img_links.hitbox_img[0].is_empty() {

                // Printing hitboxes in discord chat
                ctx.say(&bot_msg).await?;

                for htbx_img in img_links.hitbox_img {                        
                    ctx.channel_id().say(ctx, &htbx_img).await?;
                }
            }
            else{
                // Printing hitboxes in discord chat
                ctx.say(&bot_msg).await?;
                ctx.channel_id().say(ctx, HITBOX_DEFAULT).await?;
            }
            
        }
    }

    Ok(())
}
