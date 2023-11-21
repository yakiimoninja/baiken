use std::io::Write;
use serde::Deserialize;
use crate::{CHARS, ImageLinks};
use md5::{Digest, Md5};
use std::fs::File;
extern crate ureq;
//use ureq::Error;
//use std::fs::OpenOptions;

#[derive(Deserialize, Debug)]
struct Imageresponse {
    #[serde(rename = "cargoquery")]
    cargoquery: Vec<Imagedata>
}

#[derive(Deserialize, Debug)]
struct Imagedata {
    #[serde(rename = "title")]
    title: Imagetitle
}

#[derive(Deserialize, Debug)]
struct Imagetitle {
    input: Option<String>,
    name: Option<String>,
    images: Option<String>,
    hitboxes: Option<String>,
}

const IMAGE_HALF: &str = "https://www.dustloop.com/wiki/images";

pub async fn images_to_json(char_images_response_json: String, mut file: &File, char_count: usize) {

    let mut imagedata: Imageresponse = serde_json::from_str(&char_images_response_json).unwrap();

    for x in 0..imagedata.cargoquery.len() {
        
        // Variable that the produced hitbox links will reside
        let mut hitboxes_link: Vec<String> = Vec::new();
        // Variable that the produced image link will reside
        let image_link;

        // Replacing None values with a generic '-'
        if imagedata.cargoquery[x].title.input.is_none(){
            imagedata.cargoquery[x].title.input = Some("".to_string());
        }
        else{
            // Skips finish blow for sol
            if *imagedata.cargoquery[x].title.input.as_ref().unwrap() == "j.XX during Homing Jump" {
                continue;
            }
        }
        if imagedata.cargoquery[x].title.name.is_none(){
            imagedata.cargoquery[x].title.name = Some(imagedata.cargoquery[x].title.input.as_ref().unwrap().to_string());
        }
        else{
            // Skips dash cancel entry ino hoverdash chipp escape zato flight and finish blow
            if imagedata.cargoquery[x].title.name.as_ref().unwrap().to_string().trim() == "Dash Cancel" ||
            imagedata.cargoquery[x].title.name.as_ref().unwrap().to_string().trim() == "Hoverdash" ||
            imagedata.cargoquery[x].title.name.as_ref().unwrap().to_string().trim() == "Finish Blow" ||
            imagedata.cargoquery[x].title.name.as_ref().unwrap().to_string().trim() == "Flight" ||
            imagedata.cargoquery[x].title.name.as_ref().unwrap().to_string().trim() == "Escape" {
                continue;
            }
        }
        if imagedata.cargoquery[x].title.images.is_none(){
            image_link = "".to_string();
        }
        else{
            // If image field contains only spaces
            if imagedata.cargoquery[x].title.images.as_ref().unwrap().trim() == "" {
                image_link = "".to_string();
            }
            else {
                // Multiple image names
                // Removing any subsequent image names from field
                if imagedata.cargoquery[x].title.images.as_mut().unwrap().contains(';') {
    
                    let split_image: Vec<&str> = imagedata.cargoquery[x].title.images
                        .as_mut()
                        .unwrap()
                        .split(';')
                        .collect();
                        
                    imagedata.cargoquery[x].title.images = Some(split_image[0]
                        .to_string()
                        .replace(' ', "_"));
    
                    // Sending image name to make_link to become a link
                    image_link = make_link(imagedata.cargoquery[x].title.images.as_ref().unwrap().to_string()).await;
                }
                else{
                    // Single image name
                    imagedata.cargoquery[x].title.images = Some(imagedata.cargoquery[x].title.images
                        .as_ref()
                        .unwrap()
                        .to_string()
                        .replace(' ', "_"));
                    // Sending image name to make_link to become a link
                    image_link = make_link(imagedata.cargoquery[x].title.images.as_ref().unwrap().to_string()).await;
                }
            }
        }
        
        // If hitbox empty
        if imagedata.cargoquery[x].title.hitboxes.is_none(){
            hitboxes_link.push("".to_string());
        }
        else{
            // // If image field contains only spaces
            // if imagedata.cargoquery[x].title.hitboxes.as_ref().unwrap().trim() == "" {
            //     hitboxes_link.push("".to_string());
            // }
            // Remove any hitbox images for throws cause they dont exist
            if imagedata.cargoquery[x].title.hitboxes.as_ref().unwrap().trim().to_lowercase().contains("6d") {
                hitboxes_link.push("".to_string());
            }
            else{
                // Splitting the hitboxes names into a vector
                let hitbox_str: Vec<&str> = imagedata.cargoquery[x].title.hitboxes.as_ref().unwrap().split(';').collect();
                
                for hitbox_string in &hitbox_str{
                    // Sending hitbox names to make_link to become a vector of links
                    hitboxes_link.push(make_link(hitbox_string
                        .to_string()
                        .trim()
                        .replace(' ', "_")).await);
                }
            }
        }

        // Serializing image data
        let processed_imagedata = serde_json::to_string_pretty(&ImageLinks {
            input: imagedata.cargoquery[x].title.input.as_ref().unwrap().to_string(),
            move_img: image_link,
            hitbox_img: hitboxes_link,
        }).unwrap();
        
        write!(file, "{}", processed_imagedata)
        .expect(&("\nFailed to serialize '".to_owned() + CHARS[char_count]+ ".json'."));
        
        // Skip writting comma/tab if next and last iteration 
        // contains 'finish blow' in last the input field
        if x == imagedata.cargoquery.len() -2 &&
        *imagedata.cargoquery[x+1].title.input.as_ref().unwrap() == "j.XX during Homing Jump"{
            continue;
        }
        else if x != imagedata.cargoquery.len() - 1 {         
            // Adding comma/tab
            //file.write(b",\n\t")
            //    .expect(&("\nFailed to write ',\\n\\t' while serializing '".to_owned() + CHARS[char_count]+ ".json'."));
            (&mut file).write_all(b",\n\t")
                .expect(&("\nFailed to write ',\\n\\t' while serializing '".to_owned() + CHARS[char_count]+ ".json'."));
        } 
    }
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
