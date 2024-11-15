use colored::Colorize;
use crate::{
    Context,
    Error,
    serenity::futures::{
        Stream,
        StreamExt,
        self
    }
};

async fn autocomplete_help<'a>(
    _ctx: Context<'_>,
    partial: &'a str,
) -> impl Stream<Item = String> + 'a {
    futures::stream::iter(&[
        "general",
        "frames",
        "hitboxes",
        "fmeter",
        "moves",
        "info",
        "nicknames",
        "notes",
        "register",
        "update",
        "feedback"])
        .filter(move |name| futures::future::ready(name.to_lowercase().contains(&partial.to_lowercase())))
        .map(|name| name.to_string())
}

/// Display a help message.
#[poise::command(prefix_command, slash_command)]
pub async fn help(ctx: Context<'_>,
    #[description = "Pick a command to display help for."] 
    #[autocomplete = "autocomplete_help"] option: String
    ) -> Result<(), Error> {
    
        println!("{}", ("Command Args: '".to_owned() + &option + "'").purple());

    let help_message;

    match option.trim() {
        "feedback" => help_feedback(ctx).await,
        "fmeter" => help_fmeter(ctx).await,
        "frames" => help_frames(ctx).await,
        "general" => help_general(ctx).await,
        "hitboxes" => help_hitboxes(ctx).await,
        "info" => help_info(ctx).await,
        "moves" => help_moves(ctx).await,
        "nicknames" => help_nicknames(ctx).await,
        "notes" => help_notes(ctx).await,
        "register" => help_register(ctx).await,
        "update" => help_update(ctx).await,
        _ => {
            help_message = "Help for `".to_owned().to_string() + &option + "` not found!";
            ctx.say(&help_message).await?;
            println!("{}", ("Error: ".to_owned() + &help_message).red());

            return Ok(());
        }
    }

    Ok(())
}

async fn help_general(ctx: Context<'_>) {
    let help_msg = r#"
## __**List of commands**__
1. `frames`
1. `hitboxes`
1. `fmeter`
1. `moves`
1. `info`
1. `nicknames`
1. `feedback`
1. `help`

**[__Patch notes__](https://github.com/yakiimoninja/baiken/releases>)**
**[__Support the project__](<https://github.com/sponsors/yakiimoninja>)**
"#;

    let _ = ctx.say(help_msg).await;
}

async fn help_feedback(ctx: Context<'_>) {
    let help_msg = r#"
## __**Command**__: `/feedback`

__**text**__: Any text.
(Field cannot be empty).

Send feedback or requests to the dev."#;
    
    let _ = ctx.say(help_msg).await;
    let _ = ctx.channel_id().say(ctx, "https://raw.githubusercontent.com/yakiimoninja/baiken/main/data/images/feedback.png").await;
}

async fn help_fmeter(ctx: Context<'_>) {
    let help_msg = r#"
## __**Command**__: `/fmeter`

__**character**__: Character name or nickname.
(Field cannot be empty).

__**character_move**__: Character move name, input or alias.
(Field cannot be empty).

Display visually, a move's startup, active and recovery frames."#;
    
    let _ = ctx.say(help_msg).await;
    let _ = ctx.channel_id().say(ctx, "https://raw.githubusercontent.com/yakiimoninja/baiken/main/data/images/fmeter.png").await;
}

async fn help_frames(ctx: Context<'_>) {
    let help_msg = r#"
## __**Command**__: `/frames`

__**character**__: Character name or nickname.
(Field cannot be empty).

__**character_move**__: Character move name, input or alias.
(Field cannot be empty).

Display a move's frame data."#;
    
    let _ = ctx.say(help_msg).await;
    let _ = ctx.channel_id().say(ctx, "https://raw.githubusercontent.com/yakiimoninja/baiken/test/data/images/frames.png").await;
}

async fn help_hitboxes(ctx: Context<'_>) {
    let help_msg = r#"
## __**Command**__: `/hitboxes`

__**character**__: Character name or nickname.
(Field cannot be empty).

__**character_move**__: Character move name, input or alias.
(Field cannot be empty).

Display a move's hitbox images."#;
    
    let _ = ctx.say(help_msg).await;
    let _ = ctx.channel_id().say(ctx, "https://raw.githubusercontent.com/yakiimoninja/baiken/test/data/images/hitboxes.png").await;
}

async fn help_info(ctx: Context<'_>) {
    let help_msg = r#"
## __**Command**__: `/info`

__**character**__: Character name or nickname.
(Field cannot be empty).

Display a character's general information."#;

    let _ = ctx.say(help_msg).await;
    let _ = ctx.channel_id().say(ctx, "https://raw.githubusercontent.com/yakiimoninja/baiken/test/data/images/info.png").await;
}

async fn help_moves(ctx: Context<'_>) {
    let help_msg = r#"
## __**Command**__: `/moves`

__**character**__: Character name or nickname.
(Field cannot be empty).

Display a character's moves, inputs and move aliases."#;
    
    let _ = ctx.say(help_msg).await;
    let _ = ctx.channel_id().say(ctx, "https://raw.githubusercontent.com/yakiimoninja/baiken/test/data/images/moves.png").await;
}

async fn help_nicknames(ctx: Context<'_>) {
    let help_msg = r#"
## __Command__: `/nicknames`

Display all character nicknames."#;
    
    let _ = ctx.say(help_msg).await;
    let _ = ctx.channel_id().say(ctx, "https://raw.githubusercontent.com/yakiimoninja/baiken/test/data/images/nicknames.png").await;
}

async fn help_notes(ctx: Context<'_>) {
    let help_msg = r#"
## __Usage notes__

**•  All searching is case insensitive.**
    ○  All names, nicknames, moves and aliases are case agnostic.
            Example: `/hitboxes ky dp` = `/hitboxes KY dP`.

**•  Character searching.**
    ○  Characters can be found either using a part of their name, or any of their nicknames.
            Example: `/moves Happy Chaos` = `/moves happy` = `/moves hc`.

**•  Move searching.**
    ○  Moves can be found either using a part of their name, input, or any of their aliases.
            Example: `/frames Anji Needles` = `/frames Anji 236HP` = `/frames Anji ichi`.
    ○  Charged moves can be found with or without the use of `[]`.
            Example `/frames may 46S` = `/frames may [4]6S`.
    ○  All dots in move names are automatically ignored.
            Example: `/frames leo bts` = `/frames leo bt.S`.
    ○  For a fully charged dust attack the alias `5D!` can be used instead.
            Example: `/frames chipp 5D!`.

**•  Character specifics.**
    ○  For normals that have levels (e.g. Nagoriyuki).
            Add the level number next to the normal.
            For Level 1 `fS`: `/frames nago fs`.
            For Level 2 `fS`: `/frames nago fs2`.
            For Level 1 normals nothing needs to be added since it's the default state.

    ○  For specials that have levels (e.g. Goldlewis).
            Add the level number next to the special.
            For Level 1 `Thunderbird`: `/frames gold Drone`.
            For Level 2 `Thunderbird`: `/frames gold Drone 2`.
            The above is not always the case depending on the special move and alias used.
            For Level 1 `Thunderbird`: `/frames gold D1`.
            See `/moves gold` for more info on his aliases.

    ○  For Testament's different Grave Reaper versions.
            Regular version: `/frames testament 236S`.
            Partially charged version: `/frames testament 236S!`.
            Fully charged version: `/frames testament 236S!!`."#;

    let _ = ctx.say(help_msg).await;
}

async fn help_register(ctx: Context<'_>) {
    let help_msg = r#"
## __**Command**__: `/register`

Register or remove all slash commands in the current or every server the bot is present.
_**This command only works for owners.**_"#;
    
    let _ = ctx.say(help_msg).await;
    let _ = ctx.channel_id().say(ctx, "https://raw.githubusercontent.com/yakiimoninja/baiken/test/data/images/register.png").await;
}

async fn help_update(ctx: Context<'_>) {
    let help_msg = r#"
__**Command**__: `/update`

__**option**__: `frames`, `images` or `all`.
(Field cannot be empty).
__**character**__: Character name or nickname.
(Field cannot be empty).

Update the frame data and, image links or info for all or a specific character according to dustloop.
_**This command only works for owners.**_"#;

    let _ = ctx.say(help_msg).await;
    let _ = ctx.channel_id().say(ctx, "https://raw.githubusercontent.com/yakiimoninja/baiken/test/data/images/update.png").await;
}
