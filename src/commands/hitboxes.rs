use colored::Colorize;
use std::{fs, string::String};
use poise::serenity_prelude::{CreateEmbed, CreateEmbedFooter};
use crate::{check, find, Context, Error, ImageLinks, MoveInfo, EMBED_COLOR, HITBOX_DEFAULT};

/// Display a move's hitbox images.
#[poise::command(prefix_command, slash_command)]
pub async fn hitboxes(
    ctx: Context<'_>,
    #[min_length = 2]
    #[description = "Character name or nickname."] character: String,
    #[min_length = 2]
    #[rename = "move"]
    #[description = "Move name, input or alias."] character_move: String,
) -> Result<(), Error> {

    println!("{}", ("Command Args: '".to_owned() + &character + ", " + &character_move + "'").purple());

    if (check::adaptive_check(ctx, true, false).await).is_err() {
        return Ok(());
    }

    // Finding character
    // This will store the full character name in case user input was an alias
    let character_arg_altered = match find::find_character(&character).await {
        Ok(character_arg_altered) => character_arg_altered,
        Err(err) => {
            ctx.say(err.to_string()).await?;
            return Ok(()) }
    };

    // Reading the character json
    let char_file_path = "data/".to_owned() + &character_arg_altered + "/" + &character_arg_altered + ".json";
    let char_file_data = fs::read_to_string(char_file_path)
        .expect(&("\nFailed to read '".to_owned() + &character_arg_altered + ".json" + "' file."));

    // Deserializing from character json
    let moves_info = serde_json::from_str::<Vec<MoveInfo>>(&char_file_data).unwrap();
           
    println!("{}", ("Successfully read '".to_owned() + &character_arg_altered + ".json' file.").green());

    // Finding move index
    let index = match find::find_move_index(&character_arg_altered, character_move, &moves_info).await {
        Ok(index) => index,
        Err(err) => {
            ctx.say(err.to_string() + "\nView the moves of a character by executing `/moves`.").await?;
            return Ok(()) }    
    };

    // Reading images.json for this character
    let image_links = fs::read_to_string("data/".to_owned() + &character_arg_altered + "/images.json")
        .expect(&("\nFailed to read 'data/".to_owned() + &character_arg_altered + "'/images.json' file."));

    // Deserializing images.json for this character
    let image_links = serde_json::from_str::<Vec<ImageLinks>>(&image_links).unwrap();

    let move_info = &moves_info[index];
    let mut vec_embeds = Vec::new();

    let embed_title = "__**".to_owned()
        + &character_arg_altered.replace("_", " ") + " "
        + &move_info.input + " / "
        + &move_info.name + "**__";

    let embed_url = "https://dustloop.com/w/GGST/".to_owned() + &character_arg_altered + "#Overview";

    for img_links in image_links {
        // Iterating through the image.json to find the move's hitbox links
        if move_info.input == img_links.input {

            println!("{}", ("Successfully read move '".to_owned() + &move_info.input.to_string() + "' in '" + &character_arg_altered + ".json' file.").green());

            // No hitbox image
            if img_links.hitboxes.is_empty() {

                let empty_embed = CreateEmbed::new()
                    .color(EMBED_COLOR)
                    .title(&embed_title)
                    .url(&embed_url)
                    .image(HITBOX_DEFAULT);

                vec_embeds.push(empty_embed);
            }
            // One hitbox image
            else if  img_links.hitboxes.len() == 1 {

                let embed = CreateEmbed::new()
                    .color(EMBED_COLOR)
                    .title(&embed_title)
                    .url(&embed_url)
                    .image(&img_links.hitboxes[0]);

                vec_embeds.push(embed);
            }
            // More than one hitbox image
            else {
                for htbx_img in &img_links.hitboxes {

                    let embed_footer = CreateEmbedFooter::new(
                        "Move has ".to_owned() + &img_links.hitboxes.len().to_string() + " hitbox images.");

                    let embed = CreateEmbed::new()
                        .color(EMBED_COLOR)
                        .title(&embed_title)
                        .url(&embed_url)
                        .image(htbx_img)
                        .footer(embed_footer);

                    vec_embeds.push(embed);
                }
            }
        }
    }

    let mut reply = poise::CreateReply::default();
    reply.embeds.extend(vec_embeds);

    ctx.send(reply).await?;

    Ok(())
}
