extern crate ureq;
use rusqlite::{params, Connection as SqlConnection};
use serde::Deserialize;
use crate::CHARS;

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

pub async fn info_to_json(char_info_response_json: &str, db: SqlConnection, char_count: usize) -> SqlConnection {

    let empty = String::from("-");

    let char_info_response_json = &char_info_response_json
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

    let mut char_info_response: Response = serde_json::from_str(char_info_response_json).unwrap();
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

    db.execute("INSERT INTO Info 
(id, character_id, defense, guts, guard_balance, prejump, umo, forward_dash, backdash, backdash_duration, backdash_invincibility, backdash_airborne, backdash_distance, jump_duration, jump_height, high_jump_duration, high_jump_height, earliest_iad, ad_duration, ad_distance, abd_duration, abd_distance, movement_tension, jump_tension, airdash_tension, walk_speed, back_walk_speed, dash_initial_speed, dash_acceleration, dash_friction, jump_gravity, high_jump_gravity)

VALUES
(?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?13, ?14, ?15, ?16, ?17, ?18, ?19, ?20, ?21, ?22, ?23, ?24, ?25, ?26, ?27, ?28, ?29, ?30, ?31, ?32)

ON CONFLICT(character_id)
DO UPDATE SET 

id = ?1, character_id = ?2, defense = ?3, guts = ?4, guard_balance = ?5, prejump = ?6, umo = ?7, forward_dash = ?8, backdash = ?9, backdash_duration = ?10, backdash_invincibility = ?11, backdash_airborne = ?12, backdash_distance = ?13, jump_duration = ?14, jump_height = ?15, high_jump_duration = ?16, high_jump_height = ?17, earliest_iad = ?18, ad_duration = ?19, ad_distance = ?20, abd_duration = ?21, abd_distance = ?22, movement_tension = ?23, jump_tension = ?24, airdash_tension = ?25, walk_speed = ?26, back_walk_speed = ?27, dash_initial_speed = ?28, dash_acceleration = ?29, dash_friction = ?30, jump_gravity = ?31, high_jump_gravity = ?32;", 
    params![
        char_count + 1,
        CHARS[char_count],
        char_info.defense.as_ref().unwrap_or(&empty).to_string(),
        char_info.guts.as_ref().unwrap_or(&empty).to_string(),
        char_info.guard_balance.as_ref().unwrap_or(&empty).to_string(),
        char_info.prejump.as_ref().unwrap_or(&empty).to_string(),
        _umo_processed_string,
        char_info.forward_dash.as_ref().unwrap_or(&empty).to_string(),
        char_info.backdash.as_ref().unwrap_or(&empty).to_string(),
        char_info.backdash_duration.as_ref().unwrap_or(&empty).to_string(),
        char_info.backdash_invincibility.as_ref().unwrap_or(&empty).to_string(),
        char_info.backdash_airborne.as_ref().unwrap_or(&empty).to_string(),
        char_info.backdash_distance.as_ref().unwrap_or(&empty).to_string(),
        char_info.jump_duration.as_ref().unwrap_or(&empty).to_string(),
        char_info.jump_height.as_ref().unwrap_or(&empty).to_string(),
        char_info.high_jump_duration.as_ref().unwrap_or(&empty).to_string(),
        char_info.high_jump_height.as_ref().unwrap_or(&empty).to_string(),
        char_info.earliest_iad.as_ref().unwrap_or(&empty).to_string(),
        char_info.ad_duration.as_ref().unwrap_or(&empty).to_string(),
        char_info.ad_distance.as_ref().unwrap_or(&empty).to_string(),
        char_info.abd_duration.as_ref().unwrap_or(&empty).to_string(),
        char_info.abd_distance.as_ref().unwrap_or(&empty).to_string(),
        char_info.movement_tension.as_ref().unwrap_or(&empty).to_string(),
        char_info.jump_tension.as_ref().unwrap_or(&empty).to_string(),
        char_info.airdash_tension.as_ref().unwrap_or(&empty).to_string(),
        char_info.walk_speed.as_ref().unwrap_or(&empty).to_string(),
        char_info.back_walk_speed.as_ref().unwrap_or(&empty).to_string(),
        char_info.dash_initial_speed.as_ref().unwrap_or(&empty).to_string(),
        char_info.dash_acceleration.as_ref().unwrap_or(&empty).to_string(),
        char_info.dash_friction.as_ref().unwrap_or(&empty).to_string(),
        char_info.jump_gravity.as_ref().unwrap_or(&empty).to_string(),
        char_info.high_jump_gravity.as_ref().unwrap_or(&empty).to_string(),
]).unwrap();

    db
}
