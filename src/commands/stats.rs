use colored::Colorize;
use crate::{Context, Error, EMBED_COLOR};
use poise::serenity_prelude::CreateEmbed;

/// Display Baiken stats.
#[poise::command(prefix_command, slash_command)]
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
    
    // To view sharding amount allowed by discord
    //println!("{:#?}", ctx.http().get_bot_gateway().await);
    println!("{}", ("Server count: ".to_owned() + &total_guild_count + "\nMembers count: " + &total_members).purple());

    let msg = "- **Servers with access to Baiken →** ".to_owned() + &total_guild_count
        + "\n- **Populace with access to Baiken →** " + &total_members;

    // Sending the data as an embed
    let embed = CreateEmbed::new()
        .color(EMBED_COLOR)
        .description(msg);

    ctx.send(poise::CreateReply::default().embed(embed)).await?;

    Ok(())
}
