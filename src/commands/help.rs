use serenity::framework::standard::{macros::command, CommandResult};
use serenity::model::prelude::*;
use serenity::prelude::*;

#[command]
async fn h (ctx: &Context, msg: &Message) -> CommandResult {

    let help_message = r#"
__**Commands**__
• Command: `b.f` or `b.frames`. 
• Displays the frame data of a move.
• Use `b.f` or `b.frames` followed by the `character` and the `move` you want.
• Example: `b.f baiken 236K` or `b.frames baiken 236K`.

• Command: `b.m` or `b.moves`.
• Displays all the moves and their inputs of a character.
• Use `b.m` or `b.moves` followed by the `character` you want.
• Example: `b.m sol` or `b.moves sol`.

• Command: `b.a` or `b.aliases`.
• Displays all the aliases for each special/super move of a character.
• Use `b.a` or `b.aliases` followed by the `character` you want.
• Example: `b.a leo` or `b.moves leo`.

• Command: `b.h` or `b.help` or `b.?`.
• Displays this help message.

__**Github**__
https://github.com/yakiimoninja/baiken-bot
"#;

    msg.channel_id.say(&ctx.http, help_message).await?;
    Ok(())
}