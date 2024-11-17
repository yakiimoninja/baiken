use crate::{check, find, Context, Error, MoveAliases, MoveInfo};
use colored::Colorize;
use std::{fs, string::String};

/// Display a character's moves, inputs and aliases.
#[poise::command(prefix_command, slash_command)]
pub async fn moves(
    ctx: Context<'_>,
    #[description = "Character name or nickname."] character: String,
) -> Result<(), Error> {
    println!(
        "{}",
        ("Command Args: '".to_owned() + &character + "'").purple()
    );

    if (check::adaptive_check(
        ctx,
        (true, &character),
        (false, &String::new()),
        true,
        true,
        true,
        true,
        false,
        false,
        false,
    )
    .await)
        .is_err()
    {
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

    println!(
        "{}",
        ("Successfully read '".to_owned() + &character_arg_altered + ".json' file.").green()
    );

    // Reading the aliases json
    let aliases_path = "data/".to_owned() + &character_arg_altered + "/aliases.json";
    let aliases_data = fs::read_to_string(&aliases_path)
        .expect(&("\nFailed to read '".to_owned() + &aliases_path + "' file."));

    // Deserializing the aliases json
    let aliases_data = serde_json::from_str::<Vec<MoveAliases>>(&aliases_data).unwrap();

    // Formatting string for in discord print
    // Masked dustloop link
    let mut moves_as_msg = "## **[__".to_owned()
        + &character_arg_altered.replace('_', " ")
        + " Moves / Aliases__](<https://dustloop.com/wiki/index.php?title=GGST/"
        + &character_arg_altered
        + "#Overview>)**";

    let mut move_index;
    // Message split due to discord character limit
    for (x, moves) in moves_info.iter().enumerate() {
        moves_as_msg =
            moves_as_msg.to_owned() + "\n- **" + &moves.name + " / " + &moves.input + "**";

        'outer: for moves_aliases in aliases_data.iter() {
            // If move input exists in aliases.json
            if moves.input == moves_aliases.input && moves.move_type.to_lowercase() == "normal" {
                moves_as_msg += "\n\t\tAliases → `";

                // Format message if there is only one alias or multiple
                for a in 0..moves_aliases.aliases.len() {
                    if a != moves_aliases.aliases.len() - 1 {
                        moves_as_msg = moves_as_msg.to_owned() + &moves_aliases.aliases[a] + "`, `";
                    } else {
                        moves_as_msg = moves_as_msg.to_owned() + &moves_aliases.aliases[a];
                    }
                }
                moves_as_msg = moves_as_msg.to_owned() + "`";
            }
            else if moves.move_type.to_lowercase() != "normal" {
                move_index = x;
                break 'outer;
            }
            else {
                continue;
            }
        }
    }
    //ctx.say(&moves_as_msg).await?;

    // 2nd message builder
    let mut moves_as_msg2 = "".to_string();
    for moves in moves_info
        .iter()
        .take((moves_info.len() / 4) * 2)
        .skip(moves_info.len() / 4)
    {
        moves_as_msg2 =
            moves_as_msg2.to_owned() + "\n- **" + &moves.name + " / " + &moves.input + "**";

        for moves_aliases in aliases_data.iter() {
            // If move input exists in aliases.json
            if moves.input == moves_aliases.input {
                moves_as_msg2 += "\n\t\tAliases → `";

                // Format message if there is only one alias or multiple
                for a in 0..moves_aliases.aliases.len() {
                    if a != moves_aliases.aliases.len() - 1 {
                        moves_as_msg2 =
                            moves_as_msg2.to_owned() + &moves_aliases.aliases[a] + "`, `";
                    } else {
                        moves_as_msg2 = moves_as_msg2.to_owned() + &moves_aliases.aliases[a];
                    }
                }
                moves_as_msg2 = moves_as_msg2.to_owned() + "`";
            } else {
                continue;
            }
        }
    }
    //ctx.channel_id().say(ctx, &moves_as_msg2).await?;

    // 3rd message builder
    let mut moves_as_msg3 = "".to_string();
    for moves in moves_info
        .iter()
        .take((moves_info.len() / 4) * 3)
        .skip((moves_info.len() / 4) * 2)
    {
        moves_as_msg3 =
            moves_as_msg3.to_owned() + "\n- **" + &moves.name + " / " + &moves.input + "**";

        for moves_aliases in aliases_data.iter() {
            // If move input exists in aliases.json
            if moves.input == moves_aliases.input {
                moves_as_msg3 += "\n\t\tAliases → `";

                // Format message if there is only one alias or multiple
                for a in 0..moves_aliases.aliases.len() {
                    if a != moves_aliases.aliases.len() - 1 {
                        moves_as_msg3 =
                            moves_as_msg3.to_owned() + &moves_aliases.aliases[a] + "`, `";
                    } else {
                        moves_as_msg3 = moves_as_msg3.to_owned() + &moves_aliases.aliases[a];
                    }
                }
                moves_as_msg3 = moves_as_msg3.to_owned() + "`";
            } else {
                continue;
            }
        }
    }
    //ctx.channel_id().say(ctx, &moves_as_msg3).await?;

    // 4th message builder
    let mut moves_as_msg4 = "".to_string();
    for moves in moves_info.iter().skip((moves_info.len() / 4) * 3) {
        moves_as_msg4 =
            moves_as_msg4.to_owned() + "\n- **" + &moves.name + " / " + &moves.input + "**";

        for moves_aliases in aliases_data.iter() {
            // If move input exists in aliases.json
            if moves.input == moves_aliases.input {
                moves_as_msg4 += "\n\t\tAliases → `";

                // Format message if there is only one alias or multiple
                for a in 0..moves_aliases.aliases.len() {
                    if a != moves_aliases.aliases.len() - 1 {
                        moves_as_msg4 =
                            moves_as_msg4.to_owned() + &moves_aliases.aliases[a] + "`, `";
                    } else {
                        moves_as_msg4 = moves_as_msg4.to_owned() + &moves_aliases.aliases[a];
                    }
                }
                moves_as_msg4 = moves_as_msg4.to_owned() + "`";
            } else {
                continue;
            }
        }
    }

    // GL shenanigans
    //

    //ctx.channel_id().say(ctx, &moves_as_msg4).await?;
    // Sending the data as an embed
    let embed = poise::serenity_prelude::CreateEmbed::new()
        .description(moves_as_msg + &moves_as_msg2 + &moves_as_msg3 + &moves_as_msg4)
        //.author(CreateEmbedAuthor::new("dustloop"))
        .color((140,75,64))
        //.title(&title_embed)
        //.image(&image_embed)
        //.field("", &moves_as_msg, false)
        //.field("", &moves_as_msg2, false)
        //.field("", &moves_as_msg3, false)
        //.field("", &moves_as_msg4, false)
        //.fields(vec![("Damage", &mframes.damage.to_string(), true)])
        ;
    //.field("This is the third field", "This is not an inline field", false)
    //.footer(footer)
    // Add a timestamp for the current time
    // This also accepts a rfc3339 Timestamp
    //.timestamp(Timestamp::now());

    let vec_embeds = vec![embed];
    let mut reply = poise::CreateReply::default();
    reply.embeds.extend(vec_embeds);
        //.content(&msg)

    ctx.send(reply).await?;
    
    ctx.channel_id().say(ctx, "Try the `/help notes` command for usage notes and specifics.\nOr `/request` to request a new alias.").await?;

    Ok(())
}
