extern crate ureq;
use serde::Deserialize;
use crate::{CHARS, MoveInfo};
use std::{
    fs::File,
    io::Write,
};

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
    hitbox_caption: Option<String>,
    //images: Option<String>,
    //hitboxes: Option<String>,
}

pub async fn frames_to_json(mut char_page_response_json: String, mut file: &File, char_count: usize) {

    char_page_response_json = char_page_response_json.replace(r#"&lt;br&gt;"#, ", ");
    char_page_response_json = char_page_response_json.replace(r#"&lt;br/&gt;"#, ", ");
    // Ino low profile
    char_page_response_json = char_page_response_json.replace(r#" &lt;span class=&quot;tooltip&quot; &gt;Low Profile&lt;span class=&quot;tooltiptext&quot; style=&quot;&quot;&gt;When a character's hurtbox is entirely beneath an opponent's attack. This can be caused by crouching, certain moves, and being short.&lt;/span&gt;&lt;/span&gt;"#, "");
    // Replace apostrophe
    char_page_response_json = char_page_response_json.replace(r#"&#039;"#, "'");

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

        // Serializing frame data
        let processed_moves_info = MoveInfo {
            input: move_data.title.input.get_or_insert_with(|| String::from("-")).to_string(),
            name: move_data.title.name.get_or_insert_with(|| String::from("-")).to_string(),
            damage: move_data.title.damage.get_or_insert_with(|| String::from("-")).to_string(),
            guard: move_data.title.guard.get_or_insert_with(|| String::from("-")).to_string(),
            startup: move_data.title.startup.get_or_insert_with(|| String::from("-")).to_string(),
            active: move_data.title.active.get_or_insert_with(|| String::from("-")).to_string(),
            recovery: move_data.title.recovery.get_or_insert_with(|| String::from("-")).to_string(),
            on_hit: move_data.title.on_hit.get_or_insert_with(|| String::from("-")).to_string(),
            on_block: move_data.title.on_block.get_or_insert_with(|| String::from("-")).to_string(),
            level: move_data.title.level.get_or_insert_with(|| String::from("-")).to_string(),
            counter: move_data.title.counter.get_or_insert_with(|| String::from("-")).to_string(),
            move_type: move_data.title.move_type.get_or_insert_with(|| String::from("-")).to_string(),
            risc_gain: move_data.title.risc_gain.get_or_insert_with(|| String::from("-")).to_string(),
            risc_loss: move_data.title.risc_loss.get_or_insert_with(|| String::from("-")).to_string(),
            wall_damage: move_data.title.wall_damage.get_or_insert_with(|| String::from("-")).to_string(),
            input_tension: move_data.title.input_tension.get_or_insert_with(|| String::from("-")).to_string(),
            chip_ratio: move_data.title.chip_ratio.get_or_insert_with(|| String::from("-")).to_string(),
            otg_ratio: move_data.title.otg_ratio.get_or_insert_with(|| String::from("-")).to_string(),
            scaling: move_data.title.scaling.get_or_insert_with(|| String::from("-")).to_string(),
            invincibility: move_data.title.invincibility.get_or_insert_with(|| String::from("-")).to_string(),
            cancel: move_data.title.cancel.get_or_insert_with(|| String::from("-")).to_string(),
            caption: move_data.title.caption.get_or_insert_with(|| String::from("-")).to_string(),
            notes: move_data.title.notes.get_or_insert_with(|| String::from("-")).to_string(),
            hitbox_caption: move_data.title.hitbox_caption.get_or_insert_with(|| String::from("-")).to_string(),
        };

        vec_processed_moves_info.push(processed_moves_info);
    }

    file.write_all(&(serde_json::to_vec_pretty(&vec_processed_moves_info).unwrap()))
        .expect(&("\nFailed to serialize '".to_owned() + CHARS[char_count]+ ".json'."));
}
