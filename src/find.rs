use std::{fs, path::Path};
use crate::{Nicknames, Error, MoveInfo, MoveAliases};


pub async fn find_character(character: &String) -> Result<String, Error> {

    // Flags that will be used for logic to determine output
    let character_found = false;

    // Reading the nicknames json
    let data_from_file = fs::read_to_string("data/nicknames.json")
        .expect("\nFailed to read 'nicknames.json' file.");
    
    // Deserializing from nicknames json
    let vec_nicknames = serde_json::from_str::<Vec<Nicknames>>(&data_from_file).unwrap();

    // Iterating through the nicknames.json character entries
    if !character_found {
            
        for x_nicknames in &vec_nicknames {

            // Iterating through the nicknames.json nickname entries
            for y_nicknames in &x_nicknames.nicknames {

                // If user input equals a character nickname then pass the full character name
                // To the new variable 'character_arg_altered'
                if y_nicknames.to_lowercase() == character.to_lowercase().trim() {

                    return Ok(x_nicknames.character.to_owned());
                }
            }
        }
    }

    if !character_found {
        
        // Iterating through the nicknames.json character entries
        for x_nicknames in &vec_nicknames {

            // If user input is part of a characters full name or the full name itself
            // Then pass the full and correct charactet name to the new var 'character_arg_altered'
            if x_nicknames.character.to_lowercase().replace('-', "").contains(&character.to_lowercase()) ||
            x_nicknames.character.to_lowercase().contains(&character.to_lowercase()) {
                
                return Ok(x_nicknames.character.to_owned());
            }
        }
    }
    // Edge case for update.rs
    if !character_found && character.trim().to_lowercase() == "all".to_lowercase() {
        return Ok("".into());
    }

    if !character_found {
        // If user input isnt the full name, part of a full name or a nickname
        // Error out cause requested character was not found in the json
        let error_msg= "Character `".to_owned() + &character + "` was not found!";
        Err(error_msg.into())
    }
    else {
        Err("Weird logic error in find_character".into())
    }
}


pub async fn find_move_index(character_arg_altered: &String, mut character_move: String, moves_info: &[MoveInfo]) -> Result<(usize, String), Error> {

    // Flags that will be used for logic to determine output
    let move_found = false;

    // Checking if aliases for this characters moves exist
    let aliases_path = "data/".to_owned() + &character_arg_altered + "/aliases.json";
    if Path::new(&aliases_path).exists() {
        
        // Reading the aliases json
        let aliases_data = fs::read_to_string(&aliases_path)
            .expect(&("\nFailed to read '".to_owned() + &aliases_path + "' file."));
        
        // Deserializing the aliases json
        let aliases_data = serde_json::from_str::<Vec<MoveAliases>>(&aliases_data).unwrap();

        'outer: for alias_data in aliases_data {
            for x_aliases in alias_data.aliases {
                
                // If the requested argument (character_move) is an alias for any of the moves listed in aliases.json
                // Change the given argument (character_move) to the actual move name instead of the alias
                if x_aliases.to_lowercase().trim().replace(['.', ' '], "")
                == character_move.to_lowercase().trim().replace(['.', ' '], "") {

                    character_move = alias_data.input.to_string();
                    break 'outer;
                }
            }
        }
    }

    for (x, moves) in moves_info.iter().enumerate() {
        // Iterating through the moves of the json file to find the move requested
        // Specifically if user arg is exactly move input
        if moves.input.to_string().to_lowercase().replace('.', "") 
        == character_move.to_string().to_lowercase().replace('.', "") {

            return Ok((x,character_move));
        }        
    }

    if !move_found {
        for (x, moves) in moves_info.iter().enumerate() {
            // Iterating through the moves of the json file to find the move requested
            // Specifically if user arg is contained in move name
            if moves.name.to_string().to_lowercase().contains(&character_move.to_string().to_lowercase()) {
                
                return Ok((x, character_move));
            } 
        }
    }

    if !move_found {
        // Error message cause given move wasnt found in the json
        let error_msg= "Move `".to_owned() + &character_move + "` was not found!";
        Err(error_msg.into())
    }
    else {
        Err("Weird logic error in find_move".into())
    }

}