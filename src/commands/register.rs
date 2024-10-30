use crate::{Context, Error};

/// Register or remove commands in this guild or globally. Owners only.
#[poise::command(prefix_command, slash_command, hide_in_help, owners_only, ephemeral)]
pub async fn register(ctx: Context<'_>) -> Result<(), Error> {
    poise::builtins::register_application_commands_buttons(ctx).await?;

    Ok(())
}
