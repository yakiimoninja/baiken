use std::{fs::{File, self}, io::Write};
use crate::{Frames, CHARS};


pub fn write_data_to_json(move_information: String, mut file: &File, char_count: usize){

    let move_information: Vec<&str> = move_information.split("|").collect();
    let mut image_link_index = 0;

    let mut character_image_links = fs::read_to_string(&("data/images/".to_owned() + CHARS.0[char_count] + ".txt"))
                .expect(&("\nFailed to read 'data/images/".to_owned() + CHARS.0[char_count] + "'.txt' file."));

    character_image_links = character_image_links.replace("\r", "");
    let character_image_links: Vec<&str> = character_image_links.split("\n").collect();

    // Iterating through the vector containg the moves
    // With a step of 14 because most of the moves have 14 properties
    let mut x = 13;
    while x < move_information.len() {

        // Since semicolon dictates the end of a moves data
        // If semicolon is present in the 14th position
        // Proceed to write the data of these 14 positions
        if move_information[x].contains(";") == true {

            // Vectoring move data
            let vectored_frame_data = serde_json::to_vec(&Frames{
                r#move: move_information[x-13].to_string(),
                damage: move_information[x-12].to_string(),
                guard: move_information[x-11].to_string(),
                invincibility: move_information[x-1].to_string(),
                startup: move_information[x-10].to_string(),
                block: move_information[x-7].to_string(),
                hit: move_information[x-6].to_string(),
                active: move_information[x-9].to_string(),
                recovery: move_information[x-8].to_string(),
                counter: move_information[x-2].to_string(),
                img_link: character_image_links[image_link_index].to_string(),
            }).unwrap();

            file.write_all(&vectored_frame_data)
                .expect(&("\nFailed to serialize '".to_owned() + CHARS.0[char_count]+ ".json'."));

            if x != move_information.len() - 1 {
                if move_information[x-13] != "bt.22"{
                    
                    file.write(b",\n\t")
                        .expect(&("\nFailed to write ',\\n\\t' while serializing '".to_owned() + CHARS.0[char_count]+ ".json'."));
                
                    image_link_index += 1;
                }
            }            
        }

        // Instead of the 14 properties the normal moves have
        // Special and super moves have 15 properties
        // Since semicolon dictates the end of a moves data if
        // It isnt in the 14th position of the vector
        // We increment by 1 the pack of 14 data entries 
        // And skip the data in the second position cause we dont need it
        if move_information[x].contains(";") == false {            

            // Vectoring move data
            let vectored_frame_data = serde_json::to_vec(&Frames{
                r#move: move_information[x-13].to_string().replace("c.", "5").replace("f.", "5"),
                damage: move_information[x-11].to_string(),
                guard: move_information[x-10].to_string(),
                invincibility: move_information[x-0].to_string(),
                startup: move_information[x-9].to_string(),
                block: move_information[x-6].to_string(),
                hit: move_information[x-5].to_string(),
                active: move_information[x-8].to_string(),
                recovery: move_information[x-7].to_string(),
                counter: move_information[x-1].to_string(),
                img_link: character_image_links[image_link_index].to_string(),
            }).unwrap();
            
            file.write_all(&vectored_frame_data)
                .expect(&("\nFailed to serialize '".to_owned() + CHARS.0[char_count]+ ".json'."));

            if x != move_information.len() - 1 || move_information[x-13] == "bt.22" {
                if move_information[x-13] != "bt.22"{

                    file.write(b",\n\t")
                        .expect(&("\nFailed to write ',\\n\\t' while serializing '".to_owned() + CHARS.0[char_count]+ ".json'."));
                
                    image_link_index += 1;
                }
            }  

            x += 1;
        }
        
        x += 14;
    }
}