use std::{fs::File, io::Write};
use crate::{Frames, CHARS};


pub fn write_data_to_json(move_information: String, mut file: &File, char_count: usize) {

    let move_information: Vec<&str> = move_information.split("|").collect();

    // Iterating through the vector containg the moves
    // With a step of 14 because most of the moves have 14 properties
    let mut x = 13;
    while x < move_information.len() {

        // Since semicolon dictates the end of a moves data
        // If semicolon is present in the 14th position
        // Proceed to write the data of these 14 positions
        if move_information[x].contains(";") == true {

            // Replaces the contents of the input field with the actual input for both throws
            let mut move_input = move_information[x-13].to_string();
            if move_information[x-13] == "Ground Throw" {
                move_input = move_information[x-13].replace("Ground Throw", "6D or 4D");
            }
            else if move_information[x-13] == "Air Throw" {
                move_input = move_information[x-13].replace("Air Throw", "j.6D or j.4D");
            }

            // Vectoring move data
            let vectored_frame_data = serde_json::to_vec(&Frames {
                input: move_input,
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
            }).unwrap();

            file.write_all(&vectored_frame_data)
                .expect(&("\nFailed to serialize '".to_owned() + CHARS[char_count]+ ".json'."));

            if x != move_information.len() - 1 {
                if move_information[x-13] != "bt.22" {
                    
                    file.write(b",\n\t")
                        .expect(&("\nFailed to write ',\\n\\t' while serializing '".to_owned() + CHARS[char_count]+ ".json'."));
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
            let vectored_frame_data = serde_json::to_vec(&Frames {
                input: move_information[x-13].to_string().replace("c.", "5").replace("f.", "5"),
                r#move: move_information[x-12].to_string().replace("c.", "5").replace("f.", "5"),
                damage: move_information[x-11].to_string(),
                guard: move_information[x-10].to_string(),
                invincibility: move_information[x-0].to_string(),
                startup: move_information[x-9].to_string(),
                block: move_information[x-6].to_string(),
                hit: move_information[x-5].to_string(),
                active: move_information[x-8].to_string(),
                recovery: move_information[x-7].to_string(),
                counter: move_information[x-1].to_string(),
            }).unwrap();
            
            file.write_all(&vectored_frame_data)
                .expect(&("\nFailed to serialize '".to_owned() + CHARS[char_count] + ".json'."));

            if x != move_information.len() - 1 || move_information[x-13] == "bt.22" {
                if move_information[x-13] != "bt.22" {

                    file.write(b",\n\t")
                        .expect(&("\nFailed to write ',\\n\\t' while serializing '".to_owned() + CHARS[char_count]+ ".json'."));
                
                }
            }  

            x += 1;
        }
        
        x += 14;
    }
}