use colored::Colorize;
use crate::{Context, Error};

/// Displays Baiken stats.
#[poise::command(prefix_command, slash_command, aliases("s"))]
pub async fn stats(ctx: Context<'_>) -> Result<(), Error> {

    let cache = ctx.cache();
    let guild_ids = cache.guilds();
    let (total_guild_count, total_members) = {
        let guilds: Vec<_> = guild_ids.iter().filter_map(|id| cache.guild(*id)).collect();
        (
            guilds.len().to_string(),
            guilds
                .into_iter()
                .map(|g| g.member_count)
                .sum::<u64>()
                .to_string(),
        )
    };
    
    println!("{}", ("Server count: ".to_owned() + &total_guild_count + "\nMembers count: " + &total_members).magenta());

    let msg = "Servers with access to Baiken: ".to_owned()
        + "**" + &total_guild_count + "**"
        + "\nPopulace with access to Baiken: "
        + "**" + &total_members + "**"; 

    let _ = ctx.say(msg).await;

    Ok(())
}