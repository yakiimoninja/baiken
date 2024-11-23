extern crate ureq;
use serde::Deserialize;
use crate::{CHARS, CharInfo};
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
struct Title {
    defense: Option<String>,
    guts: Option<String>,
    #[serde(rename = "guardBalance")]
    guard_balance: Option<String>,
    prejump: Option<String>,
    umo: Option<String>,
    #[serde(rename = "forwarddash")]
    forward_dash: Option<String>,
    backdash: Option<String>,
    #[serde(rename = "backdashDuration")]
    backdash_duration: Option<String>,
    #[serde(rename = "backdashInvuln")]
    backdash_invincibility: Option<String>,
    #[serde(rename = "backdashAirborne")]
    backdash_airborne: Option<String>,
    #[serde(rename = "backdashDistance")]
    backdash_distance: Option<String>,
    #[serde(rename = "jump duration")]
    jump_duration: Option<String>,
    #[serde(rename = "jump height")]
    jump_height: Option<String>,
    #[serde(rename = "high jump duration")]
    high_jump_duration: Option<String>,
    #[serde(rename = "high jump height")]
    high_jump_height: Option<String>,
    #[serde(rename = "earliest iad")]
    earliest_iad: Option<String>,
    #[serde(rename = "ad duration")]
    ad_duration: Option<String>,
    #[serde(rename = "ad distance")]
    ad_distance: Option<String>,
    #[serde(rename = "abd duration")]
    abd_duration: Option<String>,
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

    let empty = String::from("-");

    char_info_response_json = char_info_response_json.replace(r#"&lt;br&gt;"#, ", ");
    char_info_response_json = char_info_response_json.replace(r#"&lt;br/&gt;"#, ", ");
    // Ino low profile
    char_info_response_json = char_info_response_json.replace(r#" &lt;span class=&quot;tooltip&quot; &gt;Low Profile&lt;span class=&quot;tooltiptext&quot; style=&quot;&quot;&gt;When a character's hurtbox is entirely beneath an opponent's attack. This can be caused by crouching, certain moves, and being short.&lt;/span&gt;&lt;/span&gt;"#, "");
    // Replace apostrophe
    char_info_response_json = char_info_response_json.replace(r#"&#039;"#, "'");

    let mut char_info_response: Response = serde_json::from_str(&char_info_response_json).unwrap();
    let char_info = &mut char_info_response.cargoquery[0].title;
 
    let mut umo_processed_vec: Vec<String> = Vec::new();
    let mut _umo_processed_string: String = String::new();
    
    if char_info.umo.is_none(){
        _umo_processed_string = "-".to_string();
    }
    else if char_info.umo.as_ref().unwrap().to_string().contains("[[GGST") {
    // Logic for removing [[GGST/Character/UMO|UMO]] padding
        
        // Split umo entries by comma
        let split_umo: Vec<&str> = char_info.umo
            .as_mut()
            .unwrap()
            .split(',')
            .collect();
        
        for umo in split_umo {
        // Removing [[GGST/Character/UMO|UMO]] and getting umo
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
    else {
    // Logic for case of [[GGST/Character/UMO|UMO]] not existing
        _umo_processed_string = char_info.umo.as_ref().unwrap().to_string();
    }

    // Serializing character info
    let processed_char_info = serde_json::to_string_pretty(&CharInfo {
        defense: char_info.defense.as_ref().unwrap_or(&empty).to_string(),
        guts: char_info.guts.as_ref().unwrap_or(&empty).to_string(),
        guard_balance: char_info.guard_balance.as_ref().unwrap_or(&empty).to_string(),
        prejump: char_info.prejump.as_ref().unwrap_or(&empty).to_string(),
        umo: _umo_processed_string,
        forward_dash: char_info.forward_dash.as_ref().unwrap_or(&empty).to_string(),
        backdash: char_info.backdash.as_ref().unwrap_or(&empty).to_string(),
        backdash_duration: char_info.backdash_duration.as_ref().unwrap_or(&empty).to_string(),
        backdash_invincibility: char_info.backdash_invincibility.as_ref().unwrap_or(&empty).to_string(),
        backdash_airborne: char_info.backdash_airborne.as_ref().unwrap_or(&empty).to_string(),
        backdash_distance: char_info.backdash_distance.as_ref().unwrap_or(&empty).to_string(),
        jump_duration: char_info.jump_duration.as_ref().unwrap_or(&empty).to_string(),
        jump_height: char_info.jump_height.as_ref().unwrap_or(&empty).to_string(),
        high_jump_duration: char_info.high_jump_duration.as_ref().unwrap_or(&empty).to_string(),
        high_jump_height: char_info.high_jump_height.as_ref().unwrap_or(&empty).to_string(),
        earliest_iad: char_info.earliest_iad.as_ref().unwrap_or(&empty).to_string(),
        ad_duration: char_info.ad_duration.as_ref().unwrap_or(&empty).to_string(),
        ad_distance: char_info.ad_distance.as_ref().unwrap_or(&empty).to_string(),
        abd_duration: char_info.abd_duration.as_ref().unwrap_or(&empty).to_string(),
        abd_distance: char_info.abd_distance.as_ref().unwrap_or(&empty).to_string(),
        movement_tension: char_info.movement_tension.as_ref().unwrap_or(&empty).to_string(),
        jump_tension: char_info.jump_tension.as_ref().unwrap_or(&empty).to_string(),
        airdash_tension: char_info.airdash_tension.as_ref().unwrap_or(&empty).to_string(),
        walk_speed: char_info.walk_speed.as_ref().unwrap_or(&empty).to_string(),
        back_walk_speed: char_info.back_walk_speed.as_ref().unwrap_or(&empty).to_string(),
        dash_initial_speed: char_info.dash_initial_speed.as_ref().unwrap_or(&empty).to_string(),
        dash_acceleration: char_info.dash_acceleration.as_ref().unwrap_or(&empty).to_string(),
        dash_friction: char_info.dash_friction.as_ref().unwrap_or(&empty).to_string(),
        jump_gravity: char_info.jump_gravity.as_ref().unwrap_or(&empty).to_string(),
        high_jump_gravity: char_info.high_jump_gravity.as_ref().unwrap_or(&empty).to_string(),
    }).unwrap();

    write!(file, "{}", processed_char_info)
    .expect(&("\nFailed to serialize ".to_owned() + CHARS[char_count]+ " 'info.json'."));
}
