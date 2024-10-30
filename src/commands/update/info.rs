use std::fs::OpenOptions;
use std::io::Write;
use std::time::Instant;
use colored::Colorize;
use crate::CHARS;
use crate::commands::update::info_json::info_to_json;
extern crate ureq;

const SITE_LINK : &str = "https://dustloop.com/wiki/api.php?action=cargoquery&format=json&tables=ggstCharacters&fields=ggstCharacters.portrait%2C%20ggstCharacters.icon%2C%20ggstCharacters.name%2C%20ggstCharacters.defense%2C%20ggstCharacters.guts%2C%20ggstCharacters.guardBalance%2C%20ggstCharacters.prejump%2C%20ggstCharacters.backdash%2C%20ggstCharacters.forwarddash%2C%20ggstCharacters.umo%2C%20ggstCharacters.jump_duration%2C%20ggstCharacters.high_jump_duration%2C%20ggstCharacters.jump_height%2C%20ggstCharacters.high_jump_height%2C%20ggstCharacters.earliest_iad%2C%20ggstCharacters.ad_duration%2C%20ggstCharacters.abd_duration%2C%20ggstCharacters.ad_distance%2C%20ggstCharacters.abd_distance%2C%20ggstCharacters.movement_tension%2C%20ggstCharacters.jump_tension%2C%20ggstCharacters.airdash_tension%2C%20ggstCharacters.walk_speed%2C%20ggstCharacters.back_walk_speed%2C%20ggstCharacters.dash_initial_speed%2C%20ggstCharacters.dash_acceleration%2C%20ggstCharacters.dash_friction%2C%20ggstCharacters.jump_gravity%2C%20ggstCharacters.high_jump_gravity%2C&where=ggstCharacters.name%3D%22";
const SITE_HALF : &str = "%22";

pub async fn get_char_info(chars_ids: [&str; CHARS.len()], specific_char: &str) {
    // For timing the updates
    let now = Instant::now();
    
    if specific_char == "all"{
    
        for (x, char_id) in chars_ids.iter().enumerate() {
    
            println!("{}", ("Creating ".to_owned() + char_id + " 'info.json' file.").green());
            
            let char_info_json_path = "data/".to_owned() + char_id +"/info.json";
    
            // Creating character info.json file
            let mut file = OpenOptions::new()
                .create(true)
                .truncate(true)
                .read(true)
                .write(true)
                .open(char_info_json_path)
                .expect(&("\nFailed to open ".to_owned() + char_id + " 'info.json' file."));
    
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
            
            // More character info.json file stuff
            //let mut char_json_schema = "[\n\t";
            //write!(file, "{}", char_json_schema)
                //.expect(&("\nFailed to write 'char_json_schema' to ".to_owned() + char_id + " 'info.json'."));
            
            // Sending response to get processed and serialized to a json file
            // char_count is a counter to specify which json file fails to update
            info_to_json(char_info_response_json, &file, x).await;

            // Finalizing character info.json
            //char_json_schema = "\n]";
            //write!(file, "{}", char_json_schema)
                //.expect(&("\nFailed to write 'json_schema' to ".to_owned() + char_id + " 'info.json'.")); 
        }
    }
    else {

        println!("{}", ("Creating ".to_owned() + specific_char + " 'info.json' file.").green());

        let char_info_json_path = "data/".to_owned() + specific_char +"/info.json";

        // Creating character info.json file
        let mut file = OpenOptions::new()
            .create(true)
            .truncate(true)
            .read(true)
            .write(true)
            .open(char_info_json_path)
            .expect(&("\nFailed to open ".to_owned() + specific_char + " 'info.json' file."));

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
        
        // More character info.json file stuff
        //let mut char_json_schema = "[\n\t";
        //write!(file, "{}", char_json_schema)
            //.expect(&("\nFailed to write 'char_json_schema' to '".to_owned() + specific_char + ".json'."));

        // Sending response to get processed and serialized to a json file
        // char_count is a counter to specify which json file fails to update
        info_to_json(char_info_response_json, &file, 0).await;

        // Finalizing character info.json
        //char_json_schema = "\n]";
        //write!(file, "{}", char_json_schema)
            //.expect(&("\nFailed to write 'json_schema' to '{".to_owned() + specific_char + "}.json'."));

    }
    
    let elapsed_time = now.elapsed();
    println!("{}", ("Updated in ".to_owned() + &elapsed_time.as_secs().to_string() + " seconds.").yellow());
}
