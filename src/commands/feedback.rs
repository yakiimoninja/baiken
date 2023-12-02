use std::{fs::OpenOptions, io::Write};
use colored::Colorize;
use crate::{Context, Error};

/// Gives feedback or requests something from the dev.
#[poise::command(prefix_command, slash_command, aliases("r"))]
pub async fn feedback(
    ctx: Context<'_>,
    #[description = "Message for the dev."] text: String,
) -> Result<(), Error> {

    // Creating character json file
    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open("request.txt")
        .expect("\nFailed to open 'request.txt' file.");
    
    let new_text = text.to_owned() + "\n\n";

    write!(file, "{}", new_text)
        .expect("\nFailed to write to 'request.txt'");
    
    println!("{}", "Done writting to 'request.txt'".yellow());
    ctx.say("Submitted successfully!").await?;

    Ok(())
}