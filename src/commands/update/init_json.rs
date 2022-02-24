use serde_json;
use std::{fs::{File, OpenOptions},io::prelude::*};

use crate::CHARS;
use crate::CharInfo;
use crate::commands::update::SITE_LINK;
use crate::commands::update::SITE_HALF;

pub fn init_json(){

    File::create("data/init.json")
        .expect("\nFailed to create 'init.json' file.");  


    // Creating the init.json data
    let mut json_schema = "[\n\t";
    
    let mut init_file = OpenOptions::new()
        .append(true)
        .open("data/init.json")
        .expect("\nFailed to read init.json file.");

    // Formatting the json file
    write!(init_file, "{}", json_schema)
        .expect("\nFailed to write 'json_schema' to 'init.json'.");
    
    // Creating all the char data
    for x in 0..CHARS.0.len(){

        // Iterating through the names and ids of each character
        // And constructing the link and page field values
        let init_json = serde_json::to_vec(&CharInfo{ 
            page: ("GGST/".to_owned() + &CHARS.0[x] + "/Frame_Data" ),
            link: (SITE_LINK.to_owned() + &CHARS.1[x].to_string() +  SITE_HALF),
            pageid: CHARS.1[x],
            // For time based restriction
            // update_timestamp: (SystemTime::now().duration_since(SystemTime::UNIX_EPOCH)
            //                                     .expect("\nFailed getting the UNIX timestamp.")
            //                                     .as_secs()),
        },).expect("\nFailed to serialize to json.");
        
        // Writting the char data
        init_file.write_all(&init_json)
            .expect("\nFailed to write 'CharInfo Vec<Struct>' to init.json.");
        
        // Skips the last ','
        if x != CHARS.0.len() - 1 {
            init_file.write(b",\n\t")
                .expect("\nFailed to write ',\\n\\t\\t' while serializing 'init.json'.");
        }
    }
    
    json_schema = "\n]";
    write!(init_file, "{}", json_schema)
        .expect("\nFailed to write ending 'json_schema' to 'init.json'.");
}