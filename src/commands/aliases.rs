use std::string::String;
use crate::{Context, Error};

/// Displays all the aliases for each normal/special/super move of a character.
#[poise::command(prefix_command, slash_command, aliases("a"))]
pub async fn aliases(
    ctx: Context<'_>,
    #[description = "Character name or nickname."] character: String,
) -> Result<(), Error> {

    println!("Command Args: '{}'", character);

    ctx.say("The `/aliases` command has been integrated to `/moves`.\nTry using that one instead.").await?;
    Ok(())
}
