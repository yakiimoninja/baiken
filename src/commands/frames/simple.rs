use std::{fs, string::String};
use colored::Colorize;
use poise::serenity_prelude::{CreateEmbed, CreateEmbedFooter};
use crate::{check, find, ran, Context, Error, Gids, ImageLinks, MoveInfo, EMBED_COLOR, IMAGE_DEFAULT};

/// Display a move's basic frame data.
#[poise::command(prefix_command, slash_command)]
pub async fn simple(
    ctx: Context<'_>,
    #[min_length = 2]
    #[description = "Character name or nickname."] character: String,
    #[min_length = 2]
    #[rename = "move"]
    #[description = "Move name, input or alias."] character_move: String,
) -> Result<(), Error> {

    println!("{}", ("Command Args: '".to_owned() + &character + ", " + &character_move + "'").purple());

    // Initializing variables for the embed
    // They must not be empty cause then the embed wont be sent
    let mut embed_image = IMAGE_DEFAULT.to_string();

    if (check::adaptive_check(
        ctx,
        true,
        true,
        true,
        true,
        true,
        false,
        true).await).is_err() {
        
        return Ok(());
    }

    // Finding character
    // This will store the full character name in case user input was an alias
    let character_arg_altered = match find::find_character(&character).await {
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

    // Finding move index
    let index = match find::find_move_index(&character_arg_altered, character_move, &moves_info).await {
        Ok(index) => index,
        Err(err) => {
            ctx.say(err.to_string() + "\nView the moves of a character by executing `/moves`.").await?;
            println!("{}", ("Error: ".to_owned() + &err.to_string()).red());
            return Ok(()) }    
    };

    // Reading images.json for this character
    let image_links = fs::read_to_string("data/".to_owned() + &character_arg_altered + "/images.json")
        .expect(&("\nFailed to read 'data/".to_owned() + &character_arg_altered + "'/images.json' file."));

    // Deserializing images.json for this character
    let image_links = serde_json::from_str::<Vec<ImageLinks>>(&image_links).unwrap();
    let move_info = &moves_info[index];
    
    println!("{}", ("Successfully read move '".to_owned() + &move_info.input.to_string() + "' in '" + &character_arg_altered + ".json' file.").green());

    // Checking if the respective data field in the json file is empty
    // If they aren't empty, the variables initialized above will be replaced
    // With the corresponding data from the json file
    // Otherwise they will remain as '-'
    for img_links in image_links {
        // Iterating through the image.json to find the move's image links
        if move_info.input == img_links.input && !img_links.move_img.is_empty() {
            embed_image = img_links.move_img.to_string();
            break;
        }
    }

    // Debugging prints
    // println!("{}", content_embed);
    // println!("{}", embed_image);
    // println!("{}", embed_title);
    // println!("{}", damage_embed);
    // println!("{}", guard_embed);
    // println!("{}", invin_embed);
    // println!("{}", startup_embed);
    // println!("{}", hit_embed);
    // println!("{}", block_embed);
    // println!("{}", active_embed);
    // println!("{}", recovery_embed);
    // println!("{}", counter_embed);

    {
        // Reading the gids json
        let data_from_file = fs::read_to_string("data/gids.json")
            .expect("\nFailed to read 'gids.json' file.");

        // Deserializing from gids json
        let vec_gids = serde_json::from_str::<Gids>(&data_from_file).unwrap();

        // Parse user guild id to string
        let guild_id = ctx.guild_id().unwrap().to_string();

        let mut gid_found = false;

        // Hand to add guild id from exclusion list
        for x in vec_gids.id.iter() {
            // Checking if guild is in the exclusion list
            if guild_id == *x.to_string() {
                gid_found = true;
                break;
            }
        }
        if !gid_found {
            if let Some(image_path) = ran::random_p().await {
                embed_image = image_path;
            }
        }
    }

    let embed_title = "__**".to_owned()
        + &character_arg_altered.replace('_', " ") + " "
        + &move_info.input + " / " + &move_info.name + "**__";

    let embed_url = "https://dustloop.com/w/GGST/".to_owned() + &character_arg_altered + "#Overview";
    let embed_footer = CreateEmbedFooter::new(&move_info.caption);

    // Sending the data as an embed
    let embed = CreateEmbed::new()
        .color(EMBED_COLOR)
        .title(&embed_title)
        .url(&embed_url)
        .image(&embed_image)
        .fields(vec![
            ("Damage", &move_info.damage.to_string(), true),
            ("Guard", &move_info.guard.to_string(), true),
            ("Invinciblity", &move_info.invincibility.to_string(), true),
            ("Startup", &move_info.startup.to_string(), true),
            ("Active", &move_info.active.to_string(), true),
            ("Recovery", &move_info.recovery.to_string(), true),
            ("On Hit", &move_info.on_hit.to_string(), true),
            ("On Block", &move_info.on_block.to_string(), true),
            ("Counter", &move_info.counter.to_string(), true)
        ])
        .footer(embed_footer);
        
    ctx.send(poise::CreateReply::default().embed(embed)).await?;

    // New version notification
    //ctx.channel_id().say(ctx, r"[__**Patch.**__](<https://github.com/yakiimoninja/baiken/releases>)").await?;
    Ok(())
}
