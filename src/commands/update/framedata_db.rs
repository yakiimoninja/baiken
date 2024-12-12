extern crate ureq;
use serde::Deserialize;
use crate::{CHARS, MoveInfo};
use std::{fs::File, io::Write};

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

async fn remove_tags(mut char_page_response_json: String) -> String {

    char_page_response_json = char_page_response_json
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

    char_page_response_json
}

pub async fn frames_to_json(mut char_page_response_json: String, mut file: &File, char_count: usize) {

    let empty = String::from("-");
    
    char_page_response_json = remove_tags(char_page_response_json).await;

    let mut move_data_response: Response = serde_json::from_str(&char_page_response_json).unwrap();
    let char_move_data = &mut move_data_response.cargoquery;
    let mut vec_processed_moves_info = Vec::new();

    for move_data in char_move_data {
        
        // Replacing None values with a generic '-'
        if move_data.title.input.is_none(){
            move_data.title.input = Some("-".to_string());
        }
        else{
            // Skips finish blow for sol
            if *move_data.title.input.as_ref().unwrap() == "j.XX during Homing Jump" {
                continue;
            }
        }
        if move_data.title.name.is_none(){
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

        // Serializing frame data
        let processed_moves_info = MoveInfo {
            input: move_data.title.input.as_ref().unwrap_or(&empty).to_string(),
            name: move_data.title.name.as_ref().unwrap_or(&empty).to_string(),
            damage: move_data.title.damage.as_ref().unwrap_or(&empty).to_string(),
            guard: move_data.title.guard.as_ref().unwrap_or(&empty).to_string(),
            startup: move_data.title.startup.as_ref().unwrap_or(&empty).to_string(),
            active: move_data.title.active.as_ref().unwrap_or(&empty).to_string(),
            recovery: move_data.title.recovery.as_ref().unwrap_or(&empty).to_string(),
            on_hit: move_data.title.on_hit.as_ref().unwrap_or(&empty).to_string(),
            on_block: move_data.title.on_block.as_ref().unwrap_or(&empty).to_string(),
            level: move_data.title.level.as_ref().unwrap_or(&empty).to_string(),
            counter: move_data.title.counter.as_ref().unwrap_or(&empty).to_string(),
            move_type: move_data.title.move_type.as_ref().unwrap_or(&empty).to_string(),
            risc_gain: move_data.title.risc_gain.as_ref().unwrap_or(&empty).to_string(),
            risc_loss: move_data.title.risc_loss.as_ref().unwrap_or(&empty).to_string(),
            wall_damage: move_data.title.wall_damage.as_ref().unwrap_or(&empty).to_string(),
            input_tension: move_data.title.input_tension.as_ref().unwrap_or(&empty).to_string(),
            chip_ratio: move_data.title.chip_ratio.as_ref().unwrap_or(&empty).to_string(),
            otg_ratio: move_data.title.otg_ratio.as_ref().unwrap_or(&empty).to_string(),
            scaling: move_data.title.scaling.as_ref().unwrap_or(&empty).to_string(),
            invincibility: move_data.title.invincibility.as_ref().unwrap_or(&empty).to_string(),
            cancel: move_data.title.cancel.as_ref().unwrap_or(&empty).to_string(),
            caption: move_data.title.caption.as_ref().unwrap_or(&"".to_string()).to_string(),
            notes: move_data.title.notes.as_ref().unwrap_or(&"".to_string()).to_string(),
            //hitbox_caption: move_data.title.hitbox_caption.as_ref().unwrap_or(&"".to_string()).to_string(),
        };

        vec_processed_moves_info.push(processed_moves_info);
    }

    file.write_all(&(serde_json::to_vec_pretty(&vec_processed_moves_info).unwrap()))
        .expect(&("\nFailed to serialize '".to_owned() + CHARS[char_count]+ ".json'."));
}
