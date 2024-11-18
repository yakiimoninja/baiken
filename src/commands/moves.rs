use crate::{check, find, Context, Error, MoveAliases, MoveInfo};
use colored::Colorize;
use std::{fs, string::String};

/// Display a character's moves, inputs and aliases.
#[poise::command(prefix_command, slash_command)]
pub async fn moves(
    ctx: Context<'_>,
    #[min_length = 2]
    #[description = "Character name or nickname."] character: String,
) -> Result<(), Error> {

    println!("{}", ("Command Args: '".to_owned() + &character + "'").purple());

    if (check::adaptive_check(
        ctx,
        true,
        true,
        true,
        true,
        false,
        false,
        false,).await).is_err() {
        return Ok(());
    }

    // Finding character
    let character_arg_altered = match find::find_character(&character).await {
        Ok(character_arg_altered) => character_arg_altered,
        Err(err) => {
            ctx.say(err.to_string()).await?;
            println!("{}", ("Error: ".to_owned() + &err.to_string()).red());
            return Ok(());
        }
    };

    // Reading the character json
    let char_file_path =
        "data/".to_owned() + &character_arg_altered + "/" + &character_arg_altered + ".json";
    let char_file_data = fs::read_to_string(char_file_path)
        .expect(&("\nFailed to read '".to_owned() + &character + ".json" + "' file."));

    // Deserializing from character json
    let moves_info = serde_json::from_str::<Vec<MoveInfo>>(&char_file_data).unwrap();

    println!("{}", ("Successfully read '".to_owned() + &character_arg_altered + ".json' file.").green());

    // Reading the aliases json
    let aliases_path = "data/".to_owned() + &character_arg_altered + "/aliases.json";
    let aliases_data = fs::read_to_string(&aliases_path)
        .expect(&("\nFailed to read '".to_owned() + &aliases_path + "' file."));

    // Deserializing the aliases json
    let aliases_data = serde_json::from_str::<Vec<MoveAliases>>(&aliases_data).unwrap();

    let mut normal_moves = String::new();
    let mut move_index: usize = 0;
    // Message split due to discord character limit
    for (x, moves) in moves_info.iter().enumerate() {

        // Stopping loop and spiltting message if move isnt a normal
        if moves.move_type.to_lowercase() != "normal" {
            move_index = x;
            break;
        }

        normal_moves =
            normal_moves.to_owned() + "\n- **" + &moves.input + " / " + &moves.name + "**";

        for moves_aliases in aliases_data.iter() {
            // If move input exists in aliases.json
            if moves.input == moves_aliases.input  {
                normal_moves += "\n\tAliases → `";

                // Format message if there is only one alias or multiple
                for a in 0..moves_aliases.aliases.len() {
                    if a != moves_aliases.aliases.len() - 1 {
                        normal_moves = normal_moves.to_owned() + &moves_aliases.aliases[a] + "`, `";
                    }
                    else {
                        normal_moves = normal_moves.to_owned() + &moves_aliases.aliases[a];
                    }
                }
                normal_moves = normal_moves.to_owned() + "`\n";
            }
            else {
                continue;
            }
        }
    }

    let mut special_moves = String::new();
    for (x, moves) in moves_info.iter().enumerate().skip(move_index) {
        // Stopping loop and spiltting message if move isnt a normal
        if moves.move_type.to_lowercase() != "other" && moves.move_type.to_lowercase() != "special" {
            move_index = x;
            break;
        }
        special_moves =
            special_moves.to_owned() + "\n- **" + &moves.input + " / " + &moves.name + "**";

        for moves_aliases in aliases_data.iter() {
            // If move input exists in aliases.json
            if moves.input == moves_aliases.input {
                special_moves += "\n\tAliases → `";

                // Format message if there is only one alias or multiple
                for a in 0..moves_aliases.aliases.len() {
                    if a != moves_aliases.aliases.len() - 1 {
                        special_moves =
                            special_moves.to_owned() + &moves_aliases.aliases[a] + "`, `";
                    }
                    else {
                        special_moves = special_moves.to_owned() + &moves_aliases.aliases[a];
                    }
                }
                special_moves = special_moves.to_owned() + "`\n";
            }
            else {
                continue;
            }
        }
    }

    let mut super_moves = String::new();
    for moves in moves_info.iter().skip(move_index) {
        super_moves =
            super_moves.to_owned() + "\n- **" + &moves.input + " / " + &moves.name + "**";

        for moves_aliases in aliases_data.iter() {
            // If move input exists in aliases.json
            if moves.input == moves_aliases.input  {
                super_moves += "\n\tAliases → `";

                // Format message if there is only one alias or multiple
                for a in 0..moves_aliases.aliases.len() {
                    if a != moves_aliases.aliases.len() - 1 {
                        super_moves = super_moves.to_owned() + &moves_aliases.aliases[a] + "`, `";
                    }
                    else {
                        super_moves = super_moves.to_owned() + &moves_aliases.aliases[a];
                    }
                }
                super_moves = super_moves.to_owned() + "`\n";
            }
            else {
                continue;
            }
        }
    }

    let normals_embed = poise::serenity_prelude::CreateEmbed::new()
        .description(normal_moves)
        .color((140,75,64));

    let specials_embed = poise::serenity_prelude::CreateEmbed::new()
        .description(special_moves)
        .color((140,75,64));

    let embed_footer = poise::serenity_prelude::CreateEmbedFooter::
        new("Try the \"/help notes\" command for usage notes and specifics.\nOr \"/report\" to request a new alias.");
    
    let supers_embed = poise::serenity_prelude::CreateEmbed::new()
        .description(super_moves)
        .footer(embed_footer)
        .color((140,75,64));

    let vec_embeds = vec![normals_embed, specials_embed, supers_embed];

    let mut reply = poise::CreateReply::default();
    reply.embeds.extend(vec_embeds);

    ctx.send(reply).await?;

    Ok(())
}
