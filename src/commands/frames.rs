mod meter;
mod simple;
mod advanced;
use meter::meter;
use simple::simple;
use advanced::advanced;
use std::string::String;
use crate::{Context, Error};

/// Display a move's frame data.
#[poise::command(
    prefix_command,
    slash_command,
    subcommands("simple", "advanced", "meter"),
    subcommand_required
)]
pub async fn frames(_: Context<'_>) -> Result<(), Error> {
    Ok(())
}
