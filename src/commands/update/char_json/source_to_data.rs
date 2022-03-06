//use std::{fs::OpenOptions, io::Write};
//const CHAR_AMOUNT: usize = 19;
const MALFORMED_STRINGS: ([&str; 4], [&str; 4]) = (["Sury","Uimate","Kaes","Sepuura"],["Sultry","Ultimate","Kaltes","Sepultura"]);

pub fn html_to_data(html: String) -> String {
    

    // Processing the string html source of each characters page
    // To extract data from it
    let html: Vec<&str> = html.split(" ").collect();
    let mut html = html.join("\n");
    html = html.replace(r#"&lt;/td>\t\t\t\t&lt;td>"#, "|");
    html = html.replace("&amp;", "");
    html = html.replace("quot;", "");
    html = html.replace("&", "");
    html = html.replace(";", "");
    html = html.replace("lt", "");
    html = html.replace("/div", "");
    html = html.replace("/td", "");
    html = html.replace("gt", "");
    html = html.replace("/tr>", "");
    html = html.replace("\\t>", "");
    html = html.replace("\\n", "");
    html = html.replace(">tr", "");
    html = html.replace(">\\t\\t", "\n");
    html = html.replace("br\n/>", "\n");
    html = html.replacen("text", "", 5);
    html = html.replacen(r#"data-details=\"table"#, "", 1);

    // Debuging html source file
    // let mut file_1 = OpenOptions::new()
    //     .create(true)
    //     .write(true)
    //     .open("html.txt")
    //     .expect("\nFailed to open '");
    // write!(file_1, "{}", html).expect("hello there mister");

    // Creating vector to hold all moves data
    let mut moves_info: Vec<&str> = Vec::new();
    let mut _continue_flag = false;
    
    // Uncomment for image link debugging
    // Creating vector to hold all mmoves image links
    // let mut moves_image_links: Vec<&str> = Vec::new();
    // let mut _link_flag = false;
    
    
    for line in html.lines() {

        // For move information
        if line.contains(&r#"class=\"details-control\">|"#) == true {
            moves_info.push(line);
            _continue_flag = true;
        }
        else if line.contains(r#"data-details=\"table"#) == true || line.contains(r#"/tbody>"#) == true {
            _continue_flag = false;
        }
        else if _continue_flag == true {
            moves_info.push(line);
        }


        // Uncomment for image link debugging
        // if line.contains("1.5x,") == true {
        //     _link_flag = true;
        // }
        // else if line.contains("2x") == true {
        //     _link_flag = false;
        // }
        // else if _link_flag == true && (line.contains("Hitbox.png") == false && line.contains("Hit") == false && line.contains("hitbox") == false && line.contains("HB") == false)
        // && line.contains("/wiki/images/") == true {
        //     moves_image_links.push(line);
        //     _link_flag = false;
        // }
        
    }


    // More formating to extract the moves data
    let mut moves_info = moves_info.join(" ");
    moves_info = moves_info.replace(r#"class=\"details-control\">|"#, "\n");
    moves_info = moves_info.replace(r#"\u00d"#, " ");
    moves_info = moves_info.replace(" \n", ";|");
    moves_info = moves_info.replace("a href=\\\"/wiki/index.php?title=Tiger_Knee\\\" title=\\\"Tiger Knee\\\">TK/a>", "TK");
    moves_info = moves_info.to_owned() + &";";
    let moves_info: Vec<&str> = moves_info.split("\n").collect();
    let mut moves_info = moves_info.join("|");
    moves_info = moves_info.replacen("|", "", 1);

    // Fix for breaking some move names when parsing data from the page source
    for m in 0..MALFORMED_STRINGS.0.len() {
        if moves_info.contains(&MALFORMED_STRINGS.0[m]) == true {
            moves_info = moves_info.replace(&MALFORMED_STRINGS.0[m], &MALFORMED_STRINGS.1[m]);
        }
    }
    
    // Debuging move data file
    // let mut file_1 = OpenOptions::new()
    //     .create(true)
    //     .write(true)
    //     .open("moves.txt")
    //     .expect("\nFailed to open '");
    // write!(file_1, "{}", moves_info).expect("hello there mister");

    
    // Uncomment for image link debugging

    // More formating to extract the respective image links
    // let mut moves_image_links = moves_image_links.join("\n");
    // moves_image_links = moves_image_links.replace(r#"/wiki/images/thumb/"#, "https://dustloop.com/wiki/images/");
    // let moves_image_links: Vec<&str> = moves_image_links.split("\n").collect();
    // let mut final_links: Vec<String> = Vec::new();

    
    // // Creates a new vector with each image link
    // for p in 0..moves_image_links.len()-CHAR_AMOUNT {

    //     // If ".png" exists in the current line
    //     // Pushes the line that contains the image link to the final_links vector
    //     let image_link_offset = moves_image_links[p].find(".png").unwrap_or(moves_image_links[p].len());
    //     let final_links_string: String = moves_image_links[p].to_string().drain(..image_link_offset).collect();
    //     final_links.push(final_links_string);
    // }

    // final_links.push("z".to_string());

    // let mut final_links = final_links.join("\n");
    // final_links = final_links.replace("\n", ".png\n");
    // final_links = final_links.replace("z", "");

    // Debuging image links file
    // let mut file_2 = OpenOptions::new()
    //     .create(true)
    //     .append(true)
    //     .open("links.txt")
    //     .expect("\nFailed to open '");
    // write!(file_2, "{}", final_links).expect("hello there mister");
    
    return moves_info;
}