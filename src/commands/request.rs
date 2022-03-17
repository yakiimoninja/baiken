use std::fs::OpenOptions;
use std::io::Write;
use serenity::framework::standard::{macros::command, Args, CommandResult};
use serenity::model::prelude::*;
use serenity::prelude::*;

#[command]
#[aliases("r")]
async fn request(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {

    let character_arg = args.single::<String>()?;
    let character_move_arg = args.single::<String>()?;
    let character_alias_arg = args.rest().to_string();
    
    // Creating character json file
    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open("request.txt")
        .expect(&("\nFailed to open 'request.txt' file."));
    
    let string = character_arg.to_owned() +" " + &character_move_arg +" " + &character_alias_arg;

    write!(file, "{}", string)
        .expect(&("\nFailed to write to 'request.txt'"));
    
    println!("Done writting '{}' to 'request.txt'", string);
    msg.channel_id.say(&ctx.http, "Submitted succesfully!").await?;

    Ok(())
}