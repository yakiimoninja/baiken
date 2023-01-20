use crate::{Context, Error};

/// Prints a help message.
#[poise::command(prefix_command, slash_command, aliases("?"))]
pub async fn help(ctx: Context<'_>,
    #[description = "Pick a command to display help for or leave empty for generic message."] mut cmd_arg: Option<String>
    ) -> Result<(), Error> {

    let help_message;

    match &cmd_arg {
        Some(cmd) => {
            match cmd.trim() {
                "aliases" => help_message = help_aliases().await,
                "fmeter" => help_message = help_fmeter().await,
                "frames" => help_message = help_frames().await,
                "help" => help_message = help_help().await,
                "hitboxes" => help_message = help_hitboxes().await,
                "moves" => help_message = help_moves().await,
                "nicknames" => help_message = help_nicknames().await,
                "notes" => help_message = help_notes().await,
                "register" => help_message = help_register().await,
                "request" => help_message = help_request().await,
                "specifics" => help_message = help_specifics().await,
                "update" => help_message = help_update().await,
                _ => {
                    help_message = "Command `".to_owned().to_string() + &cmd + "` not found!";
                    println!("\nCommand: '{} {}'", ctx.command().qualified_name, &cmd_arg.expect("null"));
                    ctx.say(&help_message).await?;
                    panic!("{}", &help_message);
                }
            }
        },
        None => {
            help_message = help_default().await;
            cmd_arg = Some("empty".to_string());
        }
    }

    println!("\nCommand: '{} {}'", ctx.command().qualified_name, &cmd_arg.unwrap());
    ctx.say(help_message).await?;
    Ok(())
}

async fn help_default() -> String {
    let help_msg = r#"
__**Patch notes:**__
__<https://github.com/yakiimoninja/baiken/releases>__

__**Commands:**__
__<https://github.com/yakiimoninja/baiken#commands>__

__**Support the project:**__
__<https://github.com/sponsors/yakiimoninja>__

__**Artwork:**__
__<https://twitter.com/gogalking/status/1307199393607553024>__
"#;

    return help_msg.to_string();
}

async fn help_aliases() -> String {
    let help_msg = r#"
__**Command**__: `/aliases`.
__**Example**__: `/aliases leo`.

__**character_arg**__: Character name or nickname. Cannot be empty.

Displays all the aliases for each normal/special/super move of a character."#;
    return help_msg.to_string();
}

async fn help_fmeter() -> String {
    let help_msg = r#"
__**Command**__: `/fmeter`.
__**Example**__: `/fmeter cz super`.

__**character_arg**__: Character name or nickname. Cannot be empty.
__**character_move_arg**__: Character move name, input or alias. Cannot be empty.

Displays visually the startup, active and recovery frames of a character's move."#;
    return help_msg.to_string();
}

async fn help_frames() -> String {
    let help_msg = r#"
__**Command**__: `/frames`.
__**Example**__: `/frames baiken 236K`.

__**character_arg**__: Character name or nickname. Cannot be empty.
__**character_move_arg**__: Character move name, input or alias. Cannot be empty.

Displays the frame data of a move along with an image."#;
    return help_msg.to_string();
}

async fn help_help() -> String {
    let help_msg = r#"
__**Command**__: `/help`. 
__**Example**__: `/help` or `/help fmeter`.

__**cmd_arg**__: Any command name, `notes` or specific. Can be empty.

Displays a help message. If used in conjunction with a command name, `notes` or `specifics` a different message wil be displayed."#;
    return help_msg.to_string();
}

async fn help_hitboxes() -> String {
    let help_msg = r#"
__**Command**__: `/hitboxes`. 
__**Example**__: `/hitboxes goldlewis 426H`.

__**character_arg**__: Character name or nickname. Cannot be empty.
__**character_move_arg**__: Character move name, input or alias. Cannot be empty.

Displays the hitbox images of a character's move."#;
    return help_msg.to_string();
}

async fn help_moves() -> String {
    let help_msg = r#"
__**Command**__: `/moves`.
__**Example**__: `/moves sol`.

__**character_arg**__: Character name or nickname. Cannot be empty.

Displays all the moves and inputs of a character."#;
    return help_msg.to_string();
}

async fn help_nicknames() -> String {
    let help_msg = r#"
__**Command**__: `/nicknames`.

Displays all the nicknames for each character."#;
    return help_msg.to_string();
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
    return help_msg.to_string();
}

async fn help_register() -> String {
    let help_msg = r#"
__**Command**__: `/register`.

**This command only works for owners.**
Registers or removes all slash commands in the current server or every server the bot is in."#;
    return help_msg.to_string();
}

async fn help_request() -> String {
    let help_msg = r#"
__**Command**__: `/request`.

__**text**__: Any text. Cannot be empty.

Sends a request or feedback to the dev."#;
    return help_msg.to_string();
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
    return help_msg.to_string();
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
    return help_msg.to_string();
}
