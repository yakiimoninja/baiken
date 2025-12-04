mod framedata;
mod framedata_db;
mod images;
mod images_db;
mod info;
mod info_db;
use crate::{CHARS, Context, Error, check, find};
use serde::Deserialize;

#[derive(Deserialize, Debug)]
struct Response {
    query: Tokens
}

#[derive(Deserialize, Debug)]
struct Tokens {
    tokens: LoginToken
}

#[derive(Deserialize, Debug)]
struct LoginToken {
    logintoken: String,
}

#[derive(Debug, poise::ChoiceParameter)]
pub enum UpdateChoice{
    #[name = "all"]
    All,
    #[name = "frames"]
    Frames,
    #[name = "images"]
    Images,
    #[name = "info"]
    Info,
}

/// Update data according to dustloop. Owners only.
#[poise::command(prefix_command, slash_command, owners_only, ephemeral)]
pub async fn update (
    ctx: Context<'_>,
    #[min_length = 2]
    #[description = r#"Character name, nickname or "all"."#] character: String,
    #[description = r#"Select "frames", "info", "images" or "all"."#] option: UpdateChoice,
) -> Result<(), Error> {

    if (check::adaptive_check(ctx, true, false).await).is_err() {
        return Ok(());
    }

    // Dustloop bot login
    let agent = ureq::agent();
    let lg: String = agent.get("https://www.dustloop.com/wiki/api.php?action=query&format=json&meta=tokens&formatversion=2&type=login").call().unwrap().body_mut().read_json::<Response>().unwrap().query.tokens.logintoken;
    let username: String = std::env::var("DUSTLOOP_USERNAME").unwrap();
    let pass: String = std::env::var("DUSTLOOP_PASSWORD").unwrap();
    
    let body = [
        ("action", "login"),
        ("lgname", &username),
        ("lgpassword", &pass),
        ("lgtoken", &lg),
        ("format", "json"),
        ("formatversion", "2")
    ];

    let dustloop_api = "https://www.dustloop.com/wiki/api.php?";
    agent.post(dustloop_api).send_form(body).unwrap().into_body().read_to_string().unwrap();

    if character.trim().to_lowercase() == "all" {

        match option {
            UpdateChoice::All => {
                // If character arg is all; update frames, images and info for all characters
                ctx.say("Update started!").await?;
                update_all_char_data().await;
            }
            UpdateChoice::Frames => {
                // If character arg is all; update frames for all characters
                ctx.say("Update started!").await?; 
                framedata::get_char_data(CHARS, "all", &agent).await;
            }
            UpdateChoice::Images => {
                // If character arg is all; update images for all characters
                ctx.say("Update started!").await?; 
                images::get_char_images(CHARS, "all", &agent).await;
            }
            UpdateChoice::Info => {
                // If character arg is all; update info for all characters
                ctx.say("Update started!").await?; 
                info::get_char_info(CHARS, "all", &agent).await;
            }
        }
    }
    else {

        // Finding character
        let (character, _) = match find::find_character(&character, ctx.data().db.clone()).await {
            Ok(character) => character,
            Err(err) => {
                ctx.say(err.to_string()).await?;
                return Ok(()) }
        };

        match option {
            UpdateChoice::All => {
                // If user input isnt the full name, part of a full name or a nickname
                // Update frames, images and info for specific character
                ctx.say("Update started!").await?; 
                framedata::get_char_data(CHARS, &character, &agent).await;
                images::get_char_images(CHARS, &character, &agent).await;
                info::get_char_info(CHARS, &character, &agent).await;
            }
            UpdateChoice::Frames => {
                // Updates images for specific character
                // If user input isnt the full name, part of a full name or a nickname
                // Update frames for specific character
                ctx.say("Update started!").await?; 
                framedata::get_char_data(CHARS, &character, &agent).await;
            }
            UpdateChoice::Images => {
                // Updates images for specific character
                // If user input isnt the full name, part of a full name or a nickname
                // Update images for specific character
                ctx.say("Update started!").await?; 
                images::get_char_images(CHARS, &character, &agent).await;
            }
            UpdateChoice::Info => {
                // Updates info for specific character
                // If user input isnt the full name, part of a full name or a nickname
                // Update info for specific character
                ctx.say("Update started!").await?; 
                info::get_char_info(CHARS, &character, &agent).await;
            }
        }
    }
    ctx.say("Update succesful!").await?;

    Ok(())
}

pub async fn update_all_char_data(){

    // Dustloop bot login
    let agent = ureq::agent();
    let lg: String = agent.get("https://www.dustloop.com/wiki/api.php?action=query&format=json&meta=tokens&formatversion=2&type=login").call().unwrap().body_mut().read_json::<Response>().unwrap().query.tokens.logintoken;
    let username: String = std::env::var_os("DUSTLOOP_USERNAME").unwrap().into_string().unwrap();
    let pass: String = std::env::var_os("DUSTLOOP_PASSWORD").unwrap().into_string().unwrap();
    // let pass: String = std::env::var("DUSTLOOP_PASSWORD").unwrap();

    let body = [
        ("action", "login"),
        ("lgname", &username),
        ("lgpassword", &pass),
        ("lgtoken", &lg),
        ("format", "json"),
        ("formatversion", "2")
    ];

    let login_link = "https://www.dustloop.com/wiki/api.php?";
    let login = agent.post(login_link).send_form(body).unwrap().into_body().read_to_string().unwrap();
    println!("{}", login);

    // 24 hour character data auto update function
    framedata::get_char_data(CHARS, "all", &agent).await;
    images::get_char_images(CHARS, "all", &agent).await;
    info::get_char_info(CHARS, "all", &agent).await;
}
