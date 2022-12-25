//#![warn(clippy::str_to_string)]

mod commands;
mod check;

use commands::*;
use poise::serenity_prelude as serenity;
use serde::{Serialize, Deserialize};
use std::time::Duration;

// Types used by all command functions
type Error = Box<dyn std::error::Error + Send + Sync>;
type Context<'a> = poise::Context<'a, Data, Error>;

// Custom user data passed to all command functions
pub struct Data {}

#[derive(Serialize, Deserialize, Debug)]
pub struct CharInfo {
    page: String
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Frames {
    input: String,
    name: String,
    damage: String,
    guard: String,
    startup: String,
    active: String,
    recovery: String,
    hit: String,
    block: String,
    level: String,
    counter: String,
    scaling: String,
    riscgain: String,
    invincibility: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ImageLinks {
    input: String,
    move_img: String,
    hitbox_img: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct MoveAliases {
    input: String,
    aliases: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Nicknames {
    character: String,
    nicknames: Vec<String>,
}

pub const CHARS: [&str; 22] = ["Anji_Mito","Axl_Low","Baiken","Bridget","Chipp_Zanuff","Faust","Giovanna","Goldlewis_Dickinson","Happy_Chaos","I-No","Jack-O","Ky_Kiske","Leo_Whitefang","May","Millia_Rage","Nagoriyuki","Potemkin","Ramlethal_Valentine","Sin_Kiske","Sol_Badguy","Testament","Zato-1"];


async fn on_error(error: poise::FrameworkError<'_, Data, Error>) {
    // This is our custom error handler
    // They are many errors that can occur, so we only handle the ones we want to customize
    // and forward the rest to the default handler
    match error {
        poise::FrameworkError::Setup { error, .. } => panic!("Failed to start bot: {:?}.", error),
        poise::FrameworkError::Command { error, ctx } => {
            println!("Error in command `{}`: {:?}.", ctx.command().name, error,);
        }
        error => {
            if let Err(e) = poise::builtins::on_error(error).await {
                println!("Error while handling error: {}.", e)
            }
        }
    }
}

#[tokio::main]
async fn main() {

    // Running initial checks
    check::data_folder_exists(true);
    check::character_folders_exist(true);
    check::character_images_exist(true);
    check::nicknames_json_exists(true);
    check::character_jsons_exist(true);

    let options = poise::FrameworkOptions {
        commands: vec![
            aliases::aliases(),
            easter::sake(),
            frames::frames(),
            help::help(),
            hitboxes::hitboxes(),
            nicknames::nicknames(),
            moves::moves(),
            register::register(),
            request::request(),
            update::update(),        
        ],
        prefix_options: poise::PrefixFrameworkOptions {
            //prefix: Some("!".into()),
            edit_tracker: Some(poise::EditTracker::for_timespan(Duration::from_secs(3600))),
            //additional_prefixes: vec![
            //    poise::Prefix::Literal("b."),
            //],
            ..Default::default()
        },
        
        /// The global error handler for all error cases that may occur
        on_error: |error| Box::pin(on_error(error)),
        
        // /// This code is run before every command
        // pre_command: |ctx| {
        //     Box::pin(async move {
        //         println!("\nExecuting command {}...", ctx.command().qualified_name);
        //     })
        // },

        // /// This code is run after a command if it was successful (returned Ok)
        // post_command: |ctx| {
        //     Box::pin(async move {
        //         println!("Executed command {}!", ctx.command().qualified_name);
        //     })
        // },

        /// Every command invocation must pass this check to continue execution
        command_check: Some(|ctx| {
            Box::pin(async move {
                if ctx.author().id == 123456789 {
                    return Ok(false);
                }
                Ok(true)
            })
        }),
        
        // // Uncomment for debugging
        // listener: |_ctx, event, _framework, _data| {
        //     Box::pin(async move {
        //         println!("Got an event in listener: {:?}", event.name());
        //         Ok(())
        //     })
        // },
        ..Default::default()
    };

    dotenv::dotenv().expect("Failed to load .env file.");

    poise::Framework::builder()
        .token(std::env::var("DISCORD_TOKEN").expect("missing DISCORD_TOKEN."))
        .user_data_setup(move |_ctx, _ready, _framework| {
            Box::pin(async move {
                Ok(Data {})
            })
        })
        .options(options)
        .intents(
            serenity::GatewayIntents::non_privileged() /*| serenity::GatewayIntents::MESSAGE_CONTENT*/,
        )
        .run()
        .await
        .unwrap();
}