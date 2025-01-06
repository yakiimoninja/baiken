use std::{fs, path::Path};
use rusqlite::{named_params, Connection as SqlConnection};
use crate::{update::update_all_char_data, Error, MoveAliases, Nicknames, CHARS};

/// Creates `data.db` database.
pub async fn create_db() -> Result<(), Error> {

    let schema_path = "data/schema.sql";
    let nicknames_path = "data/utils/nicknames.json";

    // check for schema.sql
    // Checking if gids db exists
    if !Path::new(&schema_path).exists() {
        // Error message cause schema does not exist
        let error_msg = "Failed to open 'schema.sql' file.";
        return Err(error_msg.into());
    }
    // utils directory
    if !Path::new("data/utils/").exists() {
        let error_msg = "Failed to open 'utils' directory.";
        return Err(error_msg.into());
    }
    // nicknames json
    if !Path::new(nicknames_path).exists() {
        let error_msg = "Failed to open 'nicknames.json' file.";
        return Err(error_msg.into());
    }
    // aliases directory
    if !Path::new("data/utils/aliases/").exists() {
        let error_msg = "Failed to open 'aliases' directory.";
        return Err(error_msg.into());
    }
    // aliases json
    for character in CHARS {
        let aliases_path = "data/utils/aliases/".to_owned() + &character.replace(" ", "_") + ".json";
        if !Path::new(&aliases_path).exists() {
            println!("{}", aliases_path);
            let error_msg = "Failed to open '".to_owned() + character + ".json' file.";
            return Err(error_msg.into());
        }
    }

    let db = SqlConnection::open("data/data.db").unwrap();

    // execute schema
    let schema = fs::read_to_string(schema_path).unwrap();
    db.execute_batch(&schema).unwrap();

    // insert characters
    let nicknames_data = fs::read_to_string(nicknames_path).unwrap();
    let nicknames_data = serde_json::from_str::<Vec<Nicknames>>(&nicknames_data).unwrap();

    for nick_data in nicknames_data.iter() {

        db.execute("INSERT INTO characters (character) VALUES (:character)",
            named_params! {
                ":character": nick_data.character
            }).unwrap();

        // insert nicknames
        for nickname in nick_data.nicknames.iter() {

            println!("char: {}, nickname: {}",
                nick_data.character, nickname);
            db.execute("
INSERT INTO nicknames (character_id, nickname)
VALUES ((SELECT characters.id FROM characters WHERE characters.character = :character), :nickname)", 
                named_params! {
                    ":character": nick_data.character,
                    ":nickname": nickname
                }).unwrap();
        }
    }
    // do update all
    update_all_char_data().await;

    // insert aliases
    for (x, char) in CHARS.iter().enumerate() {
        // Reading the aliases json
        let aliases_path = "data/utils/aliases/".to_owned() + &char.replace(" ", "_") + ".json";
        let aliases_data = fs::read_to_string(&aliases_path).unwrap();
        
        // Deserializing the aliases json
        let aliases_data = serde_json::from_str::<Vec<MoveAliases>>(&aliases_data).unwrap();

        for alias_data in aliases_data {
            for alias in alias_data.aliases {
                
                // If the requested argument (character_move) is an alias for any of the moves listed in aliases.json
                // Change the given argument (character_move) to the actual move name instead of the alias
                println!("{}, {}, {}", x+1, alias_data.input, alias.trim());
                db.execute("
INSERT INTO aliases (move_id, alias)
VALUES 
((SELECT id FROM moves WHERE character_id = :character_id AND input = :input), :alias)",
                named_params! {
                    ":character_id": x+1,
                    ":input": alias_data.input,
                    ":alias": alias.trim()
                }).unwrap();
            }
        }
    }

    Ok(())
}

/// Creates the `gid.db` database.
pub async fn create_gid_db() -> Result<(), Error> {

    let db = SqlConnection::open("data/gids.db").unwrap();
    db.execute(r#"
CREATE TABLE IF NOT EXISTS "gids" (
	"id" INTEGER NOT NULL UNIQUE,
	"gid" TEXT NOT NULL UNIQUE,
	PRIMARY KEY ("id")
)"#, ()).unwrap();

    Ok(())
}
