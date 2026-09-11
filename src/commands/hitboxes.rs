use std::string::String;
use poise::{CreateReply, serenity_prelude::{CreateEmbed, CreateEmbedFooter}};
use crate::{check, find, Context, Error, EMBED_COLOR, HITBOX_DEFAULT};

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

    if (check::adaptive_check(ctx, true, false).await).is_err() {
        return Ok(());
    }

    // Finding character
    // This will store the full character name in case user input was an alias
    let (character, char_id) = match find::find_character(&character, ctx.data().db.clone()).await {
        Ok(character) => character,
        Err(err) => {
            ctx.say(err.to_string()).await?;
            return Ok(()) }
    };

    // Finding move and move id
    let (move_data, move_id) = match find::find_move(char_id, &character_move, ctx.data().db.clone()).await {
        Ok(move_data) => move_data,
        Err(err) => {
            ctx.say(err.to_string() + "\nView the moves of a character by executing `/moves`.").await?;
            return Ok(()) }    
    };

    // Finding hitboxes
    let hitbox_data = (find::find_hitboxes(move_id, ctx.data().db.clone()).await).unwrap();

    let mut builder = CreateReply::new();

    let mut embed_title = "__**".to_owned()
        + &character.replace("_", " ") + " "
        + &move_data.input;

    // Check if the move has an actual name
    if move_data.input != move_data.name && !move_data.name.trim().is_empty() {
        embed_title += " / ";
        embed_title += &move_data.name;
    }

    embed_title += "**__";

    let embed_url = "https://dustloop.com/w/GGST/".to_owned() + &character.replace(" ", "_") + "#Overview";

    match hitbox_data.len() {
        // One hitbox image
        1 => {
            let embed = CreateEmbed::new()
                .color(EMBED_COLOR)
                .title(&embed_title)
                .url(&embed_url)
                .image(&hitbox_data[0].hitbox);

            builder = builder.embed(embed);
        },
        // More than one hitbox image
        2.. => {
            for x in 0..hitbox_data.len() {

                let embed_footer = CreateEmbedFooter::new(
                    "Move has ".to_owned() + &hitbox_data.len().to_string() + " hitbox images.");

                let embed = CreateEmbed::new()
                    .color(EMBED_COLOR)
                    .title(&embed_title)
                    .url(&embed_url)
                    .image(&hitbox_data[x].hitbox)
                    .footer(embed_footer);

                builder = builder.embed(embed);
            }
        },
        // No hitbox image
        ..1 => {
            let empty_embed = CreateEmbed::new()
                .color(EMBED_COLOR)
                .title(&embed_title)
                .url(&embed_url)
                .image(HITBOX_DEFAULT);

            builder = builder.embed(empty_embed);
        }
    };

    if !hitbox_data[0].hitbox_caption.trim().is_empty() {
        builder = builder.embed(CreateEmbed::new().color(EMBED_COLOR)
            .description(&hitbox_data[0].hitbox_caption));
    }

    ctx.send(builder).await?;
    Ok(())
}
