use std::path::Path;
use colored::Colorize;
use poise::reply::CreateReply;
use crate::{Context, Error};

// Collection of functions that check for stuff

/// Returns `Ok()` if database exists.
/// Returns `Err()` if database doesn't exist.
///
/// Function checks if `data` database exists.
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

/// Returns `Ok()` if database exists.
/// Returns `Err()` if database doesn't exist.
///
/// Function checks if `gids` database exists.
pub async fn gids_db_exists() -> Result<(), Error> {

    // Checking if gids db exists
    if Path::new("data/gids.db").exists() {
        Ok(())
    }
    else {
        // Error message cause gids db does not exist
        let error_msg = "Failed to open 'gids.db' database.";
        Err(error_msg.into())
    }
}

/// Returns `Ok()` when successful.
/// Returns `Err()` when failed.
///
/// Function checks if `data` and `gids` databases exist.
pub async fn adaptive_check(
    ctx: Context<'_>,
    check_for_db: bool,
    check_for_gids_db: bool,
) -> Result<(), Error> {
    
    if check_for_db {
        // Checking if data directory exists
        if let Err(error_msg) = data_db_exists().await {
            ctx.send(CreateReply::default()
                .content(error_msg.to_string().replace('\'', "`"))
                .ephemeral(true))
            .await?;
            println!("{}", error_msg.to_string().replace('\n', " ").red());
            return Err(error_msg);
        }
    }
    if check_for_gids_db {
        // Checking if gids json exists
        if let Err(error_msg) = gids_db_exists().await {
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

