use std::string::String;
use poise::serenity_prelude::CreateEmbed;
use crate::{check, find, Context, Error, EMBED_COLOR};

/// Display a move's frame data in a simplified view, with no image.
#[poise::command(prefix_command, slash_command)]
pub async fn tiny(
    ctx: Context<'_>,
    #[min_length = 2]
    #[description = "Character name or nickname."] character: String,
    #[min_length = 2]
    #[rename = "move"]
    #[description = "Move name, input or alias."] character_move: String,
) -> Result<(), Error> {

    if (check::adaptive_check(ctx, true, true).await).is_err() {
        return Ok(());
    }

    // Finding character
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
    
    // Sending the data as an embed
    let embed = CreateEmbed::new()
        .color(EMBED_COLOR)
        .title(&embed_title)
        .url(&embed_url)
        .fields(vec![
            ("Damage", &move_data.damage.to_string(), true),
            ("Guard", &move_data.guard.to_string(), true),
            ("Invinciblity", &move_data.invincibility.to_string(), true),
            ("Startup", &move_data.startup.to_string(), true),
            ("Active", &move_data.active.to_string(), true),
            ("Recovery", &move_data.recovery.to_string(), true),
            ("On Hit", &move_data.on_hit.to_string(), true),
            ("On Block", &move_data.on_block.to_string(), true),
            ("Counter", &move_data.counter.to_string(), true)
        ]);
        
    ctx.send(poise::CreateReply::default().embed(embed)).await?;
    
    Ok(())
}
