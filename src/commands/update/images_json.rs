use std::fs::OpenOptions;
use std::io::Write;
use std::path::Path;
use std::time::Instant;
use tokio::fs::remove_file;
use crate::CHARS;
use crate::commands::update::make_json::{images_to_json};
//use crate::Data;
//use crate::char_json::data_to_json::write_data_to_json;
extern crate ureq;


const SITE_LINK: &str = "https://dustloop.com/wiki/api.php?action=cargoquery&format=json&tables=MoveData_GGST&fields=MoveData_GGST.input%2C%20MoveData_GGST.name%2C%20MoveData_GGST.images%2C%20MoveData_GGST.hitboxes&where=chara%3D%22";
const SITE_HALF: &str = "%22&order_by=MoveData_GGST.type%20ASC%2C%20MoveData_GGST.input%20ASC&utf8=1";

pub async fn get_char_data(chars_ids: [&str; CHARS.len()], specific_char: &str) {
    // For timing the updates
    let now = Instant::now();
    
    print!("\n");

    if specific_char == "all"{

        for x in 0..chars_ids.len() {
    
            println!("Creating {} 'images.json' file.", chars_ids[x]);
            
            let images_json_path = "data/".to_owned() + &chars_ids[x] +"/images.json";
    
            if Path::new(&images_json_path).exists() == true {
                remove_file(&images_json_path).await.unwrap();
            }
    
            // Creating character images json file
            let mut file = OpenOptions::new()
                .create(true)
                .append(true)
                .open(images_json_path)
                .expect(&("\nFailed to open ".to_owned() + &chars_ids[x] + " 'images.json' file."));
    
            // Creating images request link 
            let character_images_link = SITE_LINK.to_owned() + &chars_ids[x].replace("_", " ") +  SITE_HALF;
    
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
            
            // More character json file stuff
            let mut char_json_schema = "[\n\t";
            write!(file, "{}", char_json_schema)
                .expect(&("\nFailed to write 'char_json_schema' to ".to_owned() + &chars_ids[x] + " 'images.json'."));
            
            // Sending response to get processed and serialized to a json file
            // char_count is a counter to specify which json file fails to update
            images_to_json(char_images_response_json, &file, x).await;

            // Finalizing character images json
            char_json_schema = "\n]";
            write!(file, "{}", char_json_schema)
                .expect(&("\nFailed to write 'json_schema' to ".to_owned() + chars_ids[x] + " 'images.json'."));
        }
    }
    else {

        println!("Creating {} 'images.json' file.", specific_char);

        let images_json_path = "data/".to_owned() + specific_char +"/images.json";

        if Path::new(&images_json_path).exists() == true {
            remove_file(&images_json_path).await.unwrap();
        }

        // Creating character json file
        let mut file = OpenOptions::new()
            .create(true)
            .append(true)
            .open(images_json_path)
            .expect(&("\nFailed to open ".to_owned() + specific_char + " 'images.json' file."));

        // Creating request link
        let character_link = SITE_LINK.to_owned() + &specific_char.replace("_", " ") + SITE_HALF;

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
 
        // More character json file stuff
        let mut char_json_schema = "[\n\t";
        write!(file, "{}", char_json_schema)
            .expect(&("\nFailed to write 'char_json_schema' to ".to_owned() + specific_char + " 'images.json'."));

        // Sending response to get processed and serialized to a json file
        // char_count is a counter to specify which json file fails to update
        images_to_json(char_images_response_json, &file, 0).await;

        // Finalizing character images json
        char_json_schema = "\n]";
        write!(file, "{}", char_json_schema)
            .expect(&("\nFailed to write 'json_schema' to ".to_owned() + specific_char + " 'images.json'."));

    }
    
    let elapsed_time = now.elapsed();
    println!("\nUpdated in {} seconds.", elapsed_time.as_secs());
}