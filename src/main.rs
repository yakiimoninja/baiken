use std::{//fs::File, io::Read,
    sync::Arc, env, collections::HashSet};

use serde::{Deserialize, Serialize};
use serenity::{async_trait,
    model::gateway::Ready,
    prelude::*,
    client::bridge::gateway::ShardManager,
    http::Http, framework::StandardFramework,
    framework::standard::macros::group};

mod commands;
mod check;

use commands::{frames::*,
    update::*, moves::*, 
    aliases::*, help::*, 
    hitboxes::*, request::*};

#[group]
#[commands(frames, update, moves, aliases, help, hitboxes, request)]
struct General;

#[derive(Serialize, Deserialize, Debug)]
pub struct CharInfo {
    page: String
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Frames {
    input: String,
    r#move: String,
    damage: String,
    guard: String,
    invincibility: String,
    startup: String,
    block: String,
    hit: String,
    active: String,
    recovery: String,
    counter: String,
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
struct ShardManagerContainer;

impl TypeMapKey for ShardManagerContainer{
    type Value = Arc<Mutex<ShardManager>>;
}

struct Handler;

#[async_trait]
impl EventHandler for Handler {

    async fn ready(&self, _: Context, ready: Ready) {
        println!("\n{} is connected!", ready.user.name);
        {
        // Running initial checks
        check::data_folder_exists(true);
        check::character_folders_exist(true);
        check::character_images_exist(true);
        check::init_json_exists(true);
        check::nicknames_json_exists(true);
        check::character_jsons_exist(true);
        }
    }
}

pub const CHARS: [&str; 21] = ["Anji_Mito","Axl_Low","Baiken","Bridget","Chipp_Zanuff","Faust","Giovanna","Goldlewis_Dickinson","Happy_Chaos","I-No","Jack-O","Ky_Kiske","Leo_Whitefang","May","Millia_Rage","Nagoriyuki","Potemkin","Ramlethal_Valentine","Sol_Badguy","Testament","Zato-1"];

#[tokio::main]
async fn main() {
   
    dotenv::dotenv().expect("Failed to load .env file");
    
    // Debuging
    //tracing_subscriber::fmt::init();

    let token = env::var("DISCORD_TOKEN").expect("Expected a token in the environment");

    let http = Http::new_with_token(&token);
    
    let (owners, _bot_id) = match http.get_current_application_info().await {
        Ok(info) => {
            let mut owners = HashSet::new();
            owners.insert(info.owner.id);

            (owners, info.id)
        },
        Err(why) => panic!("Could not access application info: {:?}", why),
    };

    let framework = StandardFramework::new().configure(|c|
        c.owners(owners)
        .prefixes(vec!["b.","!"])
        .case_insensitivity(true))
        .group(&GENERAL_GROUP);
        
    // Creating a new bot instance
    let mut client = Client::builder(&token)
        .framework(framework)
        .event_handler(Handler)
        .await
        .expect("\nError creating client");

    {
        let mut data = client.data.write().await;
        data.insert::<ShardManagerContainer>(client.shard_manager.clone());
    }

    let shard_manager = client.shard_manager.clone();

    tokio::spawn(async move {
        tokio::signal::ctrl_c().await.expect("Could not register ctrl+c handler");
        shard_manager.lock().await.shutdown_all().await;
    });

    // Starting the bot instance
    if let Err(why) = client.start().await {
        println!("\nClient error: {:?}", why);
    }
}
