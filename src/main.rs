mod commands;
mod check;
mod find;
mod ran;

use colored::Colorize;
use commands::*;
use poise::serenity_prelude as serenity;
use std::time::Duration;
use serde::{Serialize, Deserialize};
use tokio::{task, time};

// Types used by all command functions
type Error = Box<dyn std::error::Error + Send + Sync>;
type Context<'a> = poise::Context<'a, Data, Error>;

// Custom user data passed to all command functions
pub struct Data {}

#[derive(Serialize, Deserialize, Debug)]
pub struct CharInfo {
    defense: String,
    guts: String,
    guard_balance: String,
    prejump: String,
    umo: String,
    forward_dash: String,
    backdash: String,
    backdash_duration: String,
    backdash_invincibility: String,
    backdash_airborne: String,
    backdash_distance: String,
    jump_duration: String,
    jump_height: String,
    high_jump_duration: String,
    high_jump_height: String,
    earliest_iad: String,
    ad_duration: String,
    ad_distance: String,
    abd_duration: String,
    abd_distance: String,
    movement_tension: String,
    jump_tension: String,
    airdash_tension: String,
    walk_speed: String,
    back_walk_speed: String,
    dash_initial_speed: String,
    dash_acceleration: String,
    dash_friction: String,
    jump_gravity: String,
    high_jump_gravity: String,
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
    on_hit: String,
    on_block: String,
    level: String,
    counter: String,
    move_type: String,
    risc_gain: String,
    risc_loss: String,
    wall_damage: String,
    input_tension: String,
    chip_ratio: String,
    otg_ratio: String,
    scaling: String,
    invincibility: String,
    cancel: String,
    caption: String,
    notes: String,
    //images: String,
    //hitboxes: String,
    //hitbox_caption: String,
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

#[derive(Serialize, Deserialize, Debug)]
struct Gids {
    id: Vec<String>
}

pub const CHARS: [&str; 29] = [
    "A.B.A",
    "Anji_Mito",
    "Asuka_R",
    "Axl_Low",
    "Baiken",
    "Bedman",
    "Bridget",
    "Chipp_Zanuff",
    "Elphelt_Valentine",
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
    "Queen_Dizzy",
    "Ramlethal_Valentine",
    "Sin_Kiske",
    "Slayer",
    "Sol_Badguy",
    "Testament",
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
    check::data_folder_exists().await.expect("data_folder_exists");
    check::character_folders_exist().await.expect("character_folders_exist");
    check::character_jsons_exist().await.expect("character_jsons_exist");
    check::character_images_exist().await.expect("character_images_exist");
    check::character_info_exist().await.expect("character_info_exist");
    check::nicknames_json_exists().await.expect("nicknames_json_exists");
    check::gids_json_exists().await.expect("gids_json_exists");

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
                Ok(Data {})
            })
        })
        .options(options)
        .build();

    dotenv::dotenv().expect("Failed to load .env file.");
    let token = std::env::var("DISCORD_TOKEN").expect("Failed to load `DISCORD_TOKEN` env var.");
    let intents = serenity::GatewayIntents::non_privileged() /*| serenity::GatewayIntents::MESSAGE_CONTENT */;
    let client = serenity::ClientBuilder::new(token, intents).framework(framework).await;

    client.unwrap().start().await.unwrap()
}
