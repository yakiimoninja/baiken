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

    for x in 0..char_move_data.len() {
        
        // Replacing None values with a generic '-'
        if char_move_data[x].title.input.is_none(){
            char_move_data[x].title.input = Some("-".to_string());
        }
        else{
            // Skips finish blow for sol
            if *char_move_data[x].title.input.as_ref().unwrap() == "j.XX during Homing Jump" {
                continue;
            }
        }
        if char_move_data[x].title.name.is_none(){
            char_move_data[x].title.name = Some(char_move_data[x].title.input.as_ref().unwrap().to_string());
        }
        else {
            // Skips dash cancel entry, ino hoverdash chipp escape zato flight and finish blow 
            if *char_move_data[x].title.name.as_ref().unwrap() == "Dash Cancel" || 
            *char_move_data[x].title.name.as_ref().unwrap() == "Hoverdash" ||
            *char_move_data[x].title.name.as_ref().unwrap() == "Finish Blow" ||
            *char_move_data[x].title.name.as_ref().unwrap() == "Flight" ||
            *char_move_data[x].title.name.as_ref().unwrap() == "Escape" {
                continue;
            }
        }

        // Serializing frame data
        let processed_moves_info = MoveInfo {
            input: char_move_data[x].title.input.as_ref().unwrap_or(&"-".to_string()).to_string(),
            name: char_move_data[x].title.name.as_ref().unwrap_or(&"-".to_string()).to_string(),
            damage: char_move_data[x].title.damage.as_ref().unwrap_or(&"-".to_string()).to_string(),
            guard: char_move_data[x].title.guard.as_ref().unwrap_or(&"-".to_string()).to_string(),
            startup: char_move_data[x].title.startup.as_ref().unwrap_or(&"-".to_string()).to_string(),
            active: char_move_data[x].title.active.as_ref().unwrap_or(&"-".to_string()).to_string(),
            recovery: char_move_data[x].title.recovery.as_ref().unwrap_or(&"-".to_string()).to_string(),
            on_hit: char_move_data[x].title.on_hit.as_ref().unwrap_or(&"-".to_string()).to_string(),
            on_block: char_move_data[x].title.on_block.as_ref().unwrap_or(&"-".to_string()).to_string(),
            level: char_move_data[x].title.level.as_ref().unwrap_or(&"-".to_string()).to_string(),
            counter: char_move_data[x].title.counter.as_ref().unwrap_or(&"-".to_string()).to_string(),
            move_type: char_move_data[x].title.move_type.as_ref().unwrap_or(&"-".to_string()).to_string(),
            risc_gain: char_move_data[x].title.risc_gain.as_ref().unwrap_or(&"-".to_string()).to_string(),
            risc_loss: char_move_data[x].title.risc_loss.as_ref().unwrap_or(&"-".to_string()).to_string(),
            wall_damage: char_move_data[x].title.wall_damage.as_ref().unwrap_or(&"-".to_string()).to_string(),
            input_tension: char_move_data[x].title.input_tension.as_ref().unwrap_or(&"-".to_string()).to_string(),
            chip_ratio: char_move_data[x].title.chip_ratio.as_ref().unwrap_or(&"-".to_string()).to_string(),
            otg_ratio: char_move_data[x].title.otg_ratio.as_ref().unwrap_or(&"-".to_string()).to_string(),
            scaling: char_move_data[x].title.scaling.as_ref().unwrap_or(&"-".to_string()).to_string(),
            invincibility: char_move_data[x].title.invincibility.as_ref().unwrap_or(&"-".to_string()).to_string(),
            cancel: char_move_data[x].title.cancel.as_ref().unwrap_or(&"-".to_string()).to_string(),
            caption: char_move_data[x].title.caption.as_ref().unwrap_or(&"-".to_string()).to_string(),
            notes: char_move_data[x].title.notes.as_ref().unwrap_or(&"-".to_string()).to_string(),
            hitbox_caption: char_move_data[x].title.hitbox_caption.as_ref().unwrap_or(&"-".to_string()).to_string(),
        };

        vec_processed_moves_info.push(processed_moves_info);
    }

    file.write_all(&(serde_json::to_vec_pretty(&vec_processed_moves_info).unwrap()))
        .expect(&("\nFailed to serialize '".to_owned() + CHARS[char_count]+ ".json'."));
}
