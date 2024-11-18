extern crate ureq;
use md5::{Digest, Md5};
use serde::Deserialize;
use crate::{CHARS, ImageLinks};
use std::{
    fs::File,
    io::Write,
};
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

pub async fn images_to_json(mut char_images_response_json: String, mut file: &File, char_count: usize) {

    // Replace apostrophe
    char_images_response_json = char_images_response_json.replace(r#"&#039;"#, "'");

    let mut image_data_response: ImageResponse = serde_json::from_str(&char_images_response_json).unwrap();
    let char_image_data = &mut image_data_response.cargoquery;
    let mut vec_processed_imagedata = Vec::new();

    for image_data in char_image_data {
        
        // Variable that the produced hitbox links will reside
        let mut hitbox_links: Vec<String> = Vec::new();
        // Variable that the produced image link will reside
        let image_link;

        // Replacing None values with a generic '-'
        if image_data.title.input.is_none(){
            image_data.title.input = Some("".to_string());
        }
        else{
            // Skips finish blow for sol
            if *image_data.title.input.as_ref().unwrap() == "j.XX during Homing Jump" {
                continue;
            }
        }
        if image_data.title.name.is_none(){
            image_data.title.name = Some(image_data.title.input.as_ref().unwrap().to_string());
        }
        else{
            // Skips dash cancel entry ino hoverdash chipp escape zato flight and finish blow
            if image_data.title.name.as_ref().unwrap().to_string().trim() == "Dash Cancel" ||
            image_data.title.name.as_ref().unwrap().to_string().trim() == "Hoverdash" ||
            image_data.title.name.as_ref().unwrap().to_string().trim() == "Finish Blow" ||
            image_data.title.name.as_ref().unwrap().to_string().trim() == "Flight" ||
            image_data.title.name.as_ref().unwrap().to_string().trim() == "Escape" {
                continue;
            }
        }
        if image_data.title.images.is_none(){
            image_link = "".to_string();
        }
        else{
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
                else{
                    // Single image name
                    image_data.title.images = Some(image_data.title.images.as_ref().unwrap().to_string().replace(' ', "_"));
                    // Sending image name to make_link to become a link
                    image_link = make_link(image_data.title.images.as_ref().unwrap().to_string()).await;
                }
            }
        }
        
        // If hitbox empty
        if image_data.title.hitboxes.is_none(){
            hitbox_links.push("".to_string());
        }
        else{
            // // If image field contains only spaces
            // if char_image_data[x].title.hitboxes.as_ref().unwrap().trim() == "" {
            //     hitbox_links.push("".to_string());
            // }
            // Remove any hitbox images for throws cause they dont exist !!!!! this breaks wa 
            //if char_image_data[x].title.hitboxes.as_ref().unwrap().trim().to_lowercase().contains("6d") {
            //    hitbox_links.push("".to_string());
            //}
            //else{
            // Splitting the hitboxes names into a vector
            let hitbox_str: Vec<&str> = image_data.title.hitboxes.as_ref().unwrap().split(';').collect();
            
            for hitbox_string in &hitbox_str{
                // Sending hitbox names to make_link to become a vector of links
                hitbox_links.push(make_link(hitbox_string.to_string().trim().replace(' ', "_")).await);
            }
            //}
        }

        // Serializing image data
        let processed_imagedata = ImageLinks {
            input: image_data.title.input.as_ref().unwrap().to_string(),
            move_img: image_link,
            hitbox_img: hitbox_links,
        };

        vec_processed_imagedata.push(processed_imagedata);
    }

    file.write_all(&(serde_json::to_vec_pretty(&vec_processed_imagedata).unwrap()))
        .expect(&("\nFailed to serialize '".to_owned() + CHARS[char_count]+ ".json'."));
}

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
