use colored::Colorize;
use std::{
    fs::OpenOptions,
    io::Write,
    time::Duration,
};
use poise::execute_modal;
use crate::{Context, Error};

#[derive(Debug, poise::Modal)]
#[name = "Report / Feedback"]
struct Submission{
    #[min_length = 3 ]
    #[max_length = 20 ]
    #[placeholder = r#"Select "feature", "bug" or "other"."#]
    subject: String,
    #[paragraph]
    #[min_length = 10 ]
    #[max_length = 200 ]
    #[placeholder = "Explain in detail."]
    text: String,
}

/// Send a report or feedback.
#[poise::command(prefix_command, slash_command, ephemeral)]
pub async fn report(
    ctx: Context<'_>,
) -> Result<(), Error> {

    let Context::Application(ctx) = ctx else {
        unreachable!()
    };

    // Write to file only if modal isnt empty
    if let Some(data) = execute_modal::<_,_, Submission>(ctx, None, Some(Duration::from_secs(60*2))).await? {
        // Creating character json file
        let mut file = OpenOptions::new()
            .create(true)
            .append(true)
            .open("report.txt")
            .expect("\nFailed to open 'report.txt' file.");
        
        let new_text = "".to_owned()
            + "\n[Subject: " + &data.subject
            + "]\n" + &data.text + "\n\n";

        write!(file, "{}", new_text)
            .expect("\nFailed to write to 'report.txt'");
        
        println!("{}", "Done writting to 'report.txt'".yellow());
        ctx.say("Submitted successfully!").await?;
        return Ok(());
    }
    else {
        println!("{}", "Report modal timed out.".red());
        ctx.say("Form timed out.").await?;
        return Ok(());
    }
}
