use serenity::framework::standard::{macros::command, CommandResult};
use serenity::model::prelude::*;
use serenity::prelude::*;

#[command]
#[aliases("?")]
async fn help(ctx: &Context, msg: &Message) -> CommandResult {

    let help_message = r#"
__**Commands**__

• Command: `b.f` or `b.frames`. 
      Displays the frame data of a move.
      Use `b.f` or `b.frames` followed by the `character` and the `move` you want.
      Example: `b.f baiken 236K` or `b.frames baiken 236K`.

• Command: `b.h` or `b.hitbox`. 
      Displays the hitbox images of a move.
      Use `b.h` or `b.hitbox` followed by the `character` and the `move` you want.
      Example: `b.h goldlewis 4126H` or `b.hitbox goldlewis 4126H`.

• Command: `b.m` or `b.moves`.
      Displays all the moves and their inputs of a character.
      Use `b.m` or `b.moves` followed by the `character` you want.
      Example: `b.m sol` or `b.moves sol`.

• Command: `b.a` or `b.aliases`.
      Displays all the aliases for each special/super move of a character.
      Use `b.a` or `b.aliases` followed by the `character` you want.
      Example: `b.a leo` or `b.moves leo`.

• Command: `b.u` or `b.update`.
      Updates the frame data according to <https://dustloop.com>.

• Command: `b.?` or `b.help`.
      Displays this help message.

__**Notes**__

• Searching is case insensitive.
      This works: `b.hitbox Ky DP`.
      This also works: `b.h ky dp`.

• You don't have to write the full name of a character.
      This works: `b.frames giovanna 236K`.
      This will also work: `b.f gio 236K`.

• You can also use aliases when searching for moves.
      This works: `b.frames Anji 236HP`.
      This also works: `b.f Anji Needles`.

• You can omit the '[]' when searching for a charged move.
      This works: `b.frames May [4]6S`.
      This also works: `b.f may 46S`.

• But when searching for charged dust attack use the alias `5D!`.
      Example: `b.f chipp 5D!`.

__**Github**__
<https://github.com/yakiimoninja/baiken-bot>
"#;

    msg.channel_id.say(&ctx.http, help_message).await?;
    Ok(())
}