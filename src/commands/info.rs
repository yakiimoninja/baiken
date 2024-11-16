use std::{fs, string::String};
use colored::Colorize;
use crate::{check, find, CharInfo, Context, Error};
use poise::{reply, serenity_prelude::{CreateEmbed, CreateEmbedAuthor}};

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

    let msg = "# **[__".to_owned()
        + &character_arg_altered.replace('_', " ")
        + " Info__](<https://dustloop.com/wiki/index.php?title=GGST/"
        + &character_arg_altered + "/Data#Infobox_Data>)**" +
        "\n- **Defense** → " + &char_info.defense +
        "\n- **Guts →** " + &char_info.guts +
        "\n- **Risc Gain Modifier →** " + &char_info.guard_balance +
        "\n- **Prejump →** " + &char_info.prejump +
        "\n- **Dash →** " + &char_info.forward_dash +
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

    // Sending the data as an embed
    let embed = CreateEmbed::new()
        .description(&msg)
        //.author(CreateEmbedAuthor::new("dustloop"))
        .color((140,75,64))
        //.title(&title_embed)
        //.image(&image_embed)
        //.field("", &msg, false)
        .field("**Guts**:", &char_info.defense, false)
        //.fields(vec![("Damage", &mframes.damage.to_string(), true)])
        ;
        //.field("This is the third field", "This is not an inline field", false)
        //.footer(footer)
        // Add a timestamp for the current time
        // This also accepts a rfc3339 Timestamp
        //.timestamp(Timestamp::now());
    let embed2 = CreateEmbed::new()
        //.description("This is a description")
        //.author(CreateEmbedAuthor::new("dustloop"))
        .color((140,75,64))
        //.title(&title_embed)
        //.image(&image_embed)
        .field("", &msg, false)
        .field("**Guts**:", &char_info.defense, false)
        //.fields(vec![("Damage", &mframes.damage.to_string(), true)])
        ;
    let vec_embeds = vec![embed, embed2];

    let mut reply = poise::CreateReply::default();
    reply.embeds.extend(vec_embeds);
        //.content(&msg)


    ctx.send(reply).await?;
        //.add_file(CreateAttachment::path("./ferris_eyes.png").await.unwrap());
    Ok(())
}
