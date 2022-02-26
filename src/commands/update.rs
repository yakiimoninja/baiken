use serde_json;
use std::{fs, path::Path};

use serenity::framework::standard::{macros::command, CommandResult};
use serenity::model::prelude::*;
use serenity::prelude::*;

use crate::{CHARS, CharInfo};

mod char_json;
pub(crate) mod init_json;

const SITE_LINK: &str = "https://dustloop.com/wiki/api.php?action=parse&pageid=";
const SITE_HALF: &str = "&prop=text&formatversion=2";

#[command]
pub async fn update(ctx: &Context, msg: &Message) -> CommandResult {

    // Checking if 'data' folder exists
    if Path::new("data").exists() == true{

        // Checking if image folder and image txts exist
        if Path::new("data/images").exists() == true {
            for c in 0..CHARS.0.len(){
                if Path::new(&("data/images/".to_owned()+ CHARS.0[c]+".txt")).exists() == false {
                    // Error message cause a specific file is missing
                    let error_msg = "The `".to_owned() + &("data/images/".to_owned()+ CHARS.0[c] + ".txt") + "` file was not found.\nDownload and import the `data` folder from:\nhttps://github.com/yakiimoninja/baiken-bot.";
                    msg.channel_id.say(&ctx.http, &error_msg).await?;
                    print!("\n");
                    panic!("{}", error_msg.replace("`", "'"));
                }
            }
            println!("\nSuccesfully read {} '.txt' files from 'data/images' folder.", CHARS.0.len());
        }
        else{
            // Error message cause 'images' folder doesnt exist
            let error_msg= "The `data/images` folder was not found.\nDownload and import the `data` folder from:\nhttps://github.com/yakiimoninja/baiken-bot.";
            msg.channel_id.say(&ctx.http, error_msg).await?;
            print!("\n");
            panic!("{}", error_msg.replace("`", "'"));
        }

        // Creating the frames folder if it doesnt exist
        if Path::new("data/frames").exists() == false {
            fs::create_dir_all("data/frames")
                .expect("\nFailed to create 'data/frames' directory.");
        }

        // If init.json doesnt exist initialize it
        if Path::new("data/init.json").exists() == false {
            init_json::init_json();
        }

        // Checking if init.json exists
        if Path::new("data/init.json").exists() == true {

            // Reading init.json file
            let data_from_file = fs::read_to_string("data/init.json")
                .expect("\nFailed to read 'init.json' file.");

            match serde_json::from_str::<Vec<CharInfo>>(&data_from_file) {
                Ok(_) => (println!("\nSuccesfully deserialized 'init.json' file.")),
                Err(_) => (println!("\nFailed to deserialize 'init.json' file.\n\nConsider deleting 'init.json' from the 'data' folder.")),
            };

            // Deserializing from init.json
            let file = serde_json::from_str::<Vec<CharInfo>>(&data_from_file)
                .expect("\nFailed to deserialize from 'init.json' file.\nConsider deleting 'init.json' from the 'frame_data' folder.");

            char_json::make_char_json(CHARS, file).await;
            msg.channel_id.say(&ctx.http, "Update succesful!").await?;
        }

    }
    else {
        //Creating the 'data' folder
        fs::create_dir_all("data/images")
            .expect("\nFailed to create 'data' directory.");   
        
        // Error message cause 'images' folder is empty
        let error_msg = "The `data/images` folder was empty.\nDownload and import the `data` folder from:\nhttps://github.com/yakiimoninja/baiken-bot.";
        msg.channel_id.say(&ctx.http, error_msg).await?;
        print!("\n");
        panic!("{}", error_msg.replace("`", "'"));
    }
    
    return Ok(());
}
