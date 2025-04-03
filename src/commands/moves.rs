mod utils;
use utils::{get_normal_moves, get_special_moves, get_super_moves};
use crate::{check, find::{self, find_all_character_moves}, Context, Error, EMBED_COLOR};
use crate::structs::{MoveAliases, MoveInfo};
use poise::serenity_prelude::{CreateEmbed, CreateEmbedFooter};
use colored::Colorize;
use std::{fs, string::String};

#[derive(Debug, poise::ChoiceParameter)]
pub enum TypeChoice{
    #[name = "all"]
    All,
    #[name = "normals"]
    Normals,
    #[name = "specials"]
    Specials,
    #[name = "supers"]
    Supers,
}

/// Display a character's moves, inputs and aliases.
#[poise::command(prefix_command, slash_command)]
pub async fn moves(
    ctx: Context<'_>,
    #[min_length = 2]
    #[description = "Character name or nickname."] character: String,
    #[rename = "type"]
    #[description = "Move type."] category: TypeChoice,
) -> Result<(), Error> {

    if (check::adaptive_check(ctx, true, false).await).is_err() {
        return Ok(());
    }

    // Finding character
    let (character, _) = match find::find_character(&character, ctx.data().db.clone()).await {
        Ok(character) => character,
        Err(err) => {
            ctx.say(err.to_string()).await?;
            return Ok(());
        }
    };

    // Reading the character json
    let char_file_path =
        "data/".to_owned() + &character + "/" + &character + ".json";
    let char_file_data = fs::read_to_string(char_file_path)
        .expect(&("\nFailed to read '".to_owned() + &character + ".json" + "' file."));

    // Deserializing from character json
    let moves_info = serde_json::from_str::<Vec<MoveInfo>>(&char_file_data).unwrap();

    println!("{}", ("Successfully read '".to_owned() + &character + ".json' file.").green());

    // Reading the aliases json
    let aliases_path = "data/".to_owned() + &character + "/aliases.json";
    let aliases_data = fs::read_to_string(&aliases_path)
        .expect(&("\nFailed to read '".to_owned() + &aliases_path + "' file."));

    // Deserializing the aliases json
    let aliases_data = serde_json::from_str::<Vec<MoveAliases>>(&aliases_data).unwrap();

    let mut vec_embeds = Vec::new();

    let embed_title = "__**".to_owned() + &character.replace('_', " ") + " Moves / Aliases**__";
    let embed_url = "https://dustloop.com/w/GGST/".to_owned() + &character.replace(" ", "_") + "#Overview";
    let embed_footer = CreateEmbedFooter::new(
        "Try the \"/help notes\" command for usage notes and specifics.\nOr \"/report\" to request a new aliases.");
    
    match category {
        TypeChoice::All => {
            let normal_moves = get_normal_moves(&moves_info, &aliases_data).await;
            let special_moves = get_special_moves(&moves_info, &aliases_data).await;
            let super_moves = get_super_moves(&moves_info, &aliases_data).await;

            let normals_embed = CreateEmbed::new()
                .color(EMBED_COLOR)
                .title(embed_title)
                .url(embed_url)
                .description(normal_moves);

            let specials_embed = CreateEmbed::new()
                .color(EMBED_COLOR)
                .description(special_moves);

            let supers_embed = CreateEmbed::new()
                .color(EMBED_COLOR)
                .description(super_moves)
                .footer(embed_footer);
            
            vec_embeds.push(normals_embed);
            vec_embeds.push(specials_embed);
            vec_embeds.push(supers_embed);
        },
        TypeChoice::Normals => {
            let normal_moves = get_normal_moves(&moves_info, &aliases_data).await;
            
            let normals_embed = CreateEmbed::new()
                .color(EMBED_COLOR)
                .title(embed_title)
                .url(embed_url)
                .description(normal_moves)
                .footer(embed_footer);
            
            vec_embeds.push(normals_embed);
        },
        TypeChoice::Specials => {
            let special_moves = get_special_moves(&moves_info, &aliases_data).await;
            
            let specials_embed = CreateEmbed::new()
                .color(EMBED_COLOR)
                .title(embed_title)
                .url(embed_url)
                .description(special_moves)
                .footer(embed_footer);
            
            vec_embeds.push(specials_embed);
        },
        TypeChoice::Supers => {
            let super_moves = get_super_moves(&moves_info, &aliases_data).await;
            
            let supers_embed = CreateEmbed::new()
                .color(EMBED_COLOR)
                .title(embed_title)
                .url(embed_url)
                .description(super_moves)
                .footer(embed_footer);
            
            vec_embeds.push(supers_embed);
        },
    };

    let mut reply = poise::CreateReply::default();
    reply.embeds.extend(vec_embeds);

    ctx.send(reply).await?;

    Ok(())
}
