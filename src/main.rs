mod commands;
mod create;
mod check;
mod find;
mod ran;
mod structs;

use colored::Colorize;
use commands::*;
use poise::serenity_prelude as serenity;
use std::{io::Write, sync::{Arc, Mutex}, time::{Duration, Instant}};
use tokio::{task, time};
use rusqlite::{Connection as SqlConnection, OpenFlags};

// Types used by all command functions
type Error = Box<dyn std::error::Error + Send + Sync>;
type Context<'a> = poise::Context<'a, Data, Error>;

// Custom user data passed to all command functions
pub struct Data {
    db: Arc<Mutex<SqlConnection>>
}

pub const CHARS: [&str; 30] = [
    "A.B.A",
    "Anji Mito",
    "Asuka R",
    "Axl Low",
    "Baiken",
    "Bedman",
    "Bridget",
    "Chipp Zanuff",
    "Elphelt Valentine",
    "Faust",
    "Giovanna",
    "Goldlewis Dickinson",
    "Happy Chaos",
    "I-No",
    "Jack-O",
    "Johnny",
    "Ky Kiske",
    "Leo Whitefang",
    "May",
    "Millia Rage",
    "Nagoriyuki",
    "Potemkin",
    "Queen Dizzy",
    "Ramlethal Valentine",
    "Sin Kiske",
    "Slayer",
    "Sol Badguy",
    "Testament",
    "Venom",
    "Zato-1"
];

const IMAGE_DEFAULT: &str = "https://raw.githubusercontent.com/yakiimoninja/baiken/main/data/images/no_image.png";
const HITBOX_DEFAULT: &str = "https://raw.githubusercontent.com/yakiimoninja/baiken/main/data/images/no_hitbox.png";
const EMBED_COLOR: (u8, u8, u8) = (140,75,64);

async fn on_error(error: poise::FrameworkError<'_, Data, Error>) {
    // This is our custom error handler
    // They are many errors that can occur, so we only handle the ones we want to customize
    // and forward the rest to the default handler
    match error {
        poise::FrameworkError::Setup { error, .. } => panic!("{}", ("Failed to start bot: ".to_owned() + &error.to_string() + ".").red()),
        poise::FrameworkError::Command { error, ctx, .. } => {
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
    if check::data_db_exists().await.is_err() {
        create::create_db().await.unwrap();
    }
    if check::gids_db_exists().await.is_err() {
        create::create_gid_db().await.unwrap();
    }

    // FrameworkOptions contains all of poise's configuration option in one struct
    // Every option can be omitted to use its default value
    let options = poise::FrameworkOptions {
        commands: vec![
            frames::frames(),
            help::help(),
            hitboxes::hitboxes(),
            info::info(),
            nicknames::nicknames(),
            moves::moves(),
            register::register(),
            report::report(),
            stats::stats(),
            update::update(),    
            xx::xx(),
        ],
        // The global error handler for all error cases that may occur
        on_error: |error| Box::pin(on_error(error)),
        // This code is run before every command
        pre_command: |ctx| {
            Box::pin(async move {
                println!("{}", ("\nExecuting command ".to_owned() + &ctx.command().qualified_name + ".").cyan());
                println!("{}", ctx.invocation_string().purple());
            })
        },
        // This code is run after a command if it was successful (returned Ok)
        post_command: |ctx| {
            Box::pin(async move {
                let elapsed_time = ctx.invocation_data::<Instant>().await.as_deref().unwrap().elapsed();
                print!("{}", ("Executed command ".to_owned() + &ctx.command().qualified_name + " in ").cyan()); 
                print!("{}", (elapsed_time.as_millis().to_string() + "ms").yellow()); 
                println!("{}", ".".cyan());
                std::io::stdout().flush().unwrap();
            })
        },
        // Every command invocation must pass this check to continue execution
        command_check: Some(|ctx| {
            Box::pin(async move {
                if ctx.author().id == 123456789 {
                    return Ok(false);
                }
                ctx.set_invocation_data(Instant::now()).await;
                Ok(true)
            })
        }),
        // Enforce command checks even for owners (enforced by default)
        // Set to true to bypass checks, which is useful for testing
        skip_checks_for_owners: false,
        // On ready event start the task of auto updating 
        // the character data every 24 hours
        event_handler: |_ctx, event, _framework, _data| {
            Box::pin(async move {
                match event {
                    serenity::FullEvent::Ready { data_about_bot: _ } => {
                        let forever = task::spawn(async {
                            let mut interval = time::interval(Duration::from_secs(86400));
                            loop {
                                // Runs update_all_char_data every 24h
                                interval.tick().await;
                                //update::update_all_char_data().await;
                            }
                        });
                        
                        let _ = forever.await;
                        Ok(())
                    }
                    _ => Ok(())
                }
            })
        },
        ..Default::default()
    };

    let framework = poise::Framework::builder()
        .setup(move |ctx, _ready, framework| {
            Box::pin(async move {
                poise::builtins::register_globally(ctx, &framework.options().commands).await?;
                Ok(Data {
                    db: Arc::new(Mutex::new(SqlConnection::open_with_flags("data/data.db", OpenFlags::SQLITE_OPEN_READ_ONLY).unwrap()))
                })
            })
        })
        .options(options)
        .build();

    dotenv::dotenv().expect("Failed to load .env file.");
    let token = std::env::var("DISCORD_TOKEN").expect("Failed to load `DISCORD_TOKEN` env var.");
    let intents = serenity::GatewayIntents::non_privileged() /*| serenity::GatewayIntents::MESSAGE_CONTENT */;
    let client = serenity::ClientBuilder::new(token, intents).framework(framework).await;

    println!("{}", "Baiken is running!".green());

    client.unwrap().start().await.unwrap()
}
