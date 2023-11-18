use std::io::Write;
use serde::Deserialize;
use crate::{CHARS, Frames};
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
    input: Option<String>,
    name: Option<String>,
    damage: Option<String>,
    guard: Option<String>,
    invuln: Option<String>,
    startup: Option<String>,
    active: Option<String>,
    recovery: Option<String>,
    #[serde(rename = "onHit")]
    hit: Option<String>,
    #[serde(rename = "onBlock")]
    block: Option<String>,
    level: Option<String>,
    #[serde(rename = "riscGain")]
    riscgain: Option<String>,
    prorate: Option<String>,
    counter: Option<String>,
}

pub async fn frames_to_json(mut char_page_response_json: String, mut file: &File, char_count: usize) {

    char_page_response_json = char_page_response_json.replace(r#"&lt;br&gt;"#, ", ");
    char_page_response_json = char_page_response_json.replace(r#"&lt;br/&gt;"#, ", ");
    // Ino low profile
    char_page_response_json = char_page_response_json.replace(r#" &lt;span class=&quot;tooltip&quot; &gt;Low Profile&lt;span class=&quot;tooltiptext&quot; style=&quot;&quot;&gt;When a character's hurtbox is entirely beneath an opponent's attack. This can be caused by crouching, certain moves, and being short.&lt;/span&gt;&lt;/span&gt;"#, "");

    let mut framedata: Response = serde_json::from_str(&char_page_response_json).unwrap();

    for x in 0..framedata.cargoquery.len() {
        
        // Replacing None values with a generic '-'
        if framedata.cargoquery[x].title.input.is_none(){
            framedata.cargoquery[x].title.input = Some("-".to_string());
        }
        else{
            // Skips finish blow for sol
            if *framedata.cargoquery[x].title.input.as_ref().unwrap() == "j.XX during Homing Jump" {
                continue;
            }
        }
        if framedata.cargoquery[x].title.name.is_none(){
            framedata.cargoquery[x].title.name = Some(framedata.cargoquery[x].title.input.as_ref().unwrap().to_string());
        }
        else {
            // Skips dash cancel entry, ino hoverdash chipp escape zato flight and finish blow 
            if *framedata.cargoquery[x].title.name.as_ref().unwrap() == "Dash Cancel" || 
            *framedata.cargoquery[x].title.name.as_ref().unwrap() == "Hoverdash" ||
            *framedata.cargoquery[x].title.name.as_ref().unwrap() == "Finish Blow" ||
            *framedata.cargoquery[x].title.name.as_ref().unwrap() == "Flight" ||
            *framedata.cargoquery[x].title.name.as_ref().unwrap() == "Escape" {
                continue;
            }
        }
        if framedata.cargoquery[x].title.damage.is_none(){
            framedata.cargoquery[x].title.damage = Some("-".to_string());
        }
        if framedata.cargoquery[x].title.guard.is_none(){
            framedata.cargoquery[x].title.guard = Some("-".to_string());
        }
        if framedata.cargoquery[x].title.invuln.is_none(){
            framedata.cargoquery[x].title.invuln = Some("-".to_string());
        }
        if framedata.cargoquery[x].title.startup.is_none(){
            framedata.cargoquery[x].title.startup = Some("-".to_string());
        }
        if framedata.cargoquery[x].title.active.is_none(){
            framedata.cargoquery[x].title.active = Some("-".to_string());
        }
        if framedata.cargoquery[x].title.recovery.is_none(){
            framedata.cargoquery[x].title.recovery = Some("-".to_string());
        }
        if framedata.cargoquery[x].title.hit.is_none(){
            framedata.cargoquery[x].title.hit = Some("-".to_string());
        }
        if framedata.cargoquery[x].title.block.is_none(){
            framedata.cargoquery[x].title.block = Some("-".to_string());
        }
        if framedata.cargoquery[x].title.level.is_none(){
            framedata.cargoquery[x].title.level = Some("-".to_string());
        }
        if framedata.cargoquery[x].title.riscgain.is_none(){
            framedata.cargoquery[x].title.riscgain = Some("-".to_string());
        }
        if framedata.cargoquery[x].title.prorate.is_none(){
            framedata.cargoquery[x].title.prorate = Some("-".to_string());
        }
        if framedata.cargoquery[x].title.counter.is_none(){
            framedata.cargoquery[x].title.counter = Some("-".to_string());
        }

        // Serializing frame data
        let processed_framedata = serde_json::to_string(&Frames {
            input: framedata.cargoquery[x].title.input.as_ref().unwrap().to_string(),
            name: framedata.cargoquery[x].title.name.as_ref().unwrap().to_string(),
            damage: framedata.cargoquery[x].title.damage.as_ref().unwrap().to_string(),
            guard: framedata.cargoquery[x].title.guard.as_ref().unwrap().to_string(),
            invincibility: framedata.cargoquery[x].title.invuln.as_ref().unwrap().to_string(),
            startup: framedata.cargoquery[x].title.startup.as_ref().unwrap().to_string(),
            active: framedata.cargoquery[x].title.active.as_ref().unwrap().to_string(),
            recovery: framedata.cargoquery[x].title.recovery.as_ref().unwrap().to_string(),
            hit: framedata.cargoquery[x].title.hit.as_ref().unwrap().to_string(),
            block: framedata.cargoquery[x].title.block.as_ref().unwrap().to_string(),
            level: framedata.cargoquery[x].title.level.as_ref().unwrap().to_string(),
            riscgain: framedata.cargoquery[x].title.riscgain.as_ref().unwrap().to_string(),
            scaling: framedata.cargoquery[x].title.prorate.as_ref().unwrap().to_string(),
            counter: framedata.cargoquery[x].title.counter.as_ref().unwrap().to_string(),
        }).unwrap();
        
        write!(file, "{}", processed_framedata)
        .expect(&("\nFailed to serialize '".to_owned() + CHARS[char_count]+ ".json'."));
        
        // Skip writting comma/tab if next and last iteration 
        // contains 'finish blow' in last the input field
        if x == framedata.cargoquery.len() -2 &&
        *framedata.cargoquery[x+1].title.input.as_ref().unwrap() == "j.XX during Homing Jump" {
            continue;
        }
        else if x != framedata.cargoquery.len() - 1 {         
            // Adding comma/tab
            // file.write(b",\n\t")
            //     .expect(&("\nFailed to write ',\\n\\t' while serializing '".to_owned() + CHARS[char_count]+ ".json'."));
            (&mut file).write_all(b",\n\t")
                .expect(&("\nFailed to write ',\\n\\t' while serializing '".to_owned() + CHARS[char_count]+ ".json'."));
        } 
    }
}
