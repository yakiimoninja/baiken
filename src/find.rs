use colored::Colorize;
use rusqlite::{named_params, Connection as SqlConnection, OpenFlags};
use crate::{ CharInfo, Error, HitboxLinks, MoveInfo, CHARS};
// Regex related imports
use regex::Regex;
use rusqlite::functions::FunctionFlags;
use rusqlite::{Error as SqlError, Result as SqlResult};
use std::sync::Arc;
type BoxError = Box<dyn std::error::Error + Send + Sync + 'static>;


/// Searches inside `data.db` database for character name from user input.
///
/// Returns `Ok(CHARS[x])` when successful.
pub async fn find_character(character: &str ) -> Result<(String, usize), Error> {

    let char_regex = character.to_lowercase().trim()
        // Replace '.' with regex (may contain any number of '.')
        .replace(".", "[\\.]*")
        // Replace '-' with regex (may contain any number of '-')
        .replace("-", "[-]*")
        // Replace any horizontal whitespace char with regex (may contain any number of)
        .replace(" ", "[\\s|\\t]*")
        .replace("\t", "[\\s|\\t]*");

    let contains_char_regex = ".*".to_owned() + &char_regex + ".*";

    let db = SqlConnection::open_with_flags("data/data.db", OpenFlags::SQLITE_OPEN_READ_ONLY).unwrap();
    add_regexp_function(&db).unwrap();

    let mut nickname_query = db.prepare("SELECT character_id FROM nicknames WHERE REPLACE(LOWER(nickname), '.', '') REGEXP :char_regex").unwrap();
    let mut name_query = db.prepare("SELECT id FROM characters WHERE REPLACE(LOWER(character), '.', '') REGEXP :contains_char_regex").unwrap();

    // Iterating through the nicknames table entries
    // If user input equals a character nickname then pass the full character name
    if nickname_query.exists(named_params! {":char_regex": char_regex}).unwrap() {

        let char_id: usize = nickname_query.query_row(
            named_params! {":char_regex": char_regex},
            |row| row.get(0)
        ).unwrap();

        return Ok((CHARS[char_id-1].to_string(), char_id));
    }
    // Iterating through the nicknames.json character entries
    else if name_query.exists(named_params! {":contains_char_regex": contains_char_regex}).unwrap(){
        
        let char_id: usize = name_query.query_row(
            named_params! {":contains_char_regex": contains_char_regex},
            |row| row.get(0)
        ).unwrap();

        return Ok((CHARS[char_id-1].to_string(), char_id));
    }
    // Edge case for update.rs
    else if character.trim().to_lowercase() == "all" {
        return Ok((String::from("all"), 0));
    }

    // Error message cause character was not found
    let error_msg= "Character `".to_owned() + character + "` was not found!";
    println!("{}", error_msg.red());
    Err(error_msg.into())
}


/// Searches inside `data.db` database for character move from user input.
pub async fn find_move(char_id: usize, char_move: &str) -> Result<(MoveInfo, usize), Error> {

    let move_regex = char_move.to_lowercase().trim()
        // Replace '.' with regex (may contain any number of '.')
        .replace(".", "[\\.]*")
        // Replace '-' with regex (may contain any number of '-')
        .replace("-", "[-]*")
        // Replace any horizontal whitespace char with regex (may contain any number of)
        .replace(" ", "[\\s|\\t]*")
        .replace("\t", "[\\s|\\t]*");

    let db = SqlConnection::open_with_flags("data/data.db", OpenFlags::SQLITE_OPEN_READ_ONLY).unwrap();
    add_regexp_function(&db).unwrap();

    let mut alias_query = db.prepare("SELECT move_id FROM aliases WHERE move_id IN (SELECT id FROM moves WHERE character_id = :char_id) AND REPLACE(LOWER(alias), '.', '') REGEXP :move_regex ORDER BY id").unwrap();
    let mut input_query = db.prepare("SELECT id FROM moves WHERE character_id = :char_id AND REPLACE(LOWER(input), '.', '') REGEXP :move_regex ORDER BY id").unwrap();
    let mut name_query = db.prepare("SELECT id FROM moves WHERE character_id = :char_id AND REPLACE(LOWER(name), '.', '') REGEXP :move_regex ORDER BY id").unwrap();

    // Checking if user input is alias
    if alias_query.exists(named_params! {":char_id": char_id, ":move_regex": move_regex}).unwrap() {
        // Semi join
        // https://media.datacamp.com/legacy/v1714587799/Marketing/Blog/Joining_Data_in_SQL_2.pdf
        let move_id: usize = alias_query.query_row(
            named_params! {":char_id": char_id, ":move_regex": move_regex},
            |row| row.get(0)
        ).unwrap();

        println!("Move found in aliases.alias");
        return Ok((send_move(move_id), move_id));
    }
    // Checking if user input is move input
    else if input_query.exists(named_params! {":char_id": char_id, ":move_regex": move_regex}).unwrap() {

        let move_id: usize = input_query.query_row(
            named_params! {":char_id": char_id, ":move_regex": move_regex},
            |row| row.get(0)
        ).unwrap();

        println!("Move found in moves.input");
        return Ok((send_move(move_id), move_id));
    }
    // Checking if user input is move name
    else if name_query.exists(named_params! {":char_id": char_id, ":move_regex": move_regex}).unwrap() {

        let move_id: usize = name_query.query_row(
            named_params! {":char_id": char_id, ":move_regex": move_regex},
            |row| row.get(0)
        ).unwrap();

        println!("Move found in moves.name");
        return Ok((send_move(move_id), move_id));
    }

    // Error message cause given move wasnt found
    let error_msg= "Move `".to_owned() + char_move + "` was not found!";
    println!("{}", error_msg.red());
    Err(error_msg.into())
}


pub async fn find_hitboxes(move_id: usize) -> Option<Vec<HitboxLinks>> {

    let db = SqlConnection::open_with_flags("data/data.db", OpenFlags::SQLITE_OPEN_READ_ONLY).unwrap();

    let mut hitbox_query = db.prepare("SELECT hitbox, hitbox_caption FROM hitboxes WHERE move_id = :move_id ORDER BY id").unwrap();
    
    if hitbox_query.exists(named_params! {":move_id": move_id}).unwrap() {

        let hitbox_iter = hitbox_query.query_map(named_params! {":move_id": move_id}, |row| {
            Ok( HitboxLinks {
                hitbox: row.get(0).unwrap(),
                hitbox_caption: row.get(1).unwrap()
            })
        }).unwrap();
        
        let mut struct_vec: Vec<HitboxLinks> = Vec::new();

        for hitbox in hitbox_iter {
            struct_vec.push(hitbox.unwrap());
        }

        return Some(struct_vec);
    }
    
    None
} 


/// Searches for the given character info.
pub async fn find_info(char_id: usize) -> CharInfo {

    let db = SqlConnection::open_with_flags("data/data.db", OpenFlags::SQLITE_OPEN_READ_ONLY).unwrap();
    let mut info_query = db.prepare("SELECT * FROM info WHERE character_id = :character_id").unwrap();
    let char_info = info_query.query_row(named_params! {":character_id": &char_id},
        |row| Ok( CharInfo {
            defense: row.get(2).unwrap(),
            guts: row.get(3).unwrap(),
            guard_balance: row.get(4).unwrap(),
            prejump: row.get(5).unwrap(),
            umo: row.get(6).unwrap(),
            forward_dash: row.get(7).unwrap(),
            backdash: row.get(8).unwrap(),
            backdash_duration: row.get(9).unwrap(),
            backdash_invincibility: row.get(10).unwrap(),
            backdash_airborne: row.get(11).unwrap(),
            backdash_distance: row.get(12).unwrap(),
            jump_duration: row.get(13).unwrap(),
            jump_height: row.get(14).unwrap(),
            high_jump_duration: row.get(15).unwrap(),
            high_jump_height: row.get(16).unwrap(),
            earliest_iad: row.get(17).unwrap(),
            ad_duration: row.get(18).unwrap(),
            ad_distance: row.get(19).unwrap(),
            abd_duration: row.get(20).unwrap(),
            abd_distance: row.get(21).unwrap(),
            movement_tension: row.get(22).unwrap(),
            jump_tension: row.get(23).unwrap(),
            airdash_tension: row.get(24).unwrap(),
            walk_speed: row.get(25).unwrap(),
            back_walk_speed: row.get(26).unwrap(),
            dash_initial_speed: row.get(27).unwrap(),
            dash_acceleration: row.get(28).unwrap(),
            dash_friction: row.get(29).unwrap(),
            jump_gravity: row.get(30).unwrap(),
            high_jump_gravity: row.get(31).unwrap()
        })).unwrap();

    println!("{:#?}", char_info);
    char_info
}


/// Returns a `MoveInfo` struct given a `move_id` from a SQL query.
fn send_move(move_id: usize) -> MoveInfo {

    let db = SqlConnection::open_with_flags("data/data.db", OpenFlags::SQLITE_OPEN_READ_ONLY).unwrap();
    
    db.query_row("SELECT input, name, damage, guard, startup, active, recovery, on_hit, on_block, level, counter, move_type, risc_gain, risc_loss, wall_damage, input_tension, chip_ratio, otg_ratio, scaling, invincibility, cancel, caption, notes, image FROM moves WHERE id = :move_id",
        named_params! {":move_id": move_id},
        |row| Ok( MoveInfo {
            input: row.get(0).unwrap(),
            name: row.get(1).unwrap(),
            damage: row.get(2).unwrap(),
            guard: row.get(3).unwrap(),
            startup: row.get(4).unwrap(),
            active: row.get(5).unwrap(),
            recovery: row.get(6).unwrap(),
            on_hit: row.get(7).unwrap(),
            on_block: row.get(8).unwrap(),
            level: row.get(9).unwrap(),
            counter: row.get(10).unwrap(),
            move_type: row.get(11).unwrap(),
            risc_gain: row.get(12).unwrap(),
            risc_loss: row.get(13).unwrap(),
            wall_damage: row.get(14).unwrap(),
            input_tension: row.get(15).unwrap(),
            chip_ratio: row.get(16).unwrap(),
            otg_ratio: row.get(17).unwrap(),
            scaling: row.get(18).unwrap(),
            invincibility: row.get(19).unwrap(),
            cancel: row.get(20).unwrap(),
            caption: row.get(21).unwrap(),
            notes: row.get(22).unwrap(),
            image: row.get(23).unwrap(),
        })
    ).unwrap()

}


/// Adds regex functionality to sql queries
fn add_regexp_function(db: &SqlConnection) -> SqlResult<()> {
    db.create_scalar_function(
        "regexp",
        2,
        FunctionFlags::SQLITE_UTF8 | FunctionFlags::SQLITE_DETERMINISTIC,
        move |ctx| {
            assert_eq!(ctx.len(), 2, "called with unexpected number of arguments");
            let regexp: Arc<Regex> = ctx.get_or_create_aux(0, |vr| -> SqlResult<_, BoxError> {
                Ok(Regex::new(vr.as_str()?)?)
            })?;
            let is_match = {
                let text = ctx
                    .get_raw(1)
                    .as_str()
                    .map_err(|e| SqlError::UserFunctionError(e.into()))?;

                regexp.is_match(text)
            };

            Ok(is_match)
        },
    )
}
