use crate::{check, Context, Error};
use colored::Colorize;
use rusqlite::{named_params, Connection as SqlConnection, OpenFlags};


#[derive(Debug, poise::ChoiceParameter)]
pub enum SettingChoice{
    #[name = "easter eggs"]
    EasterEggs,
    #[name = "ephemeral replies"]
    EphemeralReplies
}

#[derive(Debug, poise::ChoiceParameter)]
pub enum ToggleChoice{
    #[name = "disable"]
    Disable,
    #[name = "enable"]
    Enable
}

/// Settings configuration for current server. Admin only.
#[poise::command(
    slash_command,
    ephemeral,
    hide_in_help,
    required_permissions = "ADMINISTRATOR",
    default_member_permissions = "ADMINISTRATOR"
)]
pub async fn settings (
    ctx: Context<'_>,
    #[description = "Pick a setting to configure."] setting: SettingChoice,
    #[description = "Enable or disable setting."] toggle: ToggleChoice,
) -> Result<(), Error> {

    // Check to see if users guild id exists
    if ctx.guild_id().is_none() {
        println!("{}", "This command is for servers only.".red());
        ctx.say("This command is for servers only.").await?;
        return Ok(());
    }

    if (check::adaptive_check(ctx, false, true).await).is_err() {
        return Ok(());
    }

    // Parse user guild id to string
    let guild_id = ctx.guild_id().unwrap().to_string();

    // Open gset.db and check if gid is in db
    let db = SqlConnection::open_with_flags("data/gset.db", OpenFlags::SQLITE_OPEN_READ_WRITE).unwrap();
    let guild_id_exists = check::gid_exists(&guild_id).await;

    match setting {
        SettingChoice::EasterEggs => {
            match toggle {
                ToggleChoice::Disable => {
                    // Gid already exists
                    if guild_id_exists {
                        println!("{}", "Easter eggs are already disabled.".purple());
                        ctx.say("Easter eggs for this server are already disabled.").await?;
                        return Ok(());
                    }

                    // Toggling disable
                    db.execute("INSERT INTO settings (gid, easter_eggs) VALUES (:gid, :disable)", named_params! {":gid": guild_id, ":disable": 0}).unwrap();
                    println!("{}", "Easter eggs have been disabled.".purple());
                    ctx.say("Easter eggs for this server have been disabled.").await?;
                }
                ToggleChoice::Enable => {
                    // Gid already exists
                    if guild_id_exists {
                        println!("{}", "Easter eggs are already enabled".purple());
                        ctx.say("Easter eggs for this server are already enabled.").await?;
                        return Ok(());
                    }

                    // Toggling enable
                    db.execute("UPDATE settings set easter_eggs = :enable WHERE gid = :gid", named_params! {":gid": guild_id, ":enable": 1}).unwrap();
                    println!("{}", "Easter eggs have been enabled.".purple());
                    ctx.say("Easter eggs for this server have been enabled.").await?;
                }
            }
        }
        SettingChoice::EphemeralReplies => {
            match toggle {
                ToggleChoice::Disable => {
                    // Gid already exists
                    if guild_id_exists {
                        println!("{}", "Ephemeral replies are already disabled.".purple());
                        ctx.say("Ephemeral replies for this server are already disabled.").await?;
                        return Ok(());
                    }

                    // Toggling disable
                    db.execute("INSERT INTO settings (gid, ephemeral_replies) VALUES (:gid, :disable)", named_params! {":gid": guild_id, ":disable": 0}).unwrap();
                    println!("{}", "Ephemeral replies have been disabled.".purple());
                    ctx.say("Ephemeral replies for this server have been disabled.").await?;
                }
                ToggleChoice::Enable => {
                    // Gid already exists
                    if guild_id_exists {
                        println!("{}", "Ephemeral replies are already enabled".purple());
                        ctx.say("Ephemeral replies for this server are already enabled.").await?;
                        return Ok(());
                    }

                    // Toggling enable
                    db.execute("UPDATE settings set ephemeral_replies = :enable WHERE gid = :gid", named_params! {":gid": guild_id, ":enable": 1}).unwrap();
                    println!("{}", "Ephemeral replies have been enabled.".purple());
                    ctx.say("Ephemeral replies for this server have been enabled.").await?;
                }
            }
        }
    }

    Ok(())
}
