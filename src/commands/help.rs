use crate::{Context, Error};

/// Prints a help message.
#[poise::command(prefix_command, slash_command, aliases("?"))]
pub async fn help(ctx: Context<'_>) -> Result<(), Error> {

    let help_message = r#"
__**Patch Notes:**__
__<https://github.com/yakiimoninja/baiken/releases>__

__**Commands:**__
__<https://github.com/yakiimoninja/baiken/blob/main/README.md#commands>__

__**Usage Notes:**__
__<https://github.com/yakiimoninja/baiken/blob/main/README.md#usage-notes>__

__**Source Code:**__
__<https://github.com/yakiimoninja/baiken>__

__**Artwork:**__
__<https://twitter.com/gogalking/status/1307199393607553024>__
"#;

    ctx.say(help_message).await?;
    Ok(())
}
