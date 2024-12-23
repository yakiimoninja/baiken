extern crate ureq;
use rusqlite::{params, Connection as SqlConnection};
use md5::{Digest, Md5};
use serde::Deserialize;
//use ureq::Error;
//use std::fs::OpenOptions;

#[derive(Deserialize, Debug)]
struct ImageResponse {
    #[serde(rename = "cargoquery")]
    cargoquery: Vec<ImageData>
}

#[derive(Deserialize, Debug)]
struct ImageData {
    #[serde(rename = "title")]
    title: ImageTitle
}

#[derive(Deserialize, Debug)]
struct ImageTitle {
    input: Option<String>,
    name: Option<String>,
    images: Option<String>,
    hitboxes: Option<String>,
}

const IMAGE_HALF: &str = "https://www.dustloop.com/wiki/images";

async fn make_link(image_name: String) -> String {

    let image_bytes = image_name.as_bytes();

    // Creating a Md5 hasher instance
    let mut hasher = Md5::new();
    hasher.update(image_bytes);
    // Converting hex to string
    let result = format!("{:x}", hasher.finalize());
    // Getting the first two hex digits from the md5sum
    // let char1 = result.chars().nth(0).unwrap();
    let char1 = result.chars().next().unwrap();
    let char2 = result.chars().nth(1).unwrap();
    // Making final link by concating 
    // https://www.dustloop.com/wiki/images/first hex digit/first hex second hex/image names with underscores instead of spaces
    let image_link = format!("{}/{}/{}{}/{}", IMAGE_HALF, char1, char1, char2, image_name);

    // // Debug testing links and outputting the broken ones in a file
    // match ureq::get(&image_link).call() {
    //     Ok(_) => {},
    //     Err(Error::Status(code, _/*response*/)) => {
    //         // Creating character images json file
    //         let mut file = OpenOptions::new()
    //             .create(true)
    //             .append(true)
    //             .open("broken_links.txt")
    //             .expect("\nFailed to open 'broken_links.txt'");

    //         write!(file, "Code: {}, Link: {}\n", code, image_link)
    //             .expect("\nFailed to write to 'broken_links.txt'");
    //     }
    //     Err(_) => {}
    // }
    
    image_link
}

/// Pushes move hitbox image links into Hitboxes table
pub async fn push_hitboxes_to_db(db: SqlConnection, char_count: usize, move_input: String, hitbox: String) -> SqlConnection {

    db.execute("
INSERT INTO hitboxes (character_id, input, hitbox, hitbox_caption)
VALUES (?1, ?2, ?3, ?4)

ON CONFLICT(character_id, input)
DO UPDATE SET 
hitbox = ?3, hitbox_caption = ?4", params![char_count + 1, move_input, hitbox, "TODO"]).unwrap();

    db
}

/// Pushes a move image link into Move_Data table
pub async fn push_images_to_db(db: SqlConnection, char_count: usize, move_input: String, image: String) -> SqlConnection {

    db.execute("
INSERT INTO moves (character_id, input)
VALUES (?1, ?2)

ON CONFLICT(character_id, input)
DO UPDATE SET 
image = ?3", params![char_count + 1, move_input, image]).unwrap();

    db
}

pub async fn images_to_db(char_images_response_json: &str, mut db: SqlConnection, char_count: usize) -> SqlConnection {

    // Replace apostrophe
    let char_images_response_json = &char_images_response_json.replace(r#"&#039;"#, "'");

    let mut image_data_response: ImageResponse = serde_json::from_str(char_images_response_json).unwrap();
    let char_image_data = &mut image_data_response.cargoquery;

    for image_data in char_image_data {
        
        // Variable that the produced image link will reside
        let image_link;

        // Replacing None values with a generic '-'
        if image_data.title.input.is_none() {
            image_data.title.input = Some("".to_string());
        }
        else {
            // Skips finish blow for sol
            if *image_data.title.input.as_ref().unwrap() == "j.XX during Homing Jump" {
                continue;
            }
        }
        if image_data.title.name.is_none() {
            image_data.title.name = Some(image_data.title.input.as_ref().unwrap().to_string());
        }
        else {
            // Skips dash cancel entry ino hoverdash chipp escape zato flight and finish blow
            if image_data.title.name.as_ref().unwrap().to_string().trim() == "Dash Cancel" ||
            image_data.title.name.as_ref().unwrap().to_string().trim() == "Hoverdash" ||
            image_data.title.name.as_ref().unwrap().to_string().trim() == "Finish Blow" ||
            image_data.title.name.as_ref().unwrap().to_string().trim() == "Flight" ||
            image_data.title.name.as_ref().unwrap().to_string().trim() == "Escape" {
                continue;
            }
        }
        if image_data.title.images.is_none() {
            image_link = "".to_string();
        }
        else {
            // If image field contains only spaces
            if image_data.title.images.as_ref().unwrap().trim() == "" {
                image_link = "".to_string();
            }
            else {
                // Multiple image names
                // Removing any subsequent image names from field
                if image_data.title.images.as_mut().unwrap().contains(';') {
    
                    let split_image: Vec<&str> = image_data.title.images.as_mut().unwrap().split(';').collect();
                        
                    image_data.title.images = Some(split_image[0].to_string().replace(' ', "_"));
    
                    // Sending image name to make_link to become a link
                    image_link = make_link(image_data.title.images.as_ref().unwrap().to_string()).await;
                }
                else {
                    // Single image name
                    image_data.title.images = Some(image_data.title.images.as_ref().unwrap().to_string().replace(' ', "_"));
                    // Sending image name to make_link to become a link
                    image_link = make_link(image_data.title.images.as_ref().unwrap().to_string()).await;
                }
            }
        }
        
        // If hitbox empty
        if image_data.title.hitboxes.is_none() {
            // Push image link in db here
            db = push_hitboxes_to_db(db, char_count, image_data.title.input.as_ref().unwrap().to_string(), String::from("")).await;
        }
        else {
            // Splitting the hitboxes names into a vector
            let hitbox_str: Vec<&str> = image_data.title.hitboxes.as_ref().unwrap().split(';').collect();
            
            for hitbox_string in &hitbox_str {
                // Push hitbox and or here
                db = push_hitboxes_to_db(
                    db, 
                    char_count,
                    image_data.title.input.as_ref().unwrap().to_string(),
                    make_link(hitbox_string.to_string().trim().replace(' ', "_")).await
                ).await;
            }
        }

        // Push image link into Move_Data table
        db = push_images_to_db(db, char_count, image_data.title.input.as_ref().unwrap().to_string(), image_link).await;
    }

    db
}
