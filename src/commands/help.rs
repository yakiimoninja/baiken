use crate::{Context, Error};

/// Prints a help message.
#[poise::command(prefix_command, slash_command, aliases("?"))]
pub async fn help(ctx: Context<'_>) -> Result<(), Error> {

    let help_message = r#"
__**Patch Notes:**__
__<https://github.com/yakiimoninja/baiken/releases>__

__**Commands:**__
__<https://github.com/yakiimoninja/baiken#commands>__

__**Support The Project:**__
__<https://github.com/sponsors/yakiimoninja>__

__**Artwork:**__
__<https://twitter.com/gogalking/status/1307199393607553024>__
"#;

    ctx.say(help_message).await?;
    Ok(())
}
