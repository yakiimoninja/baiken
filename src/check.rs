use std::path::Path;
use colored::Colorize;
use poise::reply::CreateReply;
use crate::{Context, Error};
use rusqlite::{named_params, Connection as SqlConnection, OpenFlags};

// Collection of functions that check for stuff

/// Checks if `data` database exists.
pub async fn data_db_exists() -> Result<(), Error> {

    // Checking if data db exists
    if Path::new("data/data.db").exists() {
        Ok(())
    }
    else {
        // Error message cause data db does not exist
        let error_msg = "Failed to open 'data.db' database.";
        Err(error_msg.into())
    }
}

/// Checks if `gset` database exists.
pub async fn gset_db_exists() -> Result<(), Error> {

    // Checking if gset db exists
    if Path::new("data/gset.db").exists() {
        Ok(())
    }
    else {
        // Error message cause gset db does not exist
        let error_msg = "Failed to open 'gset.db' database.";
        Err(error_msg.into())
    }
}

/// Checks if `data` and or `gset` databases exist.
pub async fn adaptive_check(
    ctx: Context<'_>,
    check_for_db: bool,
    check_for_gset_db: bool,
) -> Result<(), Error> {
    
    if check_for_db {
        // Checking if data db exists
        if let Err(error_msg) = data_db_exists().await {
            ctx.send(CreateReply::default()
                .content(error_msg.to_string().replace('\'', "`"))
                .ephemeral(true))
            .await?;
            println!("{}", error_msg.to_string().replace('\n', " ").red());
            return Err(error_msg);
        }
    }
    if check_for_gset_db {
        // Checking if gset database exists
        if let Err(error_msg) = gset_db_exists().await {
            ctx.send(CreateReply::default()
                .content(error_msg.to_string().replace('\'', "`"))
                .ephemeral(true))
            .await?;
            println!("{}", error_msg.to_string().replace('\n', " ").red());
            return Err(error_msg);
        }
    }
    Ok(())
}

/// Checks if given guild id exists in database.
pub async fn guild_exists(guild_id: &String) -> bool {

    // Open gids.db
    let db = SqlConnection::open_with_flags("data/gset.db", OpenFlags::SQLITE_OPEN_READ_ONLY).unwrap();
    // Check if gid is in db
    let guild_exists = db.prepare("SELECT 0 FROM settings WHERE gid = :gid").unwrap()
        .exists(named_params! {":gid": guild_id}).unwrap();

    guild_exists
}

/// Checks if given guild has ee enabled.
pub async fn guild_ee_enabled(guild_id: &String) -> bool {

    // Open gset.db
    let db = SqlConnection::open_with_flags("data/gset.db", OpenFlags::SQLITE_OPEN_READ_ONLY).unwrap();
    // Check if ee are disabled
    let guild_ee = db.prepare("SELECT 0 FROM settings WHERE gid = :gid AND easter_eggs = :bool").unwrap()
        .exists(named_params! {":gid": guild_id, ":bool": 0}).unwrap();

    if !guild_ee {
        return true;
    }
    false
}

/// Checks if given guild has er enabled.
pub async fn guild_er_enabled(guild_id: &String) -> bool {

    // Open gset.db
    let db = SqlConnection::open_with_flags("data/gset.db", OpenFlags::SQLITE_OPEN_READ_ONLY).unwrap();
    // Check if er are enabled
    let guild_er = db.prepare("SELECT 0 FROM settings WHERE gid = :gid AND ephemeral_replies = :bool").unwrap()
        .exists(named_params! {":gid": guild_id, ":bool": 1}).unwrap();

    if !guild_er {
        return false;
    }
    true
}
