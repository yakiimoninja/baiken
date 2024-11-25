use colored::Colorize;
use crate::{Context, Error, EMBED_COLOR};

#[derive(Debug, poise::ChoiceParameter)]
pub enum HelpChoice{
    #[name = "general"]
    General,
    #[name = "frames"]
    Frames,
    #[name = "hitboxes"]
    Hitboxes,
    #[name = "fmeter"]
    Fmeter,
    #[name = "moves"]
    Moves,
    #[name = "info"]
    Info,
    #[name = "nicknames"]
    Nicknames,
    #[name = "notes"]
    Notes,
    #[name = "register"]
    Register,
    #[name = "update"]
    Update,
    #[name = "report"]
    Report,
}
/// Display a help message.
#[poise::command(prefix_command, slash_command)]
pub async fn help(ctx: Context<'_>,
    #[description = "Pick a command to display help for."] option: HelpChoice
    ) -> Result<(), Error> {
    

    let (description_embed, image_embed) = match option {
        HelpChoice::Fmeter => {
            println!("{}", ("Command Args: 'fmeter'").purple());
            help_fmeter().await
            },
        HelpChoice::Frames => {
            println!("{}", ("Command Args: 'frames'").purple());
            help_frames().await
        },
        HelpChoice::General => {
            println!("{}", ("Command Args: 'general'").purple());
            help_general().await
        },
        HelpChoice::Hitboxes => {
            println!("{}", ("Command Args: 'hitboxes'").purple());
            help_hitboxes().await
        },
        HelpChoice::Info => {
            println!("{}", ("Command Args: 'info'").purple());
            help_info().await
        },
        HelpChoice::Moves => {
            println!("{}", ("Command Args: 'moves'").purple());
            help_moves().await
        },
        HelpChoice::Nicknames => {
            println!("{}", ("Command Args: 'nicknames'").purple());
            help_nicknames().await
        },
        HelpChoice::Notes => {
            println!("{}", ("Command Args: 'notes'").purple());
            help_notes().await
        },
        HelpChoice::Register => {
            println!("{}", ("Command Args: 'register'").purple());
            help_register().await
        },
        HelpChoice::Report => {
            println!("{}", ("Command Args: 'report'").purple());
            help_report().await
        },
        HelpChoice::Update => {
            println!("{}", ("Command Args: 'update'").purple());
            help_update().await
        },
    };

    // Sending the data as an embed
    let embed = poise::serenity_prelude::CreateEmbed::new()
        .color(EMBED_COLOR)
        .description(description_embed)
        //.title(&title_embed)
        .image(&image_embed);

    ctx.send(poise::CreateReply::default().embed(embed)).await?;

    Ok(())
}

async fn help_general() -> (String, String){
    let msg = String::from(r#"
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
"#);

    let img = String::new();

    (msg, img)
}

async fn help_fmeter() -> (String, String) {
    let msg = String::from(r#"
## __**Command**__: `/fmeter`

__**character**__: Character name or nickname.
__**move**__: Character move name, input or alias.

Display visually, a move's startup, active and recovery frames."#);
    
    let img = String::from("https://raw.githubusercontent.com/yakiimoninja/baiken/main/data/images/commands/fmeter.png");

    (msg, img)
}

async fn help_frames() -> (String, String){
    let msg = String::from(r#"
## __**Command**__: `/frames`

__**character**__: Character name or nickname.
__**move**__: Character move name, input or alias.

Display a move's frame data."#);
    
    let img = String::from("https://raw.githubusercontent.com/yakiimoninja/baiken/main/data/images/commands/frames.png");

    (msg, img)
}

async fn help_hitboxes() -> (String, String) {
    let msg = String::from(r#"
## __**Command**__: `/hitboxes`

__**character**__: Character name or nickname.
__**move**__: Character move name, input or alias.

Display a move's hitbox images."#);
    
    let img = String::from("https://raw.githubusercontent.com/yakiimoninja/baiken/main/data/images/commands/hitboxes.png");

    (msg, img)
}

async fn help_info() -> (String, String) {
    let msg = String::from(r#"
## __**Command**__: `/info`

__**character**__: Character name or nickname.

Display a character's general information."#);

    let img = String::from("https://raw.githubusercontent.com/yakiimoninja/baiken/main/data/images/commands/info.png");

    (msg, img)
}

async fn help_moves() -> (String, String) {
    let msg = String::from(r#"
## __**Command**__: `/moves`

__**character**__: Character name or nickname.
__**type**__: `all`, `normals`, `specials` or `supers`.

Display a character's moves, inputs and move aliases."#);
    
    let img = String::from("https://raw.githubusercontent.com/yakiimoninja/baiken/main/data/images/commands/moves.png");

    (msg, img)
}

async fn help_nicknames() -> (String, String) {
    let msg = String::from(r#"
## __Command__: `/nicknames`

Display all character nicknames."#);
    
    let img = String::from("https://raw.githubusercontent.com/yakiimoninja/baiken/main/data/images/commands/nicknames.png");

    (msg, img)
}

async fn help_notes() -> (String, String) {
    // Special whitespace character below
    // U+2000 &#8192 En Quad
    let msg = String::from(r#"
## __Usage notes__

**• All searching is case insensitive.**
    All names, nicknames, moves and aliases are case agnostic.
    Example: `/hitboxes ky dp` = `/hitboxes KY dP`.

**• Character searching.**
    Characters can be found either using a part of their name, or any of their nicknames.
    Example: `/moves Happy Chaos` = `/moves happy` = `/moves hc`.

**• Move searching.**
    Moves can be found either using a part of their name, input, or any of their aliases.
    Example: `/frames Anji Needles` = `/frames Anji 236HP` = `/frames Anji ichi`.
    Charged moves can be found with or without the use of `[]`.
    Example `/frames may 46S` = `/frames may [4]6S`.
    All dots in move names are automatically ignored.
    Example: `/frames leo bts` = `/frames leo bt.S`.
    For a fully charged dust attack the alias `5D!` can be used instead.
    Example: `/frames chipp 5D!`.

**• Character specifics.**
    For normals that have levels (e.g. Nagoriyuki).
        Add the level number next to the normal.
        For Level 1 `fS`: `/frames nago fs`.
        For Level 2 `fS`: `/frames nago fs2`.
        For Level 1 normals nothing needs to be added since it's the default state.

    For specials that have levels (e.g. Goldlewis).
        Add the level number next to the special.
        For Level 1 `Thunderbird`: `/frames gold Drone`.
        For Level 2 `Thunderbird`: `/frames gold Drone 2`.
        The above is not always the case depending on the special move and alias used.
        For Level 1 `Thunderbird`: `/frames gold D1`.
        See `/moves gold` for more info on his aliases.

    For Testament's different Grave Reaper versions.
        Regular version: `/frames testament 236S`.
        Partially charged version: `/frames testament 236S!`.
        Fully charged version: `/frames testament 236S!!`."#);

    let img = String::new();

    (msg, img)
}

async fn help_register() -> (String, String) {
    let msg = String::from(r#"
## __**Command**__: `/register`

Register or remove all slash commands in the current or every server the bot is present.
_**This command only works for owners.**_"#);
    
    let img = String::from("https://raw.githubusercontent.com/yakiimoninja/baiken/main/data/images/commands/register.png");

    (msg, img)
}

async fn help_report() -> (String, String) {
    let msg = String::from(r#"
## __**Command**__: `/report`

__**subject**__: `feature`, `bug` or `other`.
__**text**__: Text to be sent.

Send a report or feedback."#);
    
    let img = String::from("https://raw.githubusercontent.com/yakiimoninja/baiken/main/data/images/commands/feedback.png");

    (msg, img)
}

async fn help_update() -> (String, String) {
    let msg = String::from(r#"
__**Command**__: `/update`

__**option**__: `all`, `frames`, `images` or `info`.
__**character**__: Character name or nickname.

Update the frame data and, image links or info for all or a specific character according to dustloop.
_**This command only works for owners.**_"#);

    let img = String::from("https://raw.githubusercontent.com/yakiimoninja/baiken/main/data/images/commands/update.png");

    (msg, img)
}
