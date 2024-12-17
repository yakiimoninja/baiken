extern crate ureq;
use serde::Deserialize;
use rusqlite::{params, Connection as SqlConnection};

#[derive(Deserialize, Debug)]
struct Response {
    cargoquery: Vec<Data>
}

#[derive(Deserialize, Debug)]
struct Data {
    title: Title
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct Title {
    input: Option<String>,
    name: Option<String>,
    damage: Option<String>,
    guard: Option<String>,
    startup: Option<String>,
    active: Option<String>,
    recovery: Option<String>,
    on_hit: Option<String>,
    #[serde(rename = "onBlock")]
    on_block: Option<String>,
    level: Option<String>,
    counter: Option<String>,
    #[serde(rename = "type")]
    move_type: Option<String>,
    #[serde(rename = "riscGain")]
    risc_gain: Option<String>,
    #[serde(rename = "riscLoss")]
    risc_loss: Option<String>,
    #[serde(rename = "wallDamage")]
    wall_damage: Option<String>,
    input_tension: Option<String>,
    chip_ratio: Option<String>,
    #[serde(rename = "OTGRatio")]
    otg_ratio: Option<String>,
    #[serde(rename = "prorate")]
    scaling: Option<String>,
    #[serde(rename = "invuln")]
    invincibility: Option<String>,
    cancel: Option<String>,
    caption: Option<String>,
    notes: Option<String>,
    //images: Option<String>,
    //hitbox_caption: Option<String>,
    //hitboxes: Option<String>,
}

pub async fn frames_to_db(char_page_response_json: &str, db: SqlConnection, char_count: usize) -> SqlConnection {

    let empty = String::from("-");

    let char_page_response_json = &char_page_response_json
        // Colorful text RED
        .replace(r#"&lt;span class=&quot;colorful-text-4&quot; &gt;"#, "")
        // Colorful text BLUE
        .replace(r#"&lt;span class=&quot;colorful-text-2&quot; &gt;"#, "")
        // Colorful text GREEN
        .replace(r#"&lt;span class=&quot;colorful-text-3&quot; &gt;"#, "")
        // Colorful text PURPlE
        .replace(r#"&lt;span class=&quot;colorful-text-1&quot; &gt;"#, "")
        // Colorful text tag close
        .replace(r#"&lt;/span&gt;"#, "")
        .replace(r#"&lt;br&gt;"#, ", ")
        .replace(r#"&lt;br/&gt;"#, ", ")
        .replace(r#"&quot;"#, "")
        // Ino low profile
        .replace(r#" &lt;span class=&quot;tooltip&quot; &gt;Low Profile&lt;span class=&quot;tooltiptext&quot; style=&quot;&quot;&gt;When a character's hurtbox is entirely beneath an opponent's attack. This can be caused by crouching, certain moves, and being short.&lt;/span&gt;&lt;/span&gt;"#, "")
        // Replace apostrophe
        .replace(r#"&#039;"#, "'")
        .replace(r#"&amp;#32;"#, "")
        .replace(r#"'''"#, "")
        .replace(r#"; "#, r#"\n"#)
        .replace(r#";"#, r#"\n"#)
        .replace(r#"\\"#, "");

    let mut move_data_response: Response = serde_json::from_str(char_page_response_json).unwrap();
    let char_move_data = &mut move_data_response.cargoquery;

    for move_data in char_move_data {
        
        // Replacing None values with a generic '-'
        if move_data.title.input.is_none() {
            move_data.title.input = Some("-".to_string());
        }
        else {
            // Skips finish blow for sol
            if *move_data.title.input.as_ref().unwrap() == "j.XX during Homing Jump" {
                continue;
            }
        }
        if move_data.title.name.is_none() {
            move_data.title.name = Some(move_data.title.input.as_ref().unwrap().to_string());
        }
        else {
            // Skips dash cancel entry, ino hoverdash chipp escape zato flight and finish blow 
            if *move_data.title.name.as_ref().unwrap() == "Dash Cancel" || 
            *move_data.title.name.as_ref().unwrap() == "Hoverdash" ||
            *move_data.title.name.as_ref().unwrap() == "Finish Blow" ||
            *move_data.title.name.as_ref().unwrap() == "Flight" ||
            *move_data.title.name.as_ref().unwrap() == "Escape" {
                continue;
            }
        }

        if move_data.title.caption.is_some()
        && (move_data.title.caption.as_ref().unwrap() == "Ground"
        || move_data.title.caption.as_ref().unwrap() == "Air") {
            move_data.title.caption = Some(String::from(""));
        }
        
        db.execute("
INSERT INTO Move_Data
(character_id, input, name, damage, guard, startup, active, recovery, on_hit, on_block, level, counter, move_type, risc_gain, risc_loss, wall_damage, input_tension, chip_ratio, otg_ratio, scaling, invincibility, cancel, caption, notes)
VALUES
(?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?13, ?14, ?15, ?16, ?17, ?18, ?19, ?20, ?21, ?22, ?23, ?24)

ON CONFLICT(character_id, input)
DO UPDATE SET 
character_id = ?1, input = ?2, name = ?3, damage = ?4, guard = ?5, startup = ?6, active = ?7, recovery = ?8, on_hit = ?9, on_block = ?10, level = ?11, counter = ?12, move_type = ?13, risc_gain = ?14, risc_loss = ?15, wall_damage = ?16, input_tension = ?17, chip_ratio = ?18, otg_ratio = ?19, scaling = ?20, invincibility = ?21, cancel = ?22, caption = ?23, notes = ?24",
        params![
            char_count + 1,
            move_data.title.input.as_ref().unwrap_or(&empty).to_string(),
            move_data.title.name.as_ref().unwrap_or(&empty).to_string(),
            move_data.title.damage.as_ref().unwrap_or(&empty).to_string(),
            move_data.title.guard.as_ref().unwrap_or(&empty).to_string(),
            move_data.title.startup.as_ref().unwrap_or(&empty).to_string(),
            move_data.title.active.as_ref().unwrap_or(&empty).to_string(),
            move_data.title.recovery.as_ref().unwrap_or(&empty).to_string(),
            move_data.title.on_hit.as_ref().unwrap_or(&empty).to_string(),
            move_data.title.on_block.as_ref().unwrap_or(&empty).to_string(),
            move_data.title.level.as_ref().unwrap_or(&empty).to_string(),
            move_data.title.counter.as_ref().unwrap_or(&empty).to_string(),
            move_data.title.move_type.as_ref().unwrap_or(&empty).to_string(),
            move_data.title.risc_gain.as_ref().unwrap_or(&empty).to_string(),
            move_data.title.risc_loss.as_ref().unwrap_or(&empty).to_string(),
            move_data.title.wall_damage.as_ref().unwrap_or(&empty).to_string(),
            move_data.title.input_tension.as_ref().unwrap_or(&empty).to_string(),
            move_data.title.chip_ratio.as_ref().unwrap_or(&empty).to_string(),
            move_data.title.otg_ratio.as_ref().unwrap_or(&empty).to_string(),
            move_data.title.scaling.as_ref().unwrap_or(&empty).to_string(),
            move_data.title.invincibility.as_ref().unwrap_or(&empty).to_string(),
            move_data.title.cancel.as_ref().unwrap_or(&empty).to_string(),
            move_data.title.caption.as_ref().unwrap_or(&"".to_string()).to_string(),
            move_data.title.notes.as_ref().unwrap_or(&"".to_string()).to_string(),
        ]).unwrap();
    }

    db
}
