use crate::{check, Context, Error};
use colored::Colorize;
use rusqlite::{named_params, Connection as SqlConnection, OpenFlags};

/// Disable or enable easter eggs for current server. Admin only.
#[poise::command(
    slash_command,
    ephemeral,
    hide_in_help,
    required_permissions = "ADMINISTRATOR",
    default_member_permissions = "ADMINISTRATOR"
)]
pub async fn xx(
    ctx: Context<'_>,
    #[description = "Disable or enable easter eggs."]
    #[choices("disable", "enable")]
    option: &'static str,
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
    // Open gids.db
    let db = SqlConnection::open_with_flags("data/gids.db", OpenFlags::SQLITE_OPEN_READ_WRITE).unwrap();
    // Check if gid is in db
    let guild_id_exists = db.prepare("SELECT 0 FROM gids WHERE gid = :gid").unwrap()
        .exists(named_params! {":gid": guild_id}).unwrap();

    if option == "disable" {
        // Gid already exists
        if guild_id_exists {
            println!("{}", "Easter eggs are already disabled.".purple());
            ctx.say("Easter eggs for this server are already disabled.").await?;
            return Ok(());
        }

        // Adding guild id to exclusion list
        db.execute("INSERT INTO gids (gid) VALUES (:gid)", named_params! {":gid": guild_id}).unwrap();
        println!("{}", "Easter eggs have been disabled.".purple());
        ctx.say("Easter eggs for this server have been disabled.").await?;
    }
    else if option == "enable" {
        // Gid already exists
        if guild_id_exists {
            // Removing guild id from exclusion list
            
            db.execute("DELETE FROM gids WHERE gid = :gid", named_params! {":gid": guild_id}).unwrap();

            println!("{}", "Easter eggs have been enabled".purple());
            ctx.say("Easter eggs for this server have been enabled.").await?;

            return Ok(());
        }

        println!("{}", "Easter eggs are already enabled.".purple());
        ctx.say("Easter eggs for this server are already enabled.").await?;
        return Ok(());
    }
    else {
        println!("{}", "Mega weird interaction with xx.".red());
        return Ok(());
    }

    Ok(())
}
