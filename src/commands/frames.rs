use std::{fs, string::String};
use colored::Colorize;
use poise::serenity_prelude::CreateEmbed;
use crate::{Context, Error, ImageLinks , MoveInfo, ran };
use crate::{IMAGE_DEFAULT, find, check, Gids};

/// Display a move's frame data.
#[allow(unused_assignments)]
#[poise::command(prefix_command, slash_command)]
pub async fn frames(
    ctx: Context<'_>,
    #[description = "Character name or nickname."] character: String,
    #[description = "Move name, input or alias."] mut character_move: String,
) -> Result<(), Error> {

    println!("{}", ("Command Args: '".to_owned() + &character + ", " + &character_move + "'").purple());

    // This will store the full character name in case user input was an alias
    let mut character_arg_altered = String::new();
    // Initializing variables for the embed
    // They must not be empty cause then the embed wont be sent
    let mut image_embed = IMAGE_DEFAULT.to_string();

    if (check::adaptive_check(
        ctx,
        (true, &character),
        (true, &character_move),
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
    let mframes_index = find::find_move_index(&character_arg_altered, character_move, &moves_info).await;
    let mframes_index = match mframes_index {
        Ok(index) => index,
        Err(err) => {
            ctx.say(err.to_string() + "\nView the moves of a character by executing `/moves`.").await?;
            println!("{}", ("Error: ".to_owned() + &err.to_string()).red());
            return Ok(()) }    
    };

    // TODO find a fix for this
    character_move = mframes_index.1;

    // Reading images.json for this character
    let image_links = fs::read_to_string("data/".to_owned() + &character_arg_altered + "/images.json")
        .expect(&("\nFailed to read 'data/".to_owned() + &character_arg_altered + "'/images.json' file."));

    // Deserializing images.json for this character
    let image_links= serde_json::from_str::<Vec<ImageLinks>>(&image_links).unwrap();

    let mframes = &moves_info[mframes_index.0];
    
    println!("{}", ("Successfully read move '".to_owned() + &mframes.input.to_string() + "' in '" + &character_arg_altered + ".json' file.").green());
    
    let content_embed = r#"## **[__"#.to_owned()
        + &character_arg_altered.replace('_', " ") + " "
        + &mframes.input.to_string()
            .replace("]P[","|P|")
            .replace("]K[","|K|")
            .replace("]S[","|S|")
            .replace("]H[","|H|")
        + "__](<https://dustloop.com/wiki/index.php?title=GGST/"
        + &character_arg_altered + "#Overview>)**";
    //let title_embed = "Move: ".to_owned() + &mframes.input.to_string();

    // Checking if the respective data field in the json file is empty
    // If they aren't empty, the variables initialized above will be replaced
    // With the corresponding data from the json file
    // Otherwise they will remain as '-'
    for img_links in image_links {
        // Iterating through the image.json to find the move's image links
        if mframes.input == img_links.input && !img_links.move_img.is_empty() {
            image_embed = img_links.move_img.to_string();
            break;
        }
    }

    // Debugging prints
    // println!("{}", content_embed);
    // println!("{}", image_embed);
    // println!("{}", title_embed);
    // println!("{}", damage_embed);
    // println!("{}", guard_embed);
    // println!("{}", invin_embed);
    // println!("{}", startup_embed);
    // println!("{}", hit_embed);
    // println!("{}", block_embed);
    // println!("{}", active_embed);
    // println!("{}", recovery_embed);
    // println!("{}", counter_embed);


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
    if gid_found == false {
        if let Some(image_path) = ran::random_p().await {
            image_embed = image_path;
        }
    }


    // Sending the data as an embed
    let embed = CreateEmbed::new()
        //.description("This is a description") 
        .color((140,75,64))
        //.title(&title_embed)
        .image(&image_embed)
        .fields(vec![
            ("Damage", &mframes.damage.to_string(), true),
            ("Guard", &mframes.guard.to_string(), true),
            ("Invinciblity", &mframes.invincibility.to_string(), true),
            ("Startup", &mframes.startup.to_string(), true),
            ("Active", &mframes.active.to_string(), true),
            ("Recovery", &mframes.recovery.to_string(), true),
            ("On Hit", &mframes.hit.to_string(), true),
            ("On Block", &mframes.block.to_string(), true),
            ("Level", &mframes.level.to_string(), true),
            ("Risc Gain", &mframes.riscgain.to_string(), true),
            ("Scaling", &mframes.scaling.to_string(), true),
            ("Counter", &mframes.counter.to_string(), true)
        ]);
        //.field("This is the third field", "This is not an inline field", false)
        //.footer(footer)
        // Add a timestamp for the current time
        // This also accepts a rfc3339 Timestamp
        //.timestamp(Timestamp::now());
        
    ctx.send(poise::CreateReply::default()
        .content(&content_embed)
        .embed(embed)
        //.add_file(CreateAttachment::path("./ferris_eyes.png").await.unwrap());
    ).await?;

    // New version notification
    ctx.channel_id().say(ctx, r"[Patch.](<https://github.com/yakiimoninja/baiken/releases>)").await?;

    Ok(())
}
