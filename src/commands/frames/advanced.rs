use std::string::String;
use poise::serenity_prelude::{CreateEmbed, CreateEmbedFooter};
use crate::{check, find, ran, Context, Error, EMBED_COLOR, IMAGE_DEFAULT};

/// Display a move's frame data in an advanced view.
#[poise::command(prefix_command, slash_command)]
pub async fn advanced(
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

    if (check::adaptive_check(ctx, true, true).await).is_err() {
        return Ok(());
    }

    let (character, char_id) = match find::find_character(&character).await {
        Ok(character) => character,
        Err(err) => {
            ctx.say(err.to_string()).await?;
            return Ok(()) }
    };

    // Finding move and move id
    let (move_data, _) = match find::find_move(char_id, &character_move).await {
        Ok(move_data) => move_data,
        Err(err) => {
            ctx.say(err.to_string() + "\nView the moves of a character by executing `/moves`.").await?;
            return Ok(()) }    
    };

    if !move_data.image.is_empty() {
        embed_image = move_data.image.to_string();
    }

    {
        // Parse guild id to string
        let guild_id = ctx.guild_id().unwrap().to_string();
        if !check::gid_exists(&guild_id).await {
            if let Some(image_path) = ran::ran_p().await {
                embed_image = image_path;
            }
        }
    }

    let mut vec_embeds = Vec::new();
    let embed_title = "__**".to_owned()
        + &character.replace('_', " ") + " "
        + &move_data.input + " / " + &move_data.name + "**__";

    let embed_url = "https://dustloop.com/w/GGST/".to_owned() + &character.replace(" ", "_") + "#Overview";
    let embed_footer = CreateEmbedFooter::new(&move_data.caption);

    let embed = CreateEmbed::new()
        .color(EMBED_COLOR)
        .title(&embed_title)
        .url(&embed_url)
        .image(&embed_image)
        .fields(vec![
            ("Damage", &move_data.damage.to_string(), true),
            ("Guard", &move_data.guard.to_string(), true),
            ("Invinciblity", &move_data.invincibility.to_string(), true),
            ("Startup", &move_data.startup.to_string(), true),
            ("Active", &move_data.active.to_string(), true),
            ("Recovery", &move_data.recovery.to_string(), true),
            ("On Hit", &move_data.on_hit.to_string(), true),
            ("On Block", &move_data.on_block.to_string(), true),
            ("Counter", &move_data.counter.to_string(), true),
            ("Level", &move_data.level.to_string(), true),
            ("Risc Gain", &move_data.risc_gain.to_string(), true),
            ("Risc Loss", &move_data.risc_loss.to_string(), true),
            ("Scaling", &move_data.scaling.to_string(), true),
            ("Wall Damage", &move_data.wall_damage.to_string(), true),
            ("Input Tension", &move_data.input_tension.to_string(), true),
            ("Chip Ratio", &move_data.chip_ratio.to_string(), true),
            ("OTG Ratio", &move_data.otg_ratio.to_string(), true),
            ("Cancel", &move_data.cancel.to_string(), true),
        ])
        .footer(embed_footer);

    vec_embeds.push(embed);

    if !&move_data.notes.is_empty() {
        let embed2 = CreateEmbed::new()
            .color(EMBED_COLOR)
            .description(&move_data.notes);

        vec_embeds.push(embed2);
    }

    let mut reply = poise::CreateReply::default();
    reply.embeds.extend(vec_embeds);
    ctx.send(reply).await?;

    // New version notification
    //ctx.channel_id().say(ctx, r"[__**Patch.**__](<https://github.com/yakiimoninja/baiken/releases>)").await?;

    Ok(())
}
