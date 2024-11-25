use std::{fs, string::String};
use colored::Colorize;
use poise::serenity_prelude::CreateEmbed;
use crate::{check, find, CharInfo, Context, Error, EMBED_COLOR};

/// Display a character's general info.
#[poise::command(prefix_command, slash_command)]
pub async fn info(
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
        false,
        false,
        true,
        false).await).is_err() {
        
        return Ok(());
    }

    // Finding character
    let character_arg_altered = match find::find_character(&character).await {
    Ok(character_arg_altered) => character_arg_altered,
    Err(err) => {
        ctx.say(err.to_string()).await?;
        println!("{}", ("Error: ".to_owned() + &err.to_string()).red());
        return Ok(()) }
    };

    // Reading the character info json
    let char_file_path = "data/".to_owned() + &character_arg_altered + "/info.json";
    let char_file_data = fs::read_to_string(char_file_path)
        .expect(&("\nFailed to read ".to_owned() + &character + " 'info.json' file."));

    // Deserializing from character json
    let char_info = serde_json::from_str::<CharInfo>(&char_file_data).unwrap();  

    let embed_title = "__**".to_owned() + &character_arg_altered.replace('_', " ") + " Info**__";
    let embed_url = "https://dustloop.com/w/GGST/".to_owned() + &character_arg_altered + "/Data#Infobox_Data";

    let msg =
        "- **Defense →**  ".to_owned() + &char_info.defense +
        "\n- **Guts →**  " + &char_info.guts +
        "\n- **Risc Gain Modifier →**  " + &char_info.guard_balance +
        "\n- **Prejump →**  " + &char_info.prejump +
        "\n- **Unique Movement →**  " + &char_info.umo +
        "\n- **Forward Dash →**  " + &char_info.forward_dash +
        "\n- **Backdash →**  " + &char_info.backdash +
        "\n- **Backdash Duration →**  " + &char_info.backdash_duration +
        "\n- **Backdash Invincibility →**  " + &char_info.backdash_invincibility +
        "\n- **Backdash Airborne →**  " + &char_info.backdash_airborne +
        "\n- **Backdash Distance →**  " + &char_info.backdash_distance +
        "\n- **Jump Duration →**  " + &char_info.jump_duration +
        "\n- **Jump Height →**  " + &char_info.jump_height +
        "\n- **Super Jump Duration →**  " + &char_info.high_jump_duration +
        "\n- **Super Jump Height →**  " + &char_info.high_jump_height +
        "\n- **Earliest Instant Airdash →**  " + &char_info.earliest_iad +
        "\n- **Air Dash duration →**  " + &char_info.ad_duration +
        "\n- **Air Dash Distance →**  " + &char_info.ad_distance +
        "\n- **Air Backdash Duration →**  " + &char_info.abd_duration +
        "\n- **Air Backdash Distance →**  " + &char_info.abd_distance +
        "\n- **Movement Tension →**  " + &char_info.movement_tension +
        "\n- **Jump Tension →**  " + &char_info.jump_tension +
        "\n- **Airdash Tension →**  " + &char_info.airdash_tension +
        "\n- **Walk Speed →**  " + &char_info.walk_speed +
        "\n- **Back Walk Speed →**  " + &char_info.back_walk_speed +
        "\n- **Dash Initial Speed →**  " + &char_info.dash_initial_speed +
        "\n- **Dash Acceleration →**  " + &char_info.dash_acceleration +
        "\n- **Dash Friction →**  " + &char_info.dash_friction +
        "\n- **Jump Gravity →**  " + &char_info.jump_gravity +
        "\n- **Super Jump Gravity →**  " + &char_info.high_jump_gravity;

    // Sending the data as an embed
    let embed = CreateEmbed::new()
        .color(EMBED_COLOR)
        .title(embed_title)
        .url(embed_url)
        .description(msg);

    //let vec_embeds = vec![embed];
    //let mut reply = poise::CreateReply::default();
    //reply.embeds.extend(vec_embeds);
        //.content(&msg)

    ctx.send(poise::CreateReply::default().embed(embed)).await?;

    Ok(())
}
