use std::fs::OpenOptions;
use std::io::Write;
use std::time::Instant;
use colored::Colorize;
use crate::CHARS;
use crate::commands::update::framedata_json::frames_to_json;
extern crate ureq;

const SITE_LINK: &str = "https://dustloop.com/wiki/api.php?action=cargoquery&format=json&limit=100&tables=MoveData_GGST&fields=MoveData_GGST.input%2C%20MoveData_GGST.name%2C%20MoveData_GGST.damage%2C%20MoveData_GGST.guard%2C%20MoveData_GGST.invuln%2C%20MoveData_GGST.startup%2C%20MoveData_GGST.active%2C%20MoveData_GGST.recovery%2C%20MoveData_GGST.onHit%2C%20MoveData_GGST.onBlock%2C%20MoveData_GGST.level%2C%20MoveData_GGST.riscGain%2C%20MoveData_GGST.prorate%2C%20MoveData_GGST.counter&where=chara%3D%22";
const SITE_HALF: &str = "%22&order_by=MoveData_GGST.type%20ASC%2C%20MoveData_GGST.input%20ASC&utf8=1";

pub async fn get_char_data (chars_ids: [&str; CHARS.len()], specific_char: &str) {
    // For timing the updates
    let now = Instant::now();
    
    if specific_char == "all"{
    
        for (x, char_id) in chars_ids.iter().enumerate() {
    
            println!("{}", ("Creating '".to_owned() + char_id + ".json' file.").green());
            
            let char_json_path = "data/".to_owned() + char_id +"/"+ char_id + ".json";
    
            // Creating character json file
            let mut file = OpenOptions::new()
                .create(true)
                .truncate(true)
                .read(true)
                .write(true)
                .open(char_json_path)
                .expect(&("\nFailed to open '".to_owned() + char_id + ".json' file."));
    
            // Creating request link 
            let character_link = SITE_LINK.to_owned() + &char_id.replace('_', " ") +  SITE_HALF;
    
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
                .expect(&("\nFailed to write 'char_json_schema' to '".to_owned() + char_id + ".json'."));
            
            // Sending response to get processed and serialized to a json file
            // char_count is a counter to specify which json file fails to update
            frames_to_json(char_page_response_json, &file, x).await;

            // Finalizing character json
            char_json_schema = "\n]";
            write!(file, "{}", char_json_schema)
                .expect(&("\nFailed to write 'json_schema' to '".to_owned() + char_id + ".json'.")); 
        }
    }
    else {

        println!("{}", ("Creating '".to_owned() + specific_char + ".json' file.").green());

        let char_json_path = "data/".to_owned() + specific_char +"/"+ specific_char + ".json";

        // Creating character json file
        let mut file = OpenOptions::new()
            .create(true)
            .truncate(true)
            .read(true)
            .write(true)
            .open(char_json_path)
            .expect(&("\nFailed to open '".to_owned() + specific_char + ".json' file."));

        // Creating request link
        let character_link = SITE_LINK.to_owned() + &specific_char.replace('_', " ") + SITE_HALF;

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
    println!("{}", ("Updated in ".to_owned() + &elapsed_time.as_secs().to_string() + " seconds.").yellow());
}
