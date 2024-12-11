extern crate ureq;
use std::time::Instant;
use rusqlite::Connection as SqlConnection;
use colored::Colorize;
use crate::{CHARS, commands::update::info_json::info_to_json};

const SITE_LINK : &str = "https://www.dustloop.com/wiki/api.php?action=cargoquery&format=json&tables=ggstCharacters&fields=ggstCharacters.defense%2C%20ggstCharacters.guts%2C%20ggstCharacters.guardBalance%2C%20ggstCharacters.prejump%2C%20ggstCharacters.umo%2C%20ggstCharacters.forwarddash%2C%20ggstCharacters.backdash%2C%20ggstCharacters.backdashDuration%2C%20ggstCharacters.backdashInvuln%2C%20ggstCharacters.backdashAirborne%2C%20ggstCharacters.backdashDistance%2C%20ggstCharacters.jump_duration%2C%20ggstCharacters.jump_height%2C%20ggstCharacters.high_jump_duration%2C%20ggstCharacters.high_jump_height%2C%20ggstCharacters.earliest_iad%2C%20ggstCharacters.ad_duration%2C%20ggstCharacters.ad_distance%2C%20ggstCharacters.abd_duration%2C%20ggstCharacters.abd_distance%2C%20ggstCharacters.movement_tension%2C%20ggstCharacters.jump_tension%2C%20ggstCharacters.airdash_tension%2C%20ggstCharacters.walk_speed%2C%20ggstCharacters.back_walk_speed%2C%20ggstCharacters.dash_initial_speed%2C%20ggstCharacters.dash_acceleration%2C%20ggstCharacters.dash_friction%2C%20ggstCharacters.jump_gravity%2C%20ggstCharacters.high_jump_gravity%2C&where=ggstCharacters.name%3D%22";
const SITE_HALF : &str = "%22";

pub async fn get_char_info(chars_ids: [&str; CHARS.len()], specific_char: &str) {
    // For timing the updates
    let now = Instant::now();
    let mut db = SqlConnection::open("data/data.db").unwrap();
    
    if specific_char == "all" {
    
        for (x, char_id) in chars_ids.iter().enumerate() {
    
            println!("{}", ("Updating ".to_owned() + char_id + " 'info' entry.").green());
            
            // Creating request link
            let character_info_link = SITE_LINK.to_owned() + &char_id.replace('_', " ") +  SITE_HALF;
    
            // Dusloop site request
            let mut char_info_response_json = ureq::get(&character_info_link)
                .call()
                .unwrap();
            
            // Because dustloop site 500 a lot
            while char_info_response_json.status() == 500 {
                char_info_response_json = ureq::get(&character_info_link)
                    .call()
                    .unwrap();
            }
    
            // Requested website source to file
            let char_info_response_json = char_info_response_json.into_string().unwrap();
            
            // Sending response to get processed and serialized to a json file
            // char_count is a counter to specify which json file fails to update
            db = info_to_json(&char_info_response_json, db, x).await;
        }
    }
    else {

        println!("{}", ("Updating ".to_owned() + specific_char + " 'info' entry.").green());

        // Creating request link
        let char_info_link = SITE_LINK.to_owned() + &specific_char.replace('_', " ") + SITE_HALF;

        // Dusloop site request
        let mut char_info_response_json = ureq::get(&char_info_link)
            .call()
            .unwrap();
        
        // Because dustloop site 500 a lot
        while char_info_response_json.status() == 500 {
            char_info_response_json = ureq::get(&char_info_link)
                .call()
                .unwrap();
        }

        // Requested website source to file
        let char_info_response_json = char_info_response_json.into_string().unwrap();
        
        for (x, char_id) in chars_ids.iter().enumerate() {
            if *char_id == specific_char {
                // Sending response to get processed and serialized to a json file
                // char_count is a counter to specify which json file fails to update
                db = info_to_json(&char_info_response_json, db, x).await;
                break;
            }
        }
    }
    
    db.close().unwrap();
    let elapsed_time = now.elapsed();
    println!("{}", ("Updated in ".to_owned() + &elapsed_time.as_secs().to_string() + " seconds.").yellow());
}
