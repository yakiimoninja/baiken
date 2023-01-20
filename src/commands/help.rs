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
                "hitboxes" => help_message = help_hitboxes().await,
                "moves" => help_message = help_moves().await,
                "nicknames" => help_message = help_nicknames().await,
                "register" => help_message = help_register().await,
                "request" => help_message = help_request().await,
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
Displays all the aliases for each normal/special/super move of a character."#;
    return help_msg.to_string();
}

async fn help_fmeter() -> String {
    let help_msg = r#"
__**Command**__: `/fmeter`
__**Example**__: `/fmeter cz super`.
Displays visually the startup, active and recovery frames of a character's move."#;
    return help_msg.to_string();
}

async fn help_frames() -> String {
    let help_msg = r#"
__**Command**__: `/frames`.
__**Example**__: `/frames baiken 236K`.
Displays the frame data of a move along with an image."#;
    return help_msg.to_string();
}

async fn help_hitboxes() -> String {
    let help_msg = r#"
__**Command**__: `/hitboxes`. 
__**Example**__: `/hitboxes goldlewis 426H`.
Displays the hitbox images of a character's move."#;
    return help_msg.to_string();
}

async fn help_moves() -> String {
    let help_msg = r#"
__**Command**__: `/moves`.
__**Example**__: `/moves sol`.
Displays all the moves and inputs of a character."#;
    return help_msg.to_string();
}

async fn help_nicknames() -> String {
    let help_msg = r#"
__**Command**__: `/nicknames`.
Displays all the nicknames for each character."#;
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
Sends a request or feedback to the dev."#;
    return help_msg.to_string();
}

async fn help_update() -> String {
    let help_msg = r#"
__**Command**: `/update`.
__**Example**__: `/update frames all`.
**This command only works for owners.**
Meaning that it requires an instance of the source code to use it.
Updates the frame data and or image links for all or a specific character according to dustloop."#;
    return help_msg.to_string();
}
