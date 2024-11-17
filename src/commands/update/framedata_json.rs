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

    let mut moves_info: Response = serde_json::from_str(&char_page_response_json).unwrap();
    let mut vec_processed_moves_info = Vec::new();

    for x in 0..moves_info.cargoquery.len() {
        
        // Replacing None values with a generic '-'
        if moves_info.cargoquery[x].title.input.is_none(){
            moves_info.cargoquery[x].title.input = Some("-".to_string());
        }
        else{
            // Skips finish blow for sol
            if *moves_info.cargoquery[x].title.input.as_ref().unwrap() == "j.XX during Homing Jump" {
                continue;
            }
        }
        if moves_info.cargoquery[x].title.name.is_none(){
            moves_info.cargoquery[x].title.name = Some(moves_info.cargoquery[x].title.input.as_ref().unwrap().to_string());
        }
        else {
            // Skips dash cancel entry, ino hoverdash chipp escape zato flight and finish blow 
            if *moves_info.cargoquery[x].title.name.as_ref().unwrap() == "Dash Cancel" || 
            *moves_info.cargoquery[x].title.name.as_ref().unwrap() == "Hoverdash" ||
            *moves_info.cargoquery[x].title.name.as_ref().unwrap() == "Finish Blow" ||
            *moves_info.cargoquery[x].title.name.as_ref().unwrap() == "Flight" ||
            *moves_info.cargoquery[x].title.name.as_ref().unwrap() == "Escape" {
                continue;
            }
        }

        if moves_info.cargoquery[x].title.damage.is_none(){
            moves_info.cargoquery[x].title.damage = Some("-".to_string());
        }
        if moves_info.cargoquery[x].title.guard.is_none(){
            moves_info.cargoquery[x].title.guard = Some("-".to_string());
        }
        if moves_info.cargoquery[x].title.startup.is_none(){
            moves_info.cargoquery[x].title.startup = Some("-".to_string());
        }
        if moves_info.cargoquery[x].title.active.is_none(){
            moves_info.cargoquery[x].title.active = Some("-".to_string());
        }
        if moves_info.cargoquery[x].title.recovery.is_none(){
            moves_info.cargoquery[x].title.recovery = Some("-".to_string());
        }
        if moves_info.cargoquery[x].title.on_hit.is_none(){
            moves_info.cargoquery[x].title.on_hit = Some("-".to_string());
        }
        if moves_info.cargoquery[x].title.on_block.is_none(){
            moves_info.cargoquery[x].title.on_block = Some("-".to_string());
        }
        if moves_info.cargoquery[x].title.level.is_none(){
            moves_info.cargoquery[x].title.level = Some("-".to_string());
        }
        if moves_info.cargoquery[x].title.counter.is_none(){
            moves_info.cargoquery[x].title.counter = Some("-".to_string());
        }
        if moves_info.cargoquery[x].title.move_type.is_none(){
            moves_info.cargoquery[x].title.move_type = Some("-".to_string());
        }
        if moves_info.cargoquery[x].title.risc_gain.is_none(){
            moves_info.cargoquery[x].title.risc_gain = Some("-".to_string());
        }
        if moves_info.cargoquery[x].title.risc_loss.is_none(){
            moves_info.cargoquery[x].title.risc_loss = Some("-".to_string());
        }
        if moves_info.cargoquery[x].title.scaling.is_none(){
            moves_info.cargoquery[x].title.scaling = Some("-".to_string());
        }
        if moves_info.cargoquery[x].title.wall_damage.is_none(){
            moves_info.cargoquery[x].title.wall_damage = Some("-".to_string());
        }
        if moves_info.cargoquery[x].title.input_tension.is_none(){
            moves_info.cargoquery[x].title.input_tension = Some("-".to_string());
        }
        if moves_info.cargoquery[x].title.chip_ratio.is_none(){
            moves_info.cargoquery[x].title.chip_ratio = Some("-".to_string());
        }
        if moves_info.cargoquery[x].title.otg_ratio.is_none(){
            moves_info.cargoquery[x].title.otg_ratio = Some("-".to_string());
        }
        if moves_info.cargoquery[x].title.scaling.is_none(){
            moves_info.cargoquery[x].title.scaling = Some("-".to_string());
        }
        if moves_info.cargoquery[x].title.invincibility.is_none(){
            moves_info.cargoquery[x].title.invincibility = Some("-".to_string());
        }
        if moves_info.cargoquery[x].title.cancel.is_none(){
            moves_info.cargoquery[x].title.cancel = Some("-".to_string());
        }
        if moves_info.cargoquery[x].title.caption.is_none(){
            moves_info.cargoquery[x].title.caption = Some("-".to_string());
        }
        if moves_info.cargoquery[x].title.notes.is_none(){
            moves_info.cargoquery[x].title.notes = Some("-".to_string());
        }
        if moves_info.cargoquery[x].title.hitbox_caption.is_none(){
            moves_info.cargoquery[x].title.hitbox_caption = Some("-".to_string());
        }

        // Serializing frame data
        let processed_moves_info = MoveInfo {
            input: moves_info.cargoquery[x].title.input.as_ref().unwrap().to_string(),
            name: moves_info.cargoquery[x].title.name.as_ref().unwrap().to_string(),
            damage: moves_info.cargoquery[x].title.damage.as_ref().unwrap().to_string(),
            guard: moves_info.cargoquery[x].title.guard.as_ref().unwrap().to_string(),
            startup: moves_info.cargoquery[x].title.startup.as_ref().unwrap().to_string(),
            active: moves_info.cargoquery[x].title.active.as_ref().unwrap().to_string(),
            recovery: moves_info.cargoquery[x].title.recovery.as_ref().unwrap().to_string(),
            on_hit: moves_info.cargoquery[x].title.on_hit.as_ref().unwrap().to_string(),
            on_block:moves_info.cargoquery[x].title.on_block.as_ref().unwrap().to_string(),
            level:moves_info.cargoquery[x].title.level.as_ref().unwrap().to_string(),
            counter:moves_info.cargoquery[x].title.counter.as_ref().unwrap().to_string(),
            move_type:moves_info.cargoquery[x].title.move_type.as_ref().unwrap().to_string(),
            risc_gain:moves_info.cargoquery[x].title.risc_gain.as_ref().unwrap().to_string(),
            risc_loss:moves_info.cargoquery[x].title.risc_loss.as_ref().unwrap().to_string(),
            wall_damage:moves_info.cargoquery[x].title.wall_damage.as_ref().unwrap().to_string(),
            input_tension:moves_info.cargoquery[x].title.input_tension.as_ref().unwrap().to_string(),
            chip_ratio:moves_info.cargoquery[x].title.chip_ratio.as_ref().unwrap().to_string(),
            otg_ratio: moves_info.cargoquery[x].title.otg_ratio.as_ref().unwrap().to_string(),
            scaling: moves_info.cargoquery[x].title.scaling.as_ref().unwrap().to_string(),
            invincibility:moves_info.cargoquery[x].title.invincibility.as_ref().unwrap().to_string(),
            cancel: moves_info.cargoquery[x].title.cancel.as_ref().unwrap().to_string(),
            caption:moves_info.cargoquery[x].title.caption.as_ref().unwrap().to_string(),
            notes:moves_info.cargoquery[x].title.notes.as_ref().unwrap().to_string(),
            hitbox_caption:moves_info.cargoquery[x].title.hitbox_caption.as_ref().unwrap().to_string(),
        };

        vec_processed_moves_info.push(processed_moves_info);
    }

    file.write_all(&(serde_json::to_vec_pretty(&vec_processed_moves_info).unwrap()))
        .expect(&("\nFailed to serialize '".to_owned() + CHARS[char_count]+ ".json'."));
}
