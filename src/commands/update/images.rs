extern crate ureq;
use std::time::Instant;
use rusqlite::{Connection as SqlConnection, OpenFlags};
use colored::Colorize;
use crate::{CHARS, commands::update::images_db::images_to_db};

const SITE_LINK: &str = "https://dustloop.com/wiki/api.php?action=cargoquery&format=json&limit=100&tables=MoveData_GGST&fields=MoveData_GGST.input%2C%20MoveData_GGST.name%2C%20MoveData_GGST.images%2C%20MoveData_GGST.hitboxes%2C%20MoveData_GGST.hitboxCaption&where=chara%3D%22";
const SITE_HALF: &str = "%22&order_by=MoveData_GGST.type%20ASC%2C%20MoveData_GGST.input%20ASC&utf8=1";

pub async fn get_char_images(chars_ids: [&str; CHARS.len()], specific_char: &str) {
    // For timing the updates
    let now = Instant::now();
    let mut db = SqlConnection::open_with_flags("data/data.db", OpenFlags::SQLITE_OPEN_READ_WRITE).unwrap();

    if specific_char == "all" {

        for (x, char_id) in chars_ids.iter().enumerate() {

            println!("{}", ("Updating '".to_owned() + char_id + "' images.").green());

            // Creating images request link 
            let character_images_link = SITE_LINK.to_owned() + char_id +  SITE_HALF;

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
            db = images_to_db(&char_images_response_json, db, x).await;
        }
    }
    else {

        println!("{}", ("Updating '".to_owned() + specific_char + "' images.").green());

        // Creating request link
        let character_link = SITE_LINK.to_owned() + specific_char + SITE_HALF;

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

        for (x, char_id) in chars_ids.iter().enumerate() {
            if *char_id == specific_char {
                // Sending response to get processed and serialized to a json file
                // char_count is a counter to specify which json file fails to update
                db = images_to_db(&char_images_response_json, db, x).await;
                break;
            }
        }
    }

    db.close().unwrap();
    let elapsed_time = now.elapsed();
    println!("{}", ("Updated in ".to_owned() + &elapsed_time.as_millis().to_string() + "ms.").yellow());

}
