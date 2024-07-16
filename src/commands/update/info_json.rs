use std::io::Write;
use serde::Deserialize;
use crate::{CHARS, CharInfo};
use std::fs::File;
extern crate ureq;


#[derive(Deserialize, Debug)]
struct Response {
    cargoquery: Vec<Data>
}

#[derive(Deserialize, Debug)]
struct Data {
    title: Title
}

#[derive(Deserialize, Debug)]
struct Title {
    defense: Option<String>,
    guts: Option<String>,
    #[serde(rename = "guardBalance")]
    guardbalance: Option<String>,
    prejump: Option<String>,
    backdash: Option<String>,
    forwarddash: Option<String>,
    umo: Option<String>,
    #[serde(rename = "jump duration")]
    jump_duration: Option<String>,
    #[serde(rename = "high jump duration")]
    high_jump_duration: Option<String>,
    #[serde(rename = "jump height")]
    jump_height: Option<String>,
    #[serde(rename = "high jump height")]
    high_jump_height: Option<String>,
    #[serde(rename = "earliest iad")]
    earliest_iad: Option<String>,
    #[serde(rename = "ad duration")]
    ad_duration: Option<String>,
    #[serde(rename = "abd duration")]
    abd_duration: Option<String>,
    #[serde(rename = "ad distance")]
    ad_distance: Option<String>,
    #[serde(rename = "abd distance")]
    abd_distance: Option<String>,
    #[serde(rename = "movement tension")]
    movement_tension: Option<String>,
    #[serde(rename = "jump tension")]
    jump_tension: Option<String>,
    #[serde(rename = "airdash tension")]
    airdash_tension: Option<String>,
    #[serde(rename = "walk speed")]
    walk_speed: Option<String>,
    #[serde(rename = "back walk speed")]
    back_walk_speed: Option<String>,
    #[serde(rename = "dash initial speed")]
    dash_initial_speed: Option<String>,
    #[serde(rename = "dash acceleration")]
    dash_acceleration: Option<String>,
    #[serde(rename = "dash friction")]
    dash_friction: Option<String>,
    #[serde(rename = "jump gravity")]
    jump_gravity: Option<String>,
    #[serde(rename = "high jump gravity")]
    high_jump_gravity: Option<String>,
}

pub async fn info_to_json(mut char_info_response_json: String, mut file: &File, char_count: usize){

    char_info_response_json = char_info_response_json.replace(r#"&lt;br&gt;"#, ", ");
    char_info_response_json = char_info_response_json.replace(r#"&lt;br/&gt;"#, ", ");
    // Ino low profile
    char_info_response_json = char_info_response_json.replace(r#" &lt;span class=&quot;tooltip&quot; &gt;Low Profile&lt;span class=&quot;tooltiptext&quot; style=&quot;&quot;&gt;When a character's hurtbox is entirely beneath an opponent's attack. This can be caused by crouching, certain moves, and being short.&lt;/span&gt;&lt;/span&gt;"#, "");
    // Replace apostrophe
    char_info_response_json = char_info_response_json.replace(r#"&#039;"#, "'");

    let mut char_info: Response = serde_json::from_str(&char_info_response_json).unwrap();


    if char_info.cargoquery[0].title.defense.is_none(){
        char_info.cargoquery[0].title.defense = Some("-".to_string());
    }
    if char_info.cargoquery[0].title.guts.is_none(){
    char_info.cargoquery[0].title.guts = Some("-".to_string());
    }
    if char_info.cargoquery[0].title.guardbalance.is_none(){
        char_info.cargoquery[0].title.guardbalance = Some("-".to_string());
    }
    if char_info.cargoquery[0].title.prejump.is_none(){
        char_info.cargoquery[0].title.prejump = Some("-".to_string());
    }
    if char_info.cargoquery[0].title.backdash.is_none(){
        char_info.cargoquery[0].title.backdash = Some("-".to_string());
    }
    if char_info.cargoquery[0].title.forwarddash.is_none(){
        char_info.cargoquery[0].title.forwarddash = Some("-".to_string());
    }

    let mut umo_processed_vec: Vec<String> = Vec::new();
    let mut _umo_processed_string: String = String::new();
    
    if char_info.cargoquery[0].title.umo.is_none(){
        _umo_processed_string = "-".to_string();
    }
    // Logic for removing [[GGST/Character/UMO|UMO]] padding
    else if char_info.cargoquery[0].title.umo.as_ref().unwrap().to_string().contains("[[GGST") {
        
        // Split umo entries by comma
        let split_umo: Vec<&str> = char_info.cargoquery[0].title.umo
            .as_mut()
            .unwrap()
            .split(',')
            .collect();
        
        // Removing [[GGST/Character/UMO|UMO]] and getting umo
        for umo in split_umo {
            if umo.contains("[[GGST") {
                let split_padding_umo: Vec<&str> =   umo.rsplit('|').collect();  
                umo_processed_vec.push(split_padding_umo[0].to_string().replace("]]", ""));
            } 
            else {
                umo_processed_vec.push(umo.to_string())
            }     
        }
        _umo_processed_string = umo_processed_vec.join(", ");
    }
    // Logic for case of [[GGST/Character/UMO|UMO]] not existing
    else {
        _umo_processed_string = char_info.cargoquery[0].title.umo.as_ref().unwrap().to_string();
    }

    if char_info.cargoquery[0].title.jump_duration.is_none(){
        char_info.cargoquery[0].title.jump_duration = Some("-".to_string());
    }
    if char_info.cargoquery[0].title.high_jump_duration.is_none(){
        char_info.cargoquery[0].title.high_jump_duration = Some("-".to_string());
    }
    if char_info.cargoquery[0].title.jump_height.is_none(){
        char_info.cargoquery[0].title.jump_height = Some("-".to_string());
    }
    if char_info.cargoquery[0].title.high_jump_height.is_none(){
        char_info.cargoquery[0].title.high_jump_height = Some("-".to_string());
    }
    if char_info.cargoquery[0].title.earliest_iad.is_none(){
        char_info.cargoquery[0].title.earliest_iad = Some("-".to_string());
    }
    if char_info.cargoquery[0].title.ad_duration.is_none(){
        char_info.cargoquery[0].title.ad_duration = Some("-".to_string());
    }
    if char_info.cargoquery[0].title.abd_duration.is_none(){
        char_info.cargoquery[0].title.abd_duration = Some("-".to_string());
    }
    if char_info.cargoquery[0].title.ad_distance.is_none(){
        char_info.cargoquery[0].title.ad_distance = Some("-".to_string());
    }
    if char_info.cargoquery[0].title.abd_distance.is_none(){
        char_info.cargoquery[0].title.abd_distance = Some("-".to_string());
    }
    if char_info.cargoquery[0].title.movement_tension.is_none(){
        char_info.cargoquery[0].title.movement_tension = Some("-".to_string());
    }
    if char_info.cargoquery[0].title.jump_tension.is_none(){
        char_info.cargoquery[0].title.jump_tension = Some("-".to_string());
    }
    if char_info.cargoquery[0].title.airdash_tension.is_none(){
        char_info.cargoquery[0].title.airdash_tension = Some("-".to_string());
    }
    if char_info.cargoquery[0].title.walk_speed.is_none(){
        char_info.cargoquery[0].title.walk_speed = Some("-".to_string());
    }
    if char_info.cargoquery[0].title.back_walk_speed.is_none(){
        char_info.cargoquery[0].title.back_walk_speed = Some("-".to_string());
    }
    if char_info.cargoquery[0].title.dash_initial_speed.is_none(){
        char_info.cargoquery[0].title.dash_initial_speed = Some("-".to_string());
    }
    if char_info.cargoquery[0].title.dash_acceleration.is_none(){
        char_info.cargoquery[0].title.dash_acceleration = Some("-".to_string());
    }
    if char_info.cargoquery[0].title.dash_friction.is_none(){
        char_info.cargoquery[0].title.dash_friction = Some("-".to_string());
    }
    if char_info.cargoquery[0].title.jump_gravity.is_none(){
        char_info.cargoquery[0].title.jump_gravity = Some("-".to_string());
    }
    if char_info.cargoquery[0].title.high_jump_gravity.is_none(){
        char_info.cargoquery[0].title.high_jump_gravity = Some("-".to_string());
    }

    // Serializing character info
    let processed_char_info = serde_json::to_string_pretty(&CharInfo {
        defense: char_info.cargoquery[0].title.defense.as_ref().unwrap().to_string(),
        guts: char_info.cargoquery[0].title.guts.as_ref().unwrap().to_string(),
        guardbalance: char_info.cargoquery[0].title.guardbalance.as_ref().unwrap().to_string(),
        prejump: char_info.cargoquery[0].title.prejump.as_ref().unwrap().to_string(),
        backdash: char_info.cargoquery[0].title.backdash.as_ref().unwrap().to_string(),
        forwarddash: char_info.cargoquery[0].title.forwarddash.as_ref().unwrap().to_string(),
        umo: _umo_processed_string,
        jump_duration: char_info.cargoquery[0].title.jump_duration.as_ref().unwrap().to_string(),
        high_jump_duration: char_info.cargoquery[0].title.high_jump_duration.as_ref().unwrap().to_string(),
        jump_height: char_info.cargoquery[0].title.jump_height.as_ref().unwrap().to_string(),
        high_jump_height: char_info.cargoquery[0].title.high_jump_height.as_ref().unwrap().to_string(),
        earliest_iad: char_info.cargoquery[0].title.earliest_iad.as_ref().unwrap().to_string(),
        ad_duration: char_info.cargoquery[0].title.ad_duration.as_ref().unwrap().to_string(),
        abd_duration: char_info.cargoquery[0].title.abd_duration.as_ref().unwrap().to_string(),
        ad_distance: char_info.cargoquery[0].title.ad_distance.as_ref().unwrap().to_string(),
        abd_distance: char_info.cargoquery[0].title.abd_distance.as_ref().unwrap().to_string(),
        movement_tension: char_info.cargoquery[0].title.movement_tension.as_ref().unwrap().to_string(),
        jump_tension: char_info.cargoquery[0].title.jump_tension.as_ref().unwrap().to_string(),
        airdash_tension: char_info.cargoquery[0].title.airdash_tension.as_ref().unwrap().to_string(),
        walk_speed: char_info.cargoquery[0].title.walk_speed.as_ref().unwrap().to_string(),
        back_walk_speed: char_info.cargoquery[0].title.back_walk_speed.as_ref().unwrap().to_string(),
        dash_initial_speed: char_info.cargoquery[0].title.dash_initial_speed.as_ref().unwrap().to_string(),
        dash_acceleration: char_info.cargoquery[0].title.dash_acceleration.as_ref().unwrap().to_string(),
        dash_friction: char_info.cargoquery[0].title.dash_friction.as_ref().unwrap().to_string(),
        jump_gravity: char_info.cargoquery[0].title.jump_gravity.as_ref().unwrap().to_string(),
        high_jump_gravity: char_info.cargoquery[0].title.high_jump_gravity.as_ref().unwrap().to_string(),
    }).unwrap();

    write!(file, "{}", processed_char_info)
    .expect(&("\nFailed to serialize ".to_owned() + CHARS[char_count]+ " 'info.json'."));
}