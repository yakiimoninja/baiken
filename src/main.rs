mod commands;
mod check;
mod find;
mod ran;
use colored::Colorize;
use commands::*;
use poise::serenity_prelude as serenity;
use serde::{Serialize, Deserialize};

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
pub struct MoveInfo {
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

pub const CHARS: [&str; 24] = [
    "Anji_Mito",
    "Axl_Low",
    "Baiken",
    "Bedman",
    "Bridget",
    "Chipp_Zanuff",
    "Faust",
    "Giovanna",
    "Goldlewis_Dickinson",
    "Happy_Chaos",
    "I-No",
    "Jack-O",
    "Johnny",
    "Ky_Kiske",
    "Leo_Whitefang",
    "May",
    "Millia_Rage",
    "Nagoriyuki",
    "Potemkin",
    "Ramlethal_Valentine",
    "Sin_Kiske",
    "Sol_Badguy",
    "Testament",
    "Zato-1"
];

const IMAGE_DEFAULT: &str = "https://raw.githubusercontent.com/yakiimoninja/baiken/main/data/images/no_image.png";
const HITBOX_DEFAULT: &str = "https://raw.githubusercontent.com/yakiimoninja/baiken/main/data/images/no_hitbox.png";

async fn on_error(error: poise::FrameworkError<'_, Data, Error>) {
    // This is our custom error handler
    // They are many errors that can occur, so we only handle the ones we want to customize
    // and forward the rest to the default handler
    match error {
        poise::FrameworkError::Setup { error, .. } => panic!("{}", ("Failed to start bot: ".to_owned() + &error.to_string() + ".").red()),
        poise::FrameworkError::Command { error, ctx } => {
            println!("{}", ("Error in command `".to_owned() + &ctx.command().name + "`: " + &error.to_string() + ".").red());
        }
        error => {
            if let Err(e) = poise::builtins::on_error(error).await {
                println!("{}", ("Error while handling error: ".to_owned() + &e.to_string() + ".").red())
            }
        }
    }
}

#[tokio::main]
async fn main() {

    // Running initial checks
    println!();
    check::data_folder_exists(true).await;
    check::nicknames_json_exists(true).await;
    check::character_folders_exist(true).await;
    check::character_jsons_exist(true).await;
    check::character_images_exist(true).await;

    let options = poise::FrameworkOptions {
        commands: vec![
            feedback::feedback(),
            fmeter::fmeter(),
            frames::frames(),
            help::help(),
            hitboxes::hitboxes(),
            nicknames::nicknames(),
            moves::moves(),
            register::register(),
            update::update(),        
        ],
        // prefix_options: poise::PrefixFrameworkOptions {
        //     //prefix: Some("!".into()),
        //     edit_tracker: Some(poise::EditTracker::for_timespan(Duration::from_secs(3600))),
        //     //additional_prefixes: vec![
        //     //    poise::Prefix::Literal("b."),
        //     //],
        //     ..Default::default()
        // },
        
        // The global error handler for all error cases that may occur
        on_error: |error| Box::pin(on_error(error)),
        
        // This code is run before every command
        pre_command: |ctx| {
            Box::pin(async move {
                println!("{}", ("\nExecuting command ".to_owned() + &ctx.command().qualified_name + "...").cyan());
            })
        },

        // This code is run after a command if it was successful (returned Ok)
        post_command: |ctx| {
            Box::pin(async move {
                println!("{}", ("Executed command ".to_owned() + &ctx.command().qualified_name + "!").cyan());
            })
        },

        // Every command invocation must pass this check to continue execution
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
        .setup(move |_ctx, _ready, _framework| {
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
