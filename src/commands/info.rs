use std::{fs, string::String};
use colored::Colorize;
use crate::{check, find, CharInfo, Context, Error};

/// Display a character's general info.
#[poise::command(prefix_command, slash_command)]
pub async fn info(
    ctx: Context<'_>,
    #[description = "Character name or nickname."] character: String,
) -> Result<(), Error> {

    println!("{}", ("Command Args: '".to_owned() + &character + "'").purple());

    if (check::adaptive_check(
        ctx,
        (true, &character),
        (false, &String::new()),
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

    let msg = "## **[__".to_owned()
        + &character_arg_altered.replace('_', " ")
        + " Info__](<https://dustloop.com/wiki/index.php?title=GGST/"
        + &character_arg_altered + "/Data#Infobox_Data>)**" +
        "\n- **Defense →** " + &char_info.defense +
        "\n- **Guts →** " + &char_info.guts +
        "\n- **Risc Gain Modifier →** " + &char_info.guardbalance +
        "\n- **Prejump →** " + &char_info.prejump +
        "\n- **Dash →** " + &char_info.forwarddash +
        "\n- **Backdash →** " + &char_info.backdash +
        "\n- **Unique Movement →** " + &char_info.umo +
        "\n- **Jump Duration →** " + &char_info.jump_duration +
        "\n- **Super Jump Duration →** " + &char_info.high_jump_duration +
        "\n- **Jump Height →** " + &char_info.jump_height +
        "\n- **Super Jump Height →** " + &char_info.high_jump_height +
        "\n- **Earliest IAD →** " + &char_info.earliest_iad +
        "\n- **AD Duration →** " + &char_info.ad_duration +
        "\n- **ABD Duration →** " + &char_info.abd_duration +
        "\n- **AD Distance →** " + &char_info.ad_distance +
        "\n- **ABD Distance →** " + &char_info.abd_distance +
        "\n- **Movement Tension →** " + &char_info.movement_tension +
        "\n- **Jump Tension →** " + &char_info.jump_tension +
        "\n- **AD Tension →** " + &char_info.airdash_tension +
        "\n- **Walk Speed →** " + &char_info.walk_speed +
        "\n- **Back Walk Speed →** " + &char_info.back_walk_speed +
        "\n- **Dash Initial Speed →** " + &char_info.dash_initial_speed +
        "\n- **Dash Acceleration →** " + &char_info.dash_acceleration +
        "\n- **Dash Friction →** " + &char_info.dash_friction +
        "\n- **Jump Gravity →** " + &char_info.jump_gravity +
        "\n- **Super Jump Gravity →** " + &char_info.high_jump_gravity;

    ctx.say(&msg).await?;
    Ok(())
}
