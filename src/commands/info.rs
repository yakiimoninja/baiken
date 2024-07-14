use std::{fs, string::String};
use colored::Colorize;
use crate::{check, find, CharInfo, Context, Error};

/// Displays general info for a character.
#[poise::command(prefix_command, slash_command, aliases("i"))]
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
        true).await).is_err() {
        
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
    let char_info = serde_json::from_str::<Vec<CharInfo>>(&char_file_data).unwrap();  

    let msg = "## **[__".to_owned()
        + &character_arg_altered.replace('_', " ")
        + " Info__](<https://dustloop.com/wiki/index.php?title=GGST/"
        + &character_arg_altered + "/Data>)**" +
        "\n- **Defense →** " + &char_info[0].defense +
        "\n- **Guts →** " + &char_info[0].guts +
        "\n- **Risc Gain Modifier →** " + &char_info[0].guardbalance +
        "\n- **Prejump →** " + &char_info[0].prejump +
        "\n- **Dash →** " + &char_info[0].forwarddash +
        "\n- **Backdash →** " + &char_info[0].backdash +
        "\n- **Unique Movement →** " + &char_info[0].umo +
        "\n- **Jump Duration →** " + &char_info[0].jump_duration +
        "\n- **Super Jump Duration →** " + &char_info[0].high_jump_duration +
        "\n- **Jump Height →** " + &char_info[0].jump_height +
        "\n- **Super Jump Height →** " + &char_info[0].high_jump_height +
        "\n- **Earliest IAD →** " + &char_info[0].earliest_iad +
        "\n- **AD Duration →** " + &char_info[0].ad_duration +
        "\n- **ABD Duration →** " + &char_info[0].abd_duration +
        "\n- **AD Distance →** " + &char_info[0].ad_distance +
        "\n- **ABD Distance →** " + &char_info[0].abd_distance +
        "\n- **Movement Tension →** " + &char_info[0].movement_tension +
        "\n- **Jump Tension →** " + &char_info[0].jump_tension +
        "\n- **AD Tension →** " + &char_info[0].airdash_tension +
        "\n- **Walk Speed →** " + &char_info[0].walk_speed +
        "\n- **Back Walk Speed →** " + &char_info[0].back_walk_speed +
        "\n- **Dash Initial Speed →** " + &char_info[0].dash_initial_speed +
        "\n- **Dash Acceleration →** " + &char_info[0].dash_acceleration +
        "\n- **Dash Friction →** " + &char_info[0].dash_friction +
        "\n- **Jump Gravity →** " + &char_info[0].jump_gravity +
        "\n- **Super Jump Gravity →** " + &char_info[0].high_jump_gravity;

    ctx.say(&msg).await?;
    Ok(())
}