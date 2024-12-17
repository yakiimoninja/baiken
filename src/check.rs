use std::path::Path;
use colored::Colorize;
use poise::reply::CreateReply;
use crate::{Context, Error};

// Collection of functions that check for stuff

pub async fn data_db_exists() -> Result<(), Error> {

    // Checking if data db exists
    if Path::new("data/data.db").exists() {
        println!("{}", "Character database exists!".green());
        Ok(())
    }
    else {
        // Error message cause data db does not exist
        let error_msg = "Failed to open 'data.db' database.";
        Err(error_msg.into())
    }
}

pub async fn gids_db_exists() -> Result<(), Error> {

    // Checking if gids db exists
    if Path::new("data/gids.db").exists() {
        println!("{}", "Gids database exists!".green());
        Ok(())
    }
    else {
        // Error message cause gids db does not exist
        let error_msg = "Failed to open 'gids.db' database.";
        Err(error_msg.into())
    }
}

/// Runs checks depening on the arguments given
/// bool to see if a check::function to will execute
#[allow(clippy::too_many_arguments)]
pub async fn adaptive_check(
    ctx: Context<'_>,
    data_folder_check: bool,
    gids_json_check: bool,
) -> Result<(), Error> {
    
    if data_folder_check {
        // Checking if data folder exists
        if let Err(error_msg) = data_db_exists().await {
            ctx.send(CreateReply::default()
                .content(error_msg.to_string().replace('\'', "`"))
                .ephemeral(true))
            .await?;
            println!("{}", error_msg.to_string().replace('\n', " ").red());
            return Err(error_msg);
        }
    }
    if gids_json_check {
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
