extern crate ureq;
use aho_corasick::AhoCorasick;
use rusqlite::{named_params, Connection as SqlConnection};
use serde::Deserialize;

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

pub async fn info_to_db(char_info_response_json: &str, db: SqlConnection, char_count: usize) -> SqlConnection {

    let empty = String::from("-");

    let patterns = &[
        "&lt;span class=&quot;colorful-text-1&quot; &gt;",
        "&lt;span class=&quot;colorful-text-2&quot; &gt;",
        "&lt;span class=&quot;colorful-text-3&quot; &gt;",
        "&lt;span class=&quot;colorful-text-4&quot; &gt;",
        "&lt;span class=&quot;colorful-text-5&quot; &gt;",
        "&#039;&#039;&#039;",
        "&lt;/small&gt;",
        "&lt;small&gt;",
        "&lt;/span&gt;",
        "&amp;#32;",
        "&#039;",
        "&quot;",
        "\\\\",
        "; ",
        ";",
    ];

    let replace_with = &[
        "",
        "",
        "",
        "",
        "",
        "*",
        "",
        "",
        "",
        "",
        "'",
        r#"\""#,
        "\\n-",
        "\\n",
        "\\n",
    ];


    let ac = AhoCorasick::builder().match_kind(aho_corasick::MatchKind::LeftmostFirst).build(patterns).unwrap();
    let char_info_response_json = ac.replace_all(char_info_response_json.trim(), replace_with);
    println!("{:#?}", char_info_response_json);

    let mut char_info_response: Response = serde_json::from_str(&char_info_response_json).unwrap();
    let char_info = &mut char_info_response.cargoquery[0].title;

    let mut umo_processed_vec: Vec<String> = Vec::new();
    let mut _umo_processed_string: String = String::new();

    if char_info.umo.is_none(){
        _umo_processed_string = String::from("-");
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

    db.execute("
INSERT INTO info
(character_id, defense, guts, guard_balance, prejump, umo, forward_dash, backdash, backdash_duration, backdash_invincibility, backdash_airborne, backdash_distance, jump_duration, jump_height, high_jump_duration, high_jump_height, earliest_iad, ad_duration, ad_distance, abd_duration, abd_distance, movement_tension, jump_tension, airdash_tension, walk_speed, back_walk_speed, dash_initial_speed, dash_acceleration, dash_friction, jump_gravity, high_jump_gravity)

VALUES
(:character_id, :defense, :guts, :guard_balance, :prejump, :umo, :forward_dash, :backdash, :backdash_duration, :backdash_invincibility, :backdash_airborne, :backdash_distance, :jump_duration, :jump_height, :high_jump_duration, :high_jump_height, :earliest_iad, :ad_duration, :ad_distance, :abd_duration, :abd_distance, :movement_tension, :jump_tension, :airdash_tension, :walk_speed, :back_walk_speed, :dash_initial_speed, :dash_acceleration, :dash_friction, :jump_gravity, :high_jump_gravity)

ON CONFLICT (character_id)
DO UPDATE SET
character_id = :character_id, defense=:defense, guts = :guts, guard_balance = :guard_balance, prejump = :prejump, umo= :umo, forward_dash = :forward_dash, backdash = :backdash, backdash_duration = :backdash_duration, backdash_invincibility = :backdash_invincibility, backdash_airborne = :backdash_airborne, backdash_distance = :backdash_distance, jump_duration = :jump_duration, jump_height = :jump_height, high_jump_duration = :high_jump_duration, high_jump_height = :high_jump_height, earliest_iad = :earliest_iad, ad_duration = :ad_duration, ad_distance = :ad_distance, abd_duration = :abd_duration, abd_distance = :abd_distance, movement_tension = :movement_tension, jump_tension = :jump_tension, airdash_tension = :airdash_tension, walk_speed = :walk_speed, back_walk_speed = :back_walk_speed, dash_initial_speed = :dash_initial_speed, dash_acceleration = :dash_acceleration, dash_friction = :dash_friction, jump_gravity = :jump_gravity, high_jump_gravity = :high_jump_gravity",
    named_params! {
     ":character_id":             char_count + 1,
     ":defense":                  char_info.defense.as_ref().unwrap_or(&empty).to_string(),
     ":guts":                     char_info.guts.as_ref().unwrap_or(&empty).to_string(),
     ":guard_balance":            char_info.guard_balance.as_ref().unwrap_or(&empty).to_string(),
     ":prejump":                  char_info.prejump.as_ref().unwrap_or(&empty).to_string(),
     ":umo":                      _umo_processed_string,
     ":forward_dash":             char_info.forward_dash.as_ref().unwrap_or(&empty).to_string(),
     ":backdash":                 char_info.backdash.as_ref().unwrap_or(&empty).to_string(),
     ":backdash_duration":        char_info.backdash_duration.as_ref().unwrap_or(&empty).to_string(),
     ":backdash_invincibility":   char_info.backdash_invincibility.as_ref().unwrap_or(&empty).to_string(),
     ":backdash_airborne":        char_info.backdash_airborne.as_ref().unwrap_or(&empty).to_string(),
     ":backdash_distance":        char_info.backdash_distance.as_ref().unwrap_or(&empty).to_string(),
     ":jump_duration":            char_info.jump_duration.as_ref().unwrap_or(&empty).to_string(),
     ":jump_height":              char_info.jump_height.as_ref().unwrap_or(&empty).to_string(),
     ":high_jump_duration":       char_info.high_jump_duration.as_ref().unwrap_or(&empty).to_string(),
     ":high_jump_height":         char_info.high_jump_height.as_ref().unwrap_or(&empty).to_string(),
     ":earliest_iad":             char_info.earliest_iad.as_ref().unwrap_or(&empty).to_string(),
     ":ad_duration":              char_info.ad_duration.as_ref().unwrap_or(&empty).to_string(),
     ":ad_distance":              char_info.ad_distance.as_ref().unwrap_or(&empty).to_string(),
     ":abd_duration":             char_info.abd_duration.as_ref().unwrap_or(&empty).to_string(),
     ":abd_distance":             char_info.abd_distance.as_ref().unwrap_or(&empty).to_string(),
     ":movement_tension":         char_info.movement_tension.as_ref().unwrap_or(&empty).to_string(),
     ":jump_tension":             char_info.jump_tension.as_ref().unwrap_or(&empty).to_string(),
     ":airdash_tension":          char_info.airdash_tension.as_ref().unwrap_or(&empty).to_string(),
     ":walk_speed":               char_info.walk_speed.as_ref().unwrap_or(&empty).to_string(),
     ":back_walk_speed":          char_info.back_walk_speed.as_ref().unwrap_or(&empty).to_string(),
     ":dash_initial_speed":       char_info.dash_initial_speed.as_ref().unwrap_or(&empty).to_string(),
     ":dash_acceleration":        char_info.dash_acceleration.as_ref().unwrap_or(&empty).to_string(),
     ":dash_friction":            char_info.dash_friction.as_ref().unwrap_or(&empty).to_string(),
     ":jump_gravity":             char_info.jump_gravity.as_ref().unwrap_or(&empty).to_string(),
     ":high_jump_gravity":        char_info.high_jump_gravity.as_ref().unwrap_or(&empty).to_string(),
    }).unwrap();

    db
}
