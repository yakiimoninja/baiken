extern crate ureq;

use std::fs;
use aho_corasick::AhoCorasick;
use serde::Deserialize;
use rusqlite::{named_params, Connection as SqlConnection};
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
    let char_page_response_json = ac.replace_all(char_page_response_json.trim(), replace_with);

    //     // Colorful text RED
    //     .replace(r#"&lt;span class=&quot;colorful-text-4&quot; &gt;"#, "")
    //     // Colorful text BLUE
    //     .replace(r#"&lt;span class=&quot;colorful-text-2&quot; &gt;"#, "")
    //     // Colorful text GREEN
    //     .replace(r#"&lt;span class=&quot;colorful-text-3&quot; &gt;"#, "")
    //     // Colorful text PURPlE
    //     .replace(r#"&lt;span class=&quot;colorful-text-1&quot; &gt;"#, "")
    //     // Colorful text tag close
    //     .replace(r#"&lt;/span&gt;"#, "")
    //     .replace(r#"&lt;br&gt;"#, ", ")
    //     .replace(r#"&lt;br/&gt;"#, ", ")
    //     .replace(r#"&quot;"#, "")
    //     // Ino low profile
    //     .replace(r#" &lt;span class=&quot;tooltip&quot; &gt;Low Profile&lt;span class=&quot;tooltiptext&quot; style=&quot;&quot;&gt;When a character's hurtbox is entirely beneath an opponent's attack. This can be caused by crouching, certain moves, and being short.&lt;/span&gt;&lt;/span&gt;"#, "")
    //     // Replace apostrophe
    //     .replace(r#"&#039;"#, "'")
    //     .replace(r#"&amp;#32;"#, "") Ok
    //     .replace(r#"'''"#, "")
    //     .replace(r#"; "#, r#"\n"#)
    //     .replace(r#";"#, r#"\n"#)
    //     .replace(r#"\\"#, "");
    //
    //     Go from biggest string to smallest
    //     replace double "\\" with "/n bullet point"
    //     replace "; " with "\n"
    //     replace ";" with "\n"
    //     replace "&#039;" with "'"
    //     replace "&quot;" with double quote '"'
    //     replace &lt;small&gt;
    //     replace &lt;/small&gt;
    //     replace italic text "&#039;&#039;" first italic
    //     replace purple text "&lt;span class=&quot;colorful-text-1&quot; &gt;"
    //     replace "&lt;/span&gt;" with "SOME_WORD"
    //     replace blue text "&lt;span class=&quot;colorful-text-2&quot; &gt;SOME_WORD&lt;/span&gt;" with "SOME_WORD"
    //     replace green text "&lt;span class=&quot;colorful-text-3&quot; &gt;SOME_WORD&lt;/span&gt;" with "SOME_WORD"
    //     replace red text "&lt;span class=&quot;colorful-text-4&quot; &gt;SOME_WORD&lt;/span&gt;" with "SOME_WORD"
    //     replace orange text "&lt;span class=&quot;colorful-text-5&quot; &gt;SOME_WORD&lt;/span&gt;" with "SOME_WORD"
    //     
    //     replace "&lt;br&gt;" (<br>) with "\n"???
    //     replace "&lt;br/&gt;" (<br/>) "&lt;small&gt;" (<small>) SOME_WORD "&lt;/small&gt;" (</small>) "&lt;small&gt;" (<small>) "&lt;/small&gt;" (</small>)
    //     replace "&#039;&#039;&#039;SOME_WORD&#039;&#039;&#039;" becomes bold
    //     replace asuka icons "[[File:GGST Asuka R Accipiter Metron_Icon.png|50px]];" with ""

    let file = "x/".to_owned() + CHARS[char_count];
    fs::write(file , &char_page_response_json).unwrap();

    let mut move_data_response: Response = serde_json::from_str(&char_page_response_json).unwrap();
    let char_move_data = &mut move_data_response.cargoquery;

    for move_data in char_move_data {

        // Replacing None values with a generic '-'
        if move_data.title.input.is_none() {
            move_data.title.input = Some(String::from("-"));
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
INSERT INTO moves
(
character_id,
input,
name,
damage,
guard,
startup,
active,
recovery,
on_hit,
on_block,
level,
counter,
move_type,
risc_gain,
risc_loss,
wall_damage,
input_tension,
chip_ratio,
otg_ratio,
scaling,
invincibility,
cancel,
caption,
notes
)

VALUES
(
:character_id,
:input, 
:name,
:damage,
:guard,
:startup,
:active,
:recovery,
:on_hit,
:on_block,
:level,
:counter,
:move_type,
:risc_gain,
:risc_loss,
:wall_damage,
:input_tension,
:chip_ratio,
:otg_ratio,
:scaling,
:invincibility,
:cancel,
:caption,
:notes
)

ON CONFLICT (character_id, input)
DO UPDATE SET

character_id = :character_id,
input = :input, 
name = :name,
damage = :damage,
guard = :guard,
startup = :startup,
active = :active,
recovery = :recovery,
on_hit = :on_hit,
on_block = :on_block,
level = :level,
counter = :counter,
move_type = :move_type,
risc_gain = :risc_gain,
risc_loss = :risc_loss,
wall_damage = :wall_damage,
input_tension = :input_tension,
chip_ratio = :chip_ratio,
otg_ratio = :otg_ratio,
scaling = :scaling,
invincibility = :invincibility,
cancel = :cancel,
caption = :caption,
notes = :notes
", named_params!{
        ":character_id":   char_count + 1,
        ":input":          move_data.title.input.as_ref().unwrap_or(&empty).to_string(),
        ":name":           move_data.title.name.as_ref().unwrap_or(&empty).to_string(),
        ":damage":         move_data.title.damage.as_ref().unwrap_or(&empty).to_string(),
        ":guard":          move_data.title.guard.as_ref().unwrap_or(&empty).to_string(),
        ":startup":        move_data.title.startup.as_ref().unwrap_or(&empty).to_string(),
        ":active":         move_data.title.active.as_ref().unwrap_or(&empty).to_string(),
        ":recovery":       move_data.title.recovery.as_ref().unwrap_or(&empty).to_string(),
        ":on_hit":         move_data.title.on_hit.as_ref().unwrap_or(&empty).to_string(),
        ":on_block":       move_data.title.on_block.as_ref().unwrap_or(&empty).to_string(),
        ":level":          move_data.title.level.as_ref().unwrap_or(&empty).to_string(),
        ":counter":        move_data.title.counter.as_ref().unwrap_or(&empty).to_string(),
        ":move_type":      move_data.title.move_type.as_ref().unwrap_or(&empty).to_string(),
        ":risc_gain":      move_data.title.risc_gain.as_ref().unwrap_or(&empty).to_string(),
        ":risc_loss":      move_data.title.risc_loss.as_ref().unwrap_or(&empty).to_string(),
        ":wall_damage":    move_data.title.wall_damage.as_ref().unwrap_or(&empty).to_string(),
        ":input_tension":  move_data.title.input_tension.as_ref().unwrap_or(&empty).to_string(),
        ":chip_ratio":     move_data.title.chip_ratio.as_ref().unwrap_or(&empty).to_string(),
        ":otg_ratio":      move_data.title.otg_ratio.as_ref().unwrap_or(&empty).to_string(),
        ":scaling":        move_data.title.scaling.as_ref().unwrap_or(&empty).to_string(),
        ":invincibility":  move_data.title.invincibility.as_ref().unwrap_or(&empty).to_string(),
        ":cancel":         move_data.title.cancel.as_ref().unwrap_or(&empty).to_string(),
        ":caption":        move_data.title.caption.as_ref().unwrap_or(&String::from("")).to_string(),
        ":notes":          move_data.title.notes.as_ref().unwrap_or(&String::from("")).to_string(),
        }).unwrap();
    }

    db
}
