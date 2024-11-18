use colored::Colorize;
use std::{
    fs::OpenOptions,
    io::Write,
};
use crate::{Context, Error};
 #[derive(Debug, poise::ChoiceParameter)]
pub enum MyStringChoice {
    #[name = "Feature"]
    A,
    #[name = "Bug"]
    B,
    #[name = "Other"]
    C,
}
/// Send a report or feedback.
#[poise::command(slash_command, ephemeral)]
pub async fn report(
    ctx: Context<'_>,
    #[description = "Pick a subject."] subject: MyStringChoice,
    #[min_length = 10]
    #[description = "Text to be sent."] text: String,
) -> Result<(), Error> {

    let subject =  match subject {
        MyStringChoice::A => String::from("Feature"),
        MyStringChoice::B => String::from("Bug"),
        MyStringChoice::C => String::from("Other"),
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
    return Ok(());
}
