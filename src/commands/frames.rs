mod meter;
mod simple;
mod short;
mod advanced;
mod utils;
use meter::meter;
use simple::simple;
use short::short;
use advanced::advanced;
use std::string::String;
use crate::{Context, Error};

/// Display a move's frame data.
#[poise::command(
    prefix_command,
    slash_command,
    subcommands("simple", "short", "advanced", "meter"),
    subcommand_required
)]
pub async fn frames(_: Context<'_>) -> Result<(), Error> {
    Ok(())
}
