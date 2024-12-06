extern crate ureq;
use std::{fs::OpenOptions, time::Instant};
use colored::Colorize;
use crate::{CHARS, commands::update::images_json::images_to_json};


const SITE_LINK: &str = "https://dustloop.com/wiki/api.php?action=cargoquery&format=json&limit=100&tables=MoveData_GGST&fields=MoveData_GGST.input%2C%20MoveData_GGST.name%2C%20MoveData_GGST.images%2C%20MoveData_GGST.hitboxes&where=chara%3D%22";
const SITE_HALF: &str = "%22&order_by=MoveData_GGST.type%20ASC%2C%20MoveData_GGST.input%20ASC&utf8=1";

pub async fn get_char_images(chars_ids: [&str; CHARS.len()], specific_char: &str) {
    // For timing the updates
    let now = Instant::now();
    
    if specific_char == "all" {

        for (x, char_id) in chars_ids.iter().enumerate() {
    
            println!("{}", ("Creating ".to_owned() + char_id + " 'images.json' file.").green());
            
            let images_json_path = "data/".to_owned() + char_id +"/images.json";
    
            // Creating multiple character images.json files
            let file = OpenOptions::new()
                .create(true)
                .truncate(true)
                .write(true)
                .open(images_json_path)
                .expect(&("\nFailed to open ".to_owned() + char_id + " 'images.json' file."));
    
            // Creating images request link 
            let character_images_link = SITE_LINK.to_owned() + &char_id.replace('_', " ") +  SITE_HALF;
    
            // Dusloop site request
            let mut char_images_response_json = ureq::get(&character_images_link)
                .call()
                .unwrap();

            // Because dustloop site 500 a lot
            while char_images_response_json.status() == 500 {
                char_images_response_json = ureq::get(&character_images_link)
                    .call()
                    .unwrap();
            }
    
            // Requested website source to file
            let char_images_response_json = char_images_response_json.into_string().unwrap();
            
            // Sending response to get processed and serialized to a json file
            // char_count is a counter to specify which json file fails to update
            images_to_json(char_images_response_json, &file, x).await;
        }
    }
    else {

        println!("{}", ("Creating ".to_owned() + specific_char + " 'images.json' file.").green());

        let images_json_path = "data/".to_owned() + specific_char +"/images.json";

        // Creating singular character images.json file
        let file = OpenOptions::new()
            .create(true)
            .truncate(true)
            .write(true)
            .open(images_json_path)
            .expect(&("\nFailed to open ".to_owned() + specific_char + " 'images.json' file."));

        // Creating request link
        let character_link = SITE_LINK.to_owned() + &specific_char.replace('_', " ") + SITE_HALF;

        // Dusloop site request
        let mut char_images_response_json = ureq::get(&character_link)
            .call()
            .unwrap();
        
        // Because dustloop site 500 a lot
        while char_images_response_json.status() == 500 {
            char_images_response_json = ureq::get(&character_link)
                .call()
                .unwrap();
        }

        // Requested website source to file
        let char_images_response_json = char_images_response_json.into_string().unwrap();
 
        // Sending response to get processed and serialized to a json file
        // char_count is a counter to specify which json file fails to update
        images_to_json(char_images_response_json, &file, 0).await;
    }
    
    let elapsed_time = now.elapsed();
    println!("{}", ("Updated in ".to_owned() + &elapsed_time.as_secs().to_string() + " seconds.").yellow());

}
