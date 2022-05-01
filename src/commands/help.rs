use serenity::framework::standard::{macros::command, CommandResult};
use serenity::model::prelude::*;
use serenity::prelude::*;

#[command]
#[aliases("?")]
async fn help(ctx: &Context, msg: &Message) -> CommandResult {

    let help_message = r#"
__**Commands:**__
__<https://github.com/yakiimoninja/baiken/blob/main/README.md#commands>__
__**Usage Notes:**__
__<https://github.com/yakiimoninja/baiken/blob/main/README.md#usage-notes>__
__**Patch Notes:**__
__<https://github.com/yakiimoninja/baiken/blob/main/patch_notes/v0.4.0.md>__
__**Source Code:**__
__<https://github.com/yakiimoninja/baiken>__
__**Artwork:**__
__<https://twitter.com/gogalking>__
"#;

    msg.channel_id.say(&ctx.http, help_message).await?;
    Ok(())
}
