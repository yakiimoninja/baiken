use crate::{check, find::find_all, Context, Error, CHARS, EMBED_COLOR};
use poise::{serenity_prelude::{CreateEmbed, CreateEmbedFooter}, ChoiceParameter};

#[derive(Debug, poise::ChoiceParameter)]
pub enum TypeChoice {
    #[name = "all"]
    All,
    #[name = "normals"]
    Normals,
    #[name = "specials"]
    Specials,
    #[name = "supers"]
    Supers,
}

#[derive(Debug, poise::ChoiceParameter)]
pub enum FilterBy {
    #[name = "damage"]
    Damage,
    #[name = "startup"]
    Startup,
    #[name = "active"]
    Active,
    #[name = "recovery"]
    Recovery,
    #[name = "on hit"]
    Onhit,
    #[name = "on block"]
    Onblock,
    #[name = "attack level"]
    Level,
    #[name = "counter level"]
    Counter,
    #[name = "guard"]
    Guard,
}

/// Display a filtered list of moves.
#[poise::command(prefix_command, slash_command)]
pub async fn list(
    ctx: Context<'_>,
    #[rename = "type"]
    #[description = "Move type."] move_type: TypeChoice,
    #[rename = "filter"]
    #[description = "Filter to apply."] filter_type: FilterBy,
    #[min_length = 1]
    #[description = "Filter value."] value: String,
) -> Result<(), Error> {

    if (check::adaptive_check(ctx, true, false).await).is_err() {
        return Ok(());
    }

    let field: String = match filter_type {
        FilterBy::Startup => String::from("startup"),
        FilterBy::Damage => String::from("damage"),
        FilterBy::Active => String::from("active"),
        FilterBy::Recovery => String::from("recovery"),
        FilterBy::Onhit => String::from("on_hit"),
        FilterBy::Onblock => String::from("on_block"),
        FilterBy::Level => String::from("level"),
        FilterBy::Counter => String::from("counter"),
        FilterBy::Guard => String::from("guard"),
    };

    let type_of_move: String = match move_type {
        TypeChoice::All => String::from(""),
        TypeChoice::Normals => String::from("normal"),
        TypeChoice::Specials => String::from("special"),
        TypeChoice::Supers => String::from("super")
    };

    let move_list = match find_all(&type_of_move, &field, &value, ctx.data().db.clone()).await {
        Ok(move_list) => move_list,
        Err(err) => {
            ctx.say(err.to_string()).await?;
            return Ok(());
        }
    };

    if move_list.is_empty() {
        ctx.say("No moves found with this criteria!").await?;
        return Ok(());
    }

    let embed_title = "__**".to_owned() + "List of all moves filtered by:**__\n**Move type:** `"
        + move_type.name()+ "`, **" + filter_type.name() + ":** `" + &value + "`";
    let embed_footer = CreateEmbedFooter::new(
       "Execute \"/report\" to report any bugs.");
    
    let mut msg = String::new();
    for (x, move_struct) in move_list.iter().enumerate() {

        println!("{:#?}", move_struct);

        if x == 0 || (x > 0 && move_list[x].char_id != move_list[x-1].char_id) {
            if move_struct.name == move_struct.input {
                msg += &("- **".to_owned() + CHARS[move_struct.char_id-1] + " →** `" +  &move_struct.input + "`");
            }
            else {
                msg += &("- **".to_owned() + CHARS[move_struct.char_id-1] + " →** `" + &move_struct.name + " / " + &move_struct.input + "`");
            }
        }
        
        //fix multiple entries by same character to display in one line
        if x + 1 < move_list.len() {
            if move_list[x].char_id == move_list[x+1].char_id {
                msg += &(", `".to_owned() + &move_list[x+1].name + " / " + &move_list[x+1].input + "`");
            }
            else {
                msg += "\n";
            }
        }
    }

    let embed = CreateEmbed::new()
        .color(EMBED_COLOR)
        .title(embed_title)
        .description(msg)
        .footer(embed_footer);

    ctx.send(poise::CreateReply::default().embed(embed)).await?;

    Ok(())
}

