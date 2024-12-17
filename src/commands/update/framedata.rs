extern crate ureq;
use std::time::Instant;
use colored::Colorize;
use rusqlite::Connection as SqlConnection;
use crate::{CHARS, commands::update::framedata_db::frames_to_db};

const SITE_LINK: &str = "https://www.dustloop.com/wiki/api.php?action=cargoquery&format=json&limit=100&tables=MoveData_GGST&fields=MoveData_GGST.input%2C%20MoveData_GGST.name%2C%20MoveData_GGST.damage%2C%20MoveData_GGST.guard%2C%20MoveData_GGST.startup%2C%20MoveData_GGST.active%2C%20MoveData_GGST.recovery%2C%20MoveData_GGST.onHit%2C%20MoveData_GGST.onBlock%2C%20MoveData_GGST.level%2C%20MoveData_GGST.counter%2C%20MoveData_GGST.type%2C%20MoveData_GGST.riscGain%2C%20MoveData_GGST.riscLoss%2C%20MoveData_GGST.wallDamage%2C%20MoveData_GGST.inputTension%2C%20MoveData_GGST.chipRatio%2C%20MoveData_GGST.OTGRatio%2C%20MoveData_GGST.prorate%2C%20MoveData_GGST.invuln%2C%20MoveData_GGST.cancel%2C%20MoveData_GGST.caption%2C%20MoveData_GGST.notes%2C%20MoveData_GGST.hitboxCaption%2C%20MoveData_GGST.images%2C%20MoveData_GGST.hitboxes%2C&where=chara%3D%22";
const SITE_HALF: &str = "%22&order_by=MoveData_GGST.type%20ASC%2C%20MoveData_GGST.input%20ASC&utf8=1";

pub async fn get_char_data(chars_ids: [&str; CHARS.len()], specific_char: &str) {
    // For timing the updates
    let now = Instant::now();
    let mut db = SqlConnection::open("data/data.db").unwrap();
    
    if specific_char == "all" {
    
        for (x, char_id) in chars_ids.iter().enumerate() {
    
            println!("{}", ("Updating '".to_owned() + char_id + "' data.").green());
    
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
            
            // Sending response to get processed and pushed into a db
            // char_count is a counter to specify which json file fails to update
            db = frames_to_db(&char_page_response_json, db, x).await;        
        }
    }
    else {

        println!("{}", ("Updating '".to_owned() + specific_char + "' data.").green());

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
        
        // Sending response to get processed and pushed into a db
        // char_count is a counter to specify which json file fails to update
        for (x, char_id) in chars_ids.iter().enumerate() {
            if *char_id == specific_char {
                db = frames_to_db(&char_page_response_json, db, x).await;        
                break;
            }
        }
    }
    
    db.close().unwrap();
    let elapsed_time = now.elapsed();
    println!("{}", ("Updated in ".to_owned() + &elapsed_time.as_secs().to_string() + " seconds.").yellow());
}
