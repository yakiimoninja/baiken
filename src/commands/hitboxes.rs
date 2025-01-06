use std::string::String;
use poise::serenity_prelude::{CreateEmbed, CreateEmbedFooter};
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
    let (character, char_id) = match find::find_character(&character).await {
        Ok(character) => character,
        Err(err) => {
            ctx.say(err.to_string()).await?;
            return Ok(()) }
    };

    // Finding move and move id
    let (move_data, move_id) = match find::find_move(char_id, &character_move).await {
        Ok(move_data) => move_data,
        Err(err) => {
            ctx.say(err.to_string() + "\nView the moves of a character by executing `/moves`.").await?;
            return Ok(()) }    
    };

    // Finding hitboxes
    let hitbox_data = (find::find_hitboxes(move_id).await).unwrap();

    let mut vec_embeds = Vec::new();

    let embed_title = "__**".to_owned()
        + &character.replace("_", " ") + " "
        + &move_data.input + " / "
        + &move_data.name + "**__";

    let embed_url = "https://dustloop.com/w/GGST/".to_owned() + &character.replace(" ", "_") + "#Overview";

    match hitbox_data.len() {
        // One hitbox image
        1 => {
            let embed = CreateEmbed::new()
                .color(EMBED_COLOR)
                .title(&embed_title)
                .url(&embed_url)
                .image(&hitbox_data[0].hitbox);

            vec_embeds.push(embed);
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

                vec_embeds.push(embed);
            }
        },
        // No hitbox image
        ..1 => {
            let empty_embed = CreateEmbed::new()
                .color(EMBED_COLOR)
                .title(&embed_title)
                .url(&embed_url)
                .image(HITBOX_DEFAULT);

            vec_embeds.push(empty_embed);
        }
    };

    let mut reply = poise::CreateReply::default();
    reply.embeds.extend(vec_embeds);

    ctx.send(reply).await?;

    Ok(())
}
