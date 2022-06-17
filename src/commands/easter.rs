extern crate rand;

use std::string::String;
use serenity::model::prelude::*;
use serenity::prelude::*;
use rand::Rng;

struct BOTS {
    names: Vec<String>,
    ids: Vec<u64>,
    quote_pos: Vec<usize>, 
    quotes: Vec<String>,
}


pub async fn easter(ctx: &Context, msg: &Message) -> Option<String> {

    let le_bots = BOTS{
        names: Vec::from(["That Man".to_string(),
                            "Eddie".to_string(),
                            "YAGPDB.xyz".to_string(),
                            "Robo-Millia".to_string()]),
                            //baiken 919027797429727272
        ids: Vec::from([861273965666238485, 343328647391870989, 204255221017214977, 966815679238000720]),
        quote_pos: Vec::from([0,2,3,5]),
        quotes: Vec::from( ["Finally found you...That Man.".to_string(),
                            "I can't read the thoughts of the dead. Never expected that'd be a problem..".to_string(),
                            "What the hell? Your body's lifeless...Is your real form over there?".to_string(),
                            "Hah! What a single-minded attack. I like it, old man.".to_string(),
                            "Stop acting cool and calm. If you really want to defeat me, try getting a little desperate".to_string(),
                            "Wanting to protect something isn't enough... If you're not ready to put it all on the line, you don't belong on the battlefield.".to_string()]),
    };


    // Getting guild id
    let g_id = ctx.cache.guild_channel(msg.channel_id).await.expect("Failed to get guild id!").guild_id;
    let g_id: u64 = g_id.as_u64().to_owned();

    let mut found_bots: Vec<UserId> = Vec::new();

    // Checking if above bots are present in the current server
    for x in 0..le_bots.names.len(){
        
        let user = ctx.http.get_member(g_id, le_bots.ids[x]).await;
        match user {
            Ok(user) => {
                found_bots.push(user.user.id);
            },
            Err(_) => {}
        }
    
    }

    // Deciding if to display quote
    let random_number: u8 = rand::thread_rng().gen_range(1..101);

    // Chance of displaying quote
    if random_number <= 20 {

        // Deciding which quote to display
        let bot_index= rand::thread_rng().gen_range(0..found_bots.len());
        

        for x in 0..le_bots.ids.len() {

            if found_bots[bot_index] == le_bots.ids[x] {

                let quote_set_index = le_bots.quote_pos[x];

                if x != 0 {
                    let prev_quote_set_index = le_bots.quote_pos[x-1];
                    let amount_of_quotes = quote_set_index - prev_quote_set_index;
                    let random_quote = rand::thread_rng().gen_range(0..amount_of_quotes);    
                    let quote_index = quote_set_index - random_quote;
                    
                    return Some(le_bots.quotes[quote_index].to_owned());
                }
                else{
                    return Some(le_bots.quotes[quote_set_index].to_owned());
                }
            }
        }
    }
    
    return None;
       
}