use std::fs::OpenOptions;
use std::io::Write;
use std::path::Path;
use std::time::Instant;
use tokio::fs::remove_file;
use crate::CHARS;
use crate::commands::update::make_json::frames_to_json;
//use crate::Data;
//use crate::char_json::data_to_json::write_data_to_json;
extern crate ureq;

const SITE_LINK: &str = "https://dustloop.com/wiki/api.php?action=cargoquery&format=json&tables=MoveData_GGST&fields=MoveData_GGST.input%2C%20MoveData_GGST.name%2C%20MoveData_GGST.damage%2C%20MoveData_GGST.guard%2C%20MoveData_GGST.invuln%2C%20MoveData_GGST.startup%2C%20MoveData_GGST.active%2C%20MoveData_GGST.recovery%2C%20MoveData_GGST.onHit%2C%20MoveData_GGST.onBlock%2C%20MoveData_GGST.level%2C%20MoveData_GGST.riscGain%2C%20MoveData_GGST.prorate%2C%20MoveData_GGST.counter&where=chara%3D%22";
const SITE_HALF: &str = "%22&order_by=MoveData_GGST.type%20ASC%2C%20MoveData_GGST.input%20ASC&utf8=1";

pub async fn get_char_data (chars_ids: [&str; CHARS.len()], specific_char: &str) {
    // For timing the updates
    let now = Instant::now();
    
    print!("\n");

    if specific_char == "all"{
    
        for x in 0..chars_ids.len() {
    
            println!("Creating '{}.json' file.", chars_ids[x]);
            
            let char_json_path = "data/".to_owned() + &chars_ids[x] +"/"+ &chars_ids[x] + ".json";
    
            if Path::new(&char_json_path).exists() == true {
                remove_file(&char_json_path).await.unwrap();
            }
    
            // Creating character json file
            let mut file = OpenOptions::new()
                .create(true)
                .append(true)
                .open(char_json_path)
                .expect(&("\nFailed to open '".to_owned() + &chars_ids[x] + ".json' file."));
    
            // Creating request link 
            let character_link = SITE_LINK.to_owned() + &chars_ids[x].replace("_", " ") +  SITE_HALF;
    
            // Dusloop site request
            let mut char_page_response_json = ureq::get(&character_link)
                .call()
                .unwrap();
            
            // Because dustloop site 500 a lot
            while char_page_response_json.status() == 500 {
                char_page_response_json = ureq::get(&character_link)
                    .call()
                    .unwrap();
            }
    
            // Requested website source to file
            let char_page_response_json = char_page_response_json.into_string().unwrap();
            
            // More character json file stuff
            let mut char_json_schema = "[\n\t";
            write!(file, "{}", char_json_schema)
                .expect(&("\nFailed to write 'char_json_schema' to '".to_owned() + &chars_ids[x] + ".json'."));
            
            // Sending response to get processed and serialized to a json file
            // char_count is a counter to specify which json file fails to update
            frames_to_json(char_page_response_json, &file, x).await;

            // Finalizing character json
            char_json_schema = "\n]";
            write!(file, "{}", char_json_schema)
                .expect(&("\nFailed to write 'json_schema' to '{".to_owned() + chars_ids[x] + "}.json'.")); 
        }
    }
    else {

        println!("Creating '{}.json' file.", specific_char);

        let char_json_path = "data/".to_owned() + specific_char +"/"+ specific_char + ".json";

        if Path::new(&char_json_path).exists() == true {
            remove_file(&char_json_path).await.unwrap();
        }

        // Creating character json file
        let mut file = OpenOptions::new()
            .create(true)
            .append(true)
            .open(char_json_path)
            .expect(&("\nFailed to open '".to_owned() + specific_char + ".json' file."));

        // Creating request link
        let character_link = SITE_LINK.to_owned() + &specific_char.replace("_", " ") + SITE_HALF;

        // Dusloop site request
        let mut char_page_response_json = ureq::get(&character_link)
            .call()
            .unwrap();
        
        // Because dustloop site 500 a lot
        while char_page_response_json.status() == 500 {
            char_page_response_json = ureq::get(&character_link)
                .call()
                .unwrap();
        }

        // Requested website source to file
        let char_page_response_json = char_page_response_json.into_string().unwrap();
        
        // More character json file stuff
        let mut char_json_schema = "[\n\t";
        write!(file, "{}", char_json_schema)
            .expect(&("\nFailed to write 'char_json_schema' to '".to_owned() + specific_char + ".json'."));

        // Sending response to get processed and serialized to a json file
        // char_count is a counter to specify which json file fails to update
        frames_to_json(char_page_response_json, &file, 0).await;

        // Finalizing character json
        char_json_schema = "\n]";
        write!(file, "{}", char_json_schema)
            .expect(&("\nFailed to write 'json_schema' to '{".to_owned() + specific_char + "}.json'."));

    }
    
    let elapsed_time = now.elapsed();
    println!("\nUpdated in {} seconds.", elapsed_time.as_secs());
}