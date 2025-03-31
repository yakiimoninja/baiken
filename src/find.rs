use aho_corasick::AhoCorasick;
use colored::Colorize;
use rusqlite::{named_params, Connection as SqlConnection};
use crate::{ CharInfo, Error, HitboxLinks, MoveInfo, CHARS};
// Regex related imports
use regex::Regex;
use rusqlite::functions::FunctionFlags;
use rusqlite::{Error as SqlError, Result as SqlResult};
use std::sync::{Arc, Mutex, MutexGuard};
type BoxError = Box<dyn std::error::Error + Send + Sync + 'static>;


/// Searches database for character name from user input.
///
/// Returns `Ok(CHARS[x])` when successful.
pub async fn find_character(character: &str, db: Arc<Mutex<SqlConnection>>) -> Result<(String, usize), Error> {

    // Replace '.' with regex (may contain any number of '.')
    // Replace '-' with regex (may contain any number of '-')
    // Replace any horizontal whitespace char with regex (may contain any number of)
    let patterns = &[" ",".","-","\t"];
    let replace_with = &["[\\s]*","[\\.|\\s]*","[-|\\s]*","[\\s]*"];

    let mut char_regex = Vec::new();

    let ac = AhoCorasick::new(patterns).unwrap();
    ac.try_stream_replace_all(character.trim().to_lowercase().as_bytes(), &mut char_regex, replace_with).unwrap();

    let char_regex = String::from_utf8(char_regex).unwrap();
    let contains_char_regex = ".*".to_owned() + &char_regex + ".*";

    let db = db.lock().unwrap();
    add_regexp_function(&db).unwrap();

    let mut nickname_query = db.prepare("SELECT character_id FROM nicknames WHERE REPLACE(LOWER(nickname), '.', '') REGEXP :char_regex").unwrap();
    let mut name_query = db.prepare("SELECT id FROM characters WHERE REPLACE(LOWER(character), '.', '') REGEXP :contains_char_regex").unwrap();

    // Iterating through the nicknames table entries
    // If user input equals a character nickname then pass the full character name
    match nickname_query.query_row(
        named_params! {":char_regex": char_regex},
        |row| row.get(0)
    ) {
        Ok(char_id) => return Ok((String::from(CHARS[char_id-1]), char_id)),
        Err(_) => {}
    }

    // Iterating through the nicknames.json character entries
    match name_query.query_row(
        named_params! {":contains_char_regex": contains_char_regex},
        |row| row.get(0)
    ) {
        Ok(char_id) => return Ok((String::from(CHARS[char_id-1]), char_id)),
        Err(_) => {}
    }

    // Edge case for update.rs
    if character.trim().to_lowercase() == "all" {
        return Ok((String::from("all"), 0));
    }

    // Error message cause character was not found
    let error_msg= "Character `".to_owned() + character + "` was not found!";
    println!("{}", error_msg.red());
    Err(error_msg.into())
}


/// Searches database for a character move from user input.
pub async fn find_move(char_id: usize, char_move: &str, db: Arc<Mutex<SqlConnection>>) -> Result<(MoveInfo, usize), Error> {

    // Replace '.' with regex (may contain any number of '.')
    // Replace '-' with regex (may contain any number of '-')
    // Replace any horizontal whitespace char with regex (may contain any number of)
    let patterns = &[" ",".","-","\t"];
    let replace_with = &["[\\s]*","[\\.|\\s]*","[-|\\s]*","[\\s]*"];

    let mut move_regex = Vec::new();

    let ac = AhoCorasick::new(patterns).unwrap();
    ac.try_stream_replace_all(char_move.trim().to_lowercase().as_bytes(), &mut  move_regex, replace_with).unwrap();
    let move_regex = String::from_utf8(move_regex).unwrap();

    let db = db.lock().unwrap();
    add_regexp_function(&db).unwrap();

    let mut alias_query = db.prepare("SELECT move_id FROM aliases WHERE move_id IN (SELECT id FROM moves WHERE character_id = :char_id) AND REPLACE(LOWER(alias), '.', '') REGEXP :move_regex ORDER BY id").unwrap();
    let mut input_query = db.prepare("SELECT id FROM moves WHERE character_id = :char_id AND REPLACE(LOWER(input), '.', '') REGEXP :move_regex ORDER BY id").unwrap();
    let mut name_query = db.prepare("SELECT id FROM moves WHERE character_id = :char_id AND REPLACE(LOWER(name), '.', '') REGEXP :move_regex ORDER BY id").unwrap();

    // Checking if user input is alias
    // Semi join
    // https://media.datacamp.com/legacy/v1714587799/Marketing/Blog/Joining_Data_in_SQL_2.pdf
    match alias_query.query_row(
        named_params! {":char_id": char_id, ":move_regex": move_regex},
        |row| row.get(0)
    ) {
        Ok(move_id) => return Ok((send_move(move_id, &db), move_id)),
        Err(_) => {}
    }

    // Checking if user input is move input
    match input_query.query_row(
        named_params! {":char_id": char_id, ":move_regex": move_regex},
        |row| row.get(0)
    ) {
        Ok(move_id) => return Ok((send_move(move_id, &db), move_id)),
        Err(_) => {}
    }

    // Checking if user input is move name
    match name_query.query_row(
        named_params! {":char_id": char_id, ":move_regex": move_regex},
        |row| row.get(0)
    ) {
        Ok(move_id) => return Ok((send_move(move_id, &db), move_id)),
        Err(_) => {}
    }

    // Error message cause given move wasnt found
    let error_msg= "Move `".to_owned() + char_move + "` was not found!";
    println!("{}", error_msg.red());
    Err(error_msg.into())
}


// Searches database for a characters full move list
pub fn find_all_moves(db: Arc<Mutex<SqlConnection>>) {

    // select name, input, type from Moves;
    // select alias, from aliases for move_id.Moves;
    //let db = SqlConnection::open_with_flags("/data/data.db", OpenFlags::SQLITE_OPEN_READ_ONLY).unwrap();

    // prototype query SELECT  moves.id, name, input, move_type, alias from moves left join aliases on aliases.move_id = moves.id where moves.character_id = '' order by moves.id;
    // get move_ids, inputs and names where char_id = 'aba' and movetype = 'whatever'
    // get aliases where move_id = 'moves.move_id'

    let db = db.lock().unwrap();
    let mut q = db.prepare("SELECT input from moves where id = :id").unwrap();
    let move_id: String = q.query_row(
        named_params! {":id": "1"},
        |row| row.get(0)
    ).unwrap();

    println!("P {}", move_id);
}

/// Searches database for a moves hitbox images and caption
pub async fn find_hitboxes(move_id: usize, db: Arc<Mutex<SqlConnection>>) -> Option<Vec<HitboxLinks>> {

    let db = db.lock().unwrap();
    let mut hitbox_query = db.prepare("SELECT hitbox, hitbox_caption FROM hitboxes WHERE move_id = :move_id ORDER BY id").unwrap();
    
    let hitbox_vec = match hitbox_query.query_map(named_params! {":move_id": move_id}, |row| {
        Ok( HitboxLinks {
            hitbox: row.get(0).unwrap(),
            hitbox_caption: row.get(1).unwrap() })})
        {
        Ok(iter) => {
            let mut struct_vec: Vec<HitboxLinks> = Vec::new();

            for hitbox in iter {
                struct_vec.push(hitbox.unwrap());
            }

            Some(struct_vec)
        },
        Err(_) => None
    };

    hitbox_vec
} 


/// Searches database for the given character info.
pub async fn find_info(char_id: usize, db: Arc<Mutex<SqlConnection>>) -> CharInfo {

    let db = db.lock().unwrap();
    let mut info_query = db.prepare("SELECT * FROM info WHERE character_id = :character_id").unwrap();
 
    info_query.query_row(named_params! {":character_id": &char_id},
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
        })).unwrap()
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
