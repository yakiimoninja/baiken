use std::{fs, string::String};
use crate::serenity::futures::{Stream, StreamExt, self};
use crate::{Context, Error, ImageLinks , MoveInfo, ran };
use crate::{IMAGE_DEFAULT, CHARS, find, check};

// Autocompletes the character name
async fn autocomplete_character<'a>(
    _ctx: Context<'_>,
    partial: &'a str,
) -> impl Stream<Item = String> + 'a {
    futures::stream::iter(&CHARS)
        .filter(move |name| futures::future::ready(name.to_lowercase().contains(&partial.to_lowercase())))
        .map(|name| name.to_string())
}

/// Displays the frame data of a move along with an image.
#[allow(unused_assignments)]
#[poise::command(prefix_command, slash_command, aliases("f"))]
pub async fn frames(
    ctx: Context<'_>,
    #[description = "Character name or nickname."]
    #[autocomplete = "autocomplete_character"] character: String,
    #[description = "Move name, input or alias."] mut character_move: String,
) -> Result<(), Error> {

    println!("Command Args: '{}, {}'", character, character_move);

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
        true).await).is_err() {
        
        return Ok(());
    }

    // Finding character
    character_arg_altered = match find::find_character(&character).await {
        Ok(character_arg_altered) => character_arg_altered,
        Err(err) => {
            ctx.say(err.to_string()).await?;
            println!("Error: {}", err);
            return Ok(()) }
    };

    // Reading the character json
    let char_file_path = "data/".to_owned() + &character_arg_altered + "/" + &character_arg_altered + ".json";
    let char_file_data = fs::read_to_string(char_file_path)
            .expect(&("\nFailed to read '".to_owned() + &character_arg_altered + ".json" + "' file."));

    // Deserializing from character json
    let moves_info = serde_json::from_str::<Vec<MoveInfo>>(&char_file_data).unwrap();
           
    println!("Successfully read '{}.json' file.", character_arg_altered);

    // Finding move struct index 
    let mframes_index = find::find_move_index(&character_arg_altered, character_move, &moves_info).await;
    let mframes_index = match mframes_index {
        Ok(index) => index,
        Err(err) => {
            ctx.say(err.to_string() + "\nView the moves of a character by executing `/moves`.").await?;
            println!("Error: {}", err);
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
    
    println!("Successfully read move '{}' in '{}.json' file.", &mframes.input.to_string(), character_arg_altered);
    
    let content_embed = "https://dustloop.com/wiki/index.php?title=GGST/".to_owned() + &character_arg_altered + "/Frame_Data";
    let title_embed = "Move: ".to_owned() + &mframes.input.to_string();

    // Checking if the respective data field in the json file is empty
    // If they aren't empty, the variables initialized above will be replaced
    // With the corresponind data from the json file
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

    // New version notification
    //ctx.say(r"Baiken enters season 2 with a new version 0.5.0!
//As always a link to the patch notes is below.
//__<https://github.com/yakiimoninja/baiken/releases>__").await?;

    if let Some(image_path) = ran::random_p().await {
        image_embed = image_path;
    }

    // Sending the data as an embed
    let _msg = ctx.send(|m| {
        m.content(&content_embed);
        m.embed(|e| {
            e.color((140,75,64));
            e.title(&title_embed);
            //e.description("This is a description");
            e.image(&image_embed);
            e.fields(vec![
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
                ("Counter", &mframes.counter.to_string(), true)]);
            //e.field("This is the third field", "This is not an inline field", false);
            e
        });
        m
    }).await;

    Ok(())
}
