use crate::{CHARS, CharInfo, commands};
use std::{fs, path::Path};

pub fn init_check() {
    
    // Checking if 'data' folder exists
    if Path::new("data").exists() == true{

        // Checking if image folder and image txts exist
        if Path::new("data/images").exists() == true{
            for c in 0..CHARS.0.len(){
                if Path::new(&("data/images/".to_owned()+ CHARS.0[c]+".txt")).exists() == true{}
                else {
                    // Error message cause a specific file is missing
                    let error_msg = "The '".to_owned() + &("data/images/".to_owned()+ CHARS.0[c]+".txt") + "' file was not found.\nPlease import all character '.txt' files to 'data/images' folder.";
                    print!("\n");
                    panic!("{}", error_msg);
                }
            }
            println!("\nSuccesfully read {} '.txt' files from 'data/images' folder.", CHARS.0.len());
        }
        else{
            // Error message cause 'images' folder doesnt exist
            let error_msg= "The 'data/images' folder was not found.\nPlease import all character '.txt' files to 'data/images' folder.";
            print!("\n");
            panic!("{}", error_msg);
        }

        // If init.json doesnt exist initialize it
        if Path::new("data/init.json").exists() == false {
            commands::update::init_json::init_json();
        }

        // Checking if init.json exists
        if Path::new("data/init.json").exists() == true {

            // Reading init.json file
            let data_from_file = fs::read_to_string("data/init.json")
                .expect("\nFailed to read 'init.json' file.");

            match serde_json::from_str::<Vec<CharInfo>>(&data_from_file) {
                Ok(_) => (println!("\nSuccesfully deserialized 'init.json' file.")),
                Err(_) => (println!("\nFailed to deserialize 'init.json' file.\n\nDelete 'init.json' from the 'data' folder and execute the 'b.update' command.")),
            };

        }

        // Creating the frames folder if it doesnt exist
        if Path::new("data/frames").exists() == false {
            fs::create_dir_all("data/frames")
                .expect("\nFailed to create 'data/frames' directory.");
        }   
        
        // Checking if character folders exist and creates them if not
        for x in 0..CHARS.0.len(){
            let character_folder = "data/frames/".to_owned() + &CHARS.0[x];
            if Path::new(&character_folder).exists() == false {
                fs::create_dir_all(&character_folder)
                    .expect(&("\nFailed to create '".to_owned() + &CHARS.0[x] +"' folder."));
            }
        }

        // Checking if character jsons exist in their respective folder
        for x in 0..CHARS.0.len(){
            let character_json = &("data/frames/".to_owned() + &CHARS.0[x] + "/" + &CHARS.0[x] +".json");
            if Path::new(&character_json).exists() == false{
                print!("\n");
                panic!("Missing '{}'.\nPlease execute the 'b.update' command.", &character_json);
            }
        }

        
    }
    else {
        //Creating the 'data' folder
        fs::create_dir_all("data/images")
            .expect("\nFailed to create 'data' directory.");   
        
        // Error message cause 'images' folder is empty
        let error_msg = "The 'data/images' folder was empty.\nDownload and import the 'data/images' folder from:\nhttps://github.com/yakiimoninja/baiken-bot.";
        print!("\n");
        panic!("{}", error_msg);
    }
}