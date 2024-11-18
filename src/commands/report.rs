use colored::Colorize;
use std::{
    fs::OpenOptions,
    io::Write,
};
use crate::{Context, Error};

#[derive(Debug, poise::ChoiceParameter)]
pub enum ReportChoice{
    Feature,
    Bug,
    Other,
}

/// Send a report or feedback.
#[poise::command(slash_command, ephemeral)]
pub async fn report(
    ctx: Context<'_>,
    #[description = "Pick a subject."] subject: ReportChoice,
    #[min_length = 10]
    #[description = "Text to be sent."] text: String,
) -> Result<(), Error> {

    let subject =  match subject {
        ReportChoice::Feature => String::from("Feature"),
        ReportChoice::Bug => String::from("Bug"),
        ReportChoice::Other => String::from("Other"),
    };

    // Creating character json file
    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open("report.txt")
        .expect("\nFailed to open 'report.txt' file.");
    
    let new_text = "".to_owned()
        + "\n[Subject: " + &subject
        + "]\n" + &text + "\n\n";

    write!(file, "{}", &new_text)
        .expect("\nFailed to write to 'report.txt'");
    
    println!("{}", "Done writting to 'report.txt'".yellow());
    ctx.say("Submitted successfully!").await?;

    Ok(())
}
