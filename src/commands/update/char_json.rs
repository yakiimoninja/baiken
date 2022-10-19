use std::fs::OpenOptions;
use std::io::Write;
use std::path::Path;
use tokio::fs::remove_file;
use crate::CHARS;
//use crate::Data;

mod source_to_data;
mod data_to_json;
//use crate::char_json::data_to_json::write_data_to_json;

extern crate ureq;

const SITE_LINK: &str = "https://dustloop.com/wiki/api.php?action=parse&page=GGST/";
const SITE_HALF: &str = "/Frame_Data&prop=text&formatversion=2";

pub async fn make_char_json (chars_ids: [&str; CHARS.len()]) {


    print!("\n");

    for x in 0..chars_ids.len() {

        println!("Creating '{}.json' file.", chars_ids[x]);
        
        let char_json_path = "data/".to_owned() + &chars_ids[x] +"/"+ &chars_ids[x] + ".json";

        if Path::new(&char_json_path).exists() == true {
            remove_file(&char_json_path).await.unwrap();
        }

        // Creating character json file
        let mut file = OpenOptions::new()
            .create(true)
            .append(true)
            .open(char_json_path)
            .expect(&("\nFailed to open '".to_owned() + &chars_ids[x] + ".json' file."));

        // More character json file stuff
        let mut char_json_schema = "[\n\t";
        write!(file, "{}", char_json_schema)
            .expect(&("\nFailed to write 'char_json_schema' to '".to_owned() + &chars_ids[x] + ".json'."));

        // Creating request link from init file
        let character_link = SITE_LINK.to_owned() + &chars_ids[x]/*&init_file[x].page.to_string()*/ +  SITE_HALF;

        // Dusloop site request
        let mut char_page_html = ureq::get(&character_link)
            .call()
            .unwrap();
        
        // Because dustloop site 500 a lot
        while char_page_html.status() == 500 {
            char_page_html = ureq::get(&character_link)
                .call()
                .unwrap();
        }

        // Requested website source to file
        let char_page_html = char_page_html.into_string().unwrap();
    
        // Processing the string html source of each characters page
        let moves_info = source_to_data::html_to_data(char_page_html).to_owned();
        
        // Serializing data to json file
        data_to_json::write_data_to_json(moves_info, &file, x);  
        
        // Finalizing character json
        char_json_schema = "\n]";

        let error_msg = "\nFailed to write 'json_schema' to '{".to_owned() + chars_ids[x] + "}.json'.";
        write!(file, "{}", char_json_schema)
            .expect(&error_msg);
    }
}
