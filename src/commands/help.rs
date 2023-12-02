use colored::Colorize;
use crate::{Context, Error};
use crate::serenity::futures::{Stream, StreamExt, self};

async fn autocomplete_help<'a>(
    _ctx: Context<'_>,
    partial: &'a str,
) -> impl Stream<Item = String> + 'a {
    futures::stream::iter(&[
        "general",
        "frames",
        "hitboxes",
        "fmeter",
        "aliases",
        "moves",
        "nicknames",
        "notes",
        "specifics",
        "register",
        "update",
        "feedback"])
        .filter(move |name| futures::future::ready(name.to_lowercase().contains(&partial.to_lowercase())))
        .map(|name| name.to_string())
}

/// Prints a help message.
#[poise::command(prefix_command, slash_command, aliases("?"))]
pub async fn help(ctx: Context<'_>,
    #[description = "Pick a command to display help for."] 
    #[autocomplete = "autocomplete_help"] option: String
    ) -> Result<(), Error> {
    
        println!("{}", ("Command Args: '".to_owned() + &option + "'").purple());

    let help_message;

    match option.trim() {
        "aliases" => help_aliases(ctx).await,
        "feedback" => help_feedback(ctx).await,
        "fmeter" => help_fmeter(ctx).await,
        "frames" => help_frames(ctx).await,
        "general" => help_general(ctx).await,
        "hitboxes" => help_hitboxes(ctx).await,
        "moves" => help_moves(ctx).await,
        "nicknames" => help_nicknames(ctx).await,
        "notes" => help_notes(ctx).await,
        "register" => help_register(ctx).await,
        "specifics" => help_specifics(ctx).await,
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
__**List of commands**__
```frames``````
hitboxes``````
fmeter``````
moves``````
nicknames``````
feedback``````
help```

__**Patch notes:**__
__<https://github.com/yakiimoninja/baiken/releases>__

__**Support the project:**__
__<https://github.com/sponsors/yakiimoninja>__
"#;

    let _ = ctx.say(help_msg).await;
}

async fn help_aliases(ctx: Context<'_>) {
    let help_msg = r#"
__**Command**__: `/aliases`.
__**Example**__: `/aliases leo`.

__**character_arg**__: Character name or nickname. Cannot be empty.

Displays all the aliases for each normal/special/super move of a character."#;
    
    let _ = ctx.say(help_msg).await;
    let _ = ctx.channel_id().say(ctx, "https://raw.githubusercontent.com/yakiimoninja/baiken/main/data/images/aliases.png").await;
}

async fn help_feedback(ctx: Context<'_>) {
    let help_msg = r#"
__**Command**__: `/feedback`.

__**text**__: Any text. Cannot be empty.

Sends feedback or a request to the dev."#;
    
    let _ = ctx.say(help_msg).await;
    let _ = ctx.channel_id().say(ctx, "https://raw.githubusercontent.com/yakiimoninja/baiken/main/data/images/feedback.png").await;
}

async fn help_fmeter(ctx: Context<'_>) {
    let help_msg = r#"
__**Command**__: `/fmeter`.
__**Example**__: `/fmeter cz super`.

__**character_arg**__: Character name or nickname. Cannot be empty.
__**character_move_arg**__: Character move name, input or alias. Cannot be empty.

Displays visually the startup, active and recovery frames of a character's move."#;
    
    let _ = ctx.say(help_msg).await;
    let _ = ctx.channel_id().say(ctx, "https://raw.githubusercontent.com/yakiimoninja/baiken/main/data/images/fmeter.png").await;
}

async fn help_frames(ctx: Context<'_>) {
    let help_msg = r#"
__**Command**__: `/frames`.
__**Example**__: `/frames baiken 236K`.

__**character_arg**__: Character name or nickname. Cannot be empty.
__**character_move_arg**__: Character move name, input or alias. Cannot be empty.

Displays the frame data of a move along with an image."#;
    
    let _ = ctx.say(help_msg).await;
    let _ = ctx.channel_id().say(ctx, "https://raw.githubusercontent.com/yakiimoninja/baiken/test/data/images/frames.png").await;
}

async fn help_hitboxes(ctx: Context<'_>) {
    let help_msg = r#"
__**Command**__: `/hitboxes`. 
__**Example**__: `/hitboxes goldlewis 426H`.

__**character_arg**__: Character name or nickname. Cannot be empty.
__**character_move_arg**__: Character move name, input or alias. Cannot be empty.

Displays the hitbox images of a character's move."#;
    
    let _ = ctx.say(help_msg).await;
    let _ = ctx.channel_id().say(ctx, "https://raw.githubusercontent.com/yakiimoninja/baiken/test/data/images/hitboxes.png").await;
}

async fn help_moves(ctx: Context<'_>) {
    let help_msg = r#"
__**Command**__: `/moves`.
__**Example**__: `/moves sol`.

__**character_arg**__: Character name or nickname. Cannot be empty.

Displays all the moves and inputs of a character."#;
    
    let _ = ctx.say(help_msg).await;
    let _ = ctx.channel_id().say(ctx, "https://raw.githubusercontent.com/yakiimoninja/baiken/test/data/images/moves.png").await;
}

async fn help_nicknames(ctx: Context<'_>) {
    let help_msg = r#"
__**Command**__: `/nicknames`.

Displays all the nicknames for each character."#;
    
    let _ = ctx.say(help_msg).await;
    let _ = ctx.channel_id().say(ctx, "https://raw.githubusercontent.com/yakiimoninja/baiken/test/data/images/nicknames.png").await;
}

async fn help_notes(ctx: Context<'_>) {
    let help_msg = r#"
__**Usage notes.**__

- **`/` commands can be substituted with direct mentions if preferred.**
    - Doing so will enable the use of shorthand commands.
        - Example: `@Baiken f sol 2k` same as `/frames sol 2k`.
        - Example: `@Baiken h ky 6p` same as `/hitboxes ky 6p`.
        - Example: `@Baiken m leo` same as `/moves leo`.
        - Example: `@Baiken a chipp` same as `/aliases chipp`.

- **All searching is case insensitive.**
    - All names, nicknames, moves and aliases are case agnostic.
    - Example: `/hitboxes ky dp` = `/hitboxes KY dP`.

- **Character searching.**
    - Characters can be found either using a part of their name, or any of their nicknames.
    - Example: `/frames Happy Chaos cs` = `/frames happy cs` = `/frames hc cs`.

- **Move searching.**
    - Moves can be found either using a part of their name, their input, or any of the aliases that exist.
        - Example: `/frames Anji Needles` = `/frames Anji 236HP` = `/frames Anji ichi`.
    - Charged moves can be found with or without the use of `[]`.
        - Example `/frames may 46S` = `/frames may [4]6S`.
    - For a fully charged dust attack the alias `5D!` can be used instead.
        - Example: `/frames chipp 5D!`."#;
    
    let _ = ctx.say(help_msg).await;
}

async fn help_register(ctx: Context<'_>) {
    let help_msg = r#"
__**Command**__: `/register`.

**This command only works for owners.**
Registers or removes all slash commands in the current server or every server the bot is in."#;
    
    let _ = ctx.say(help_msg).await;
    let _ = ctx.channel_id().say(ctx, "https://raw.githubusercontent.com/yakiimoninja/baiken/test/data/images/register.png").await;
}

async fn help_specifics(ctx: Context<'_>) {
    let help_msg = r#"
__**Character specifics.**__

- **For normals that have levels like Nagoriyuki.**
  - Add the level number next to the normal.
  - For Level 1 `fS`: `/frames nago fs`. 
  - For Level 2 `fS`: `/frames nago fs2`.
  - For Level 3 `fS`: `/frames nago fs3`.
  - If it's a level 1 normal nothing needs to be added since it's the default state.

- **For specials that have levels like Goldlewis.**
  - Add the level number next to the special.
  - For Level 1 `Thunderbird`: `/frames gold Drone`.
  - For Level 2 `Thunderbird`: `/frames gold Drone 2`.
  - For Level 3 `Thunderbird`: `/frames gold Drone 3`.
  - The above is not always the case depending on the special move and alias used.
  - For Level 1 `Thunderbird`: `/frames gold D1`.
  - For Level 2 `Thunderbird`: `/frames gold D2`.
  - For Level 3 `Thunderbird`: `/frames gold D3`.
  - See `/aliases gold` for more info on his aliases.

- **For Testament's different Grave Reaper versions use as shown.**
  - Regular version: `/frames testament 236S`.
  - Partially charged version: `/frames testament 236S!`.
  - Fully charged version: `/frames testament 236S!!`.
"#;
    
    let _ = ctx.say(help_msg).await;
}

async fn help_update(ctx: Context<'_>) {
    let help_msg = r#"
__**Command**: `/update`.
__**Example**__: `/update frames all`.

__**frames_or_images**__: `frames`, `images` or `all`. Cannot be empty.
__**character_arg**__: Character name or nickname. Cannot be empty.

**This command only works for owners.**
Meaning that it requires an instance of the source code to use it.
Updates the frame data and or image links for all or a specific character according to dustloop."#;

    let _ = ctx.say(help_msg).await;
    let _ = ctx.channel_id().say(ctx, "https://raw.githubusercontent.com/yakiimoninja/baiken/test/data/images/update.png").await;
}
