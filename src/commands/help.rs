use crate::{Context, Error};
use crate::serenity::futures::{Stream, StreamExt, self};

async fn autocomplete_help<'a>(
    _ctx: Context<'_>,
    partial: &'a str,
) -> impl Stream<Item = String> + 'a {
    futures::stream::iter(&[
        "aliases",
        "feedback",
        "fmeter",
        "frames",
        "general",
        "help",
        "hitboxes",
        "moves",
        "nicknames",
        "notes",
        "register",
        "specifics",
        "update"])
        .filter(move |name| futures::future::ready(name.to_lowercase().contains(&partial.to_lowercase())))
        .map(|name| name.to_string())
}

/// Prints a help message.
#[poise::command(prefix_command, slash_command, aliases("?"))]
pub async fn help(ctx: Context<'_>,
    #[description = "Pick a command to display help for."] 
    #[autocomplete = "autocomplete_help"] option: String
    ) -> Result<(), Error> {

    let help_message;

    match option.trim() {
                "aliases" => help_message = help_aliases().await,
                "feedback" => help_message = help_feedback().await,
                "fmeter" => help_message = help_fmeter().await,
                "frames" => help_message = help_frames().await,
                "general" => help_message = help_default().await,
                "help" => help_message = help_help().await,
                "hitboxes" => help_message = help_hitboxes().await,
                "moves" => help_message = help_moves().await,
                "nicknames" => help_message = help_nicknames().await,
                "notes" => help_message = help_notes().await,
                "register" => help_message = help_register().await,
                "specifics" => help_message = help_specifics().await,
                "update" => help_message = help_update().await,
                _ => {
                    help_message = "Help for `".to_owned().to_string() + &option + "` not found!";
                    println!("\nCommand: '{} {}'", ctx.command().qualified_name, &option);
                    ctx.say(&help_message).await?;
                    println!("\nError: {}", &help_message);
                    return Ok(());
                }
    }

    println!("\nCommand: '{} {}'", ctx.command().qualified_name, &option);
    ctx.say(help_message).await?;
    Ok(())
}

async fn help_default() -> String {
    let help_msg = r#"
__**List of commands**__
```frames``````
hitboxes``````
fmeter``````
aliases``````
moves``````
nicknames``````
request```

__**Patch notes:**__
__<https://github.com/yakiimoninja/baiken/releases>__

__**Commands:**__
__<https://github.com/yakiimoninja/baiken#commands>__

__**Support the project:**__
__<https://github.com/sponsors/yakiimoninja>__
"#;

    help_msg.to_string()
}

async fn help_aliases() -> String {
    let help_msg = r#"
__**Command**__: `/aliases`.
__**Example**__: `/aliases leo`.

__**character_arg**__: Character name or nickname. Cannot be empty.

Displays all the aliases for each normal/special/super move of a character."#;
    
    help_msg.to_string()
}

async fn help_fmeter() -> String {
    let help_msg = r#"
__**Command**__: `/fmeter`.
__**Example**__: `/fmeter cz super`.

__**character_arg**__: Character name or nickname. Cannot be empty.
__**character_move_arg**__: Character move name, input or alias. Cannot be empty.

Displays visually the startup, active and recovery frames of a character's move."#;
    
    help_msg.to_string()
}

async fn help_frames() -> String {
    let help_msg = r#"
__**Command**__: `/frames`.
__**Example**__: `/frames baiken 236K`.

__**character_arg**__: Character name or nickname. Cannot be empty.
__**character_move_arg**__: Character move name, input or alias. Cannot be empty.

Displays the frame data of a move along with an image."#;
    
    help_msg.to_string()
}

async fn help_help() -> String {
    let help_msg = r#"
__**Command**__: `/help`. 
__**Example**__: `/help` or `/help fmeter`.

__**option**__: Any command name, `notes` or specific. Can be empty.

Displays a help message. If used in conjunction with a command name, `notes` or `specifics` a different message wil be displayed."#;
    
    help_msg.to_string()
}

async fn help_hitboxes() -> String {
    let help_msg = r#"
__**Command**__: `/hitboxes`. 
__**Example**__: `/hitboxes goldlewis 426H`.

__**character_arg**__: Character name or nickname. Cannot be empty.
__**character_move_arg**__: Character move name, input or alias. Cannot be empty.

Displays the hitbox images of a character's move."#;
    
    help_msg.to_string()
}

async fn help_moves() -> String {
    let help_msg = r#"
__**Command**__: `/moves`.
__**Example**__: `/moves sol`.

__**character_arg**__: Character name or nickname. Cannot be empty.

Displays all the moves and inputs of a character."#;
    
    help_msg.to_string()
}

async fn help_nicknames() -> String {
    let help_msg = r#"
__**Command**__: `/nicknames`.

Displays all the nicknames for each character."#;
    
    help_msg.to_string()
}

async fn help_notes() -> String {
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
    
    help_msg.to_string()
}

async fn help_register() -> String {
    let help_msg = r#"
__**Command**__: `/register`.

**This command only works for owners.**
Registers or removes all slash commands in the current server or every server the bot is in."#;
    
    help_msg.to_string()
}

async fn help_feedback() -> String {
    let help_msg = r#"
__**Command**__: `/feedback`.

__**text**__: Any text. Cannot be empty.

Sends feedback or a request to the dev."#;
    
    help_msg.to_string()
}

async fn help_specifics() -> String {
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
    
    help_msg.to_string()
}

async fn help_update() -> String {
    let help_msg = r#"
__**Command**: `/update`.
__**Example**__: `/update frames all`.

__**frames_or_images**__: `frames`, `images` or `all`. Cannot be empty.
__**character_arg**__: Character name or nickname. Cannot be empty.

**This command only works for owners.**
Meaning that it requires an instance of the source code to use it.
Updates the frame data and or image links for all or a specific character according to dustloop."#;

    help_msg.to_string()
}
