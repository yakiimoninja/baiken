use crate::{Context, Error};
use poise::serenity_prelude as serenity;

/// Baiken's favorite.
#[poise::command(prefix_command, track_edits, slash_command)]
pub async fn sake(ctx: Context<'_>) -> Result<(), Error> {
    let uuid_boop = ctx.id();

    ctx.send(|m| {
        m.content("I want some sake!").components(|c| {
            c.create_action_row(|ar| {
                ar.create_button(|b| {
                    b.style(serenity::ButtonStyle::Primary)
                        .label("Sake me!")
                        .custom_id(uuid_boop)
                })
            })
        })
    })
    .await?;

    let mut boop_count = 0;
    while let Some(mci) = serenity::CollectComponentInteraction::new(ctx)
        .author_id(ctx.author().id)
        .channel_id(ctx.channel_id())
        .timeout(std::time::Duration::from_secs(120))
        .filter(move |mci| mci.data.custom_id == uuid_boop.to_string())
        .await
    {
        boop_count += 1;

        let mut msg = mci.message.clone();
        msg.edit(ctx, |m| {
            m.content(format!("Sake count: {}", boop_count))
        })
        .await?;

        mci.create_interaction_response(ctx, |ir| {
            ir.kind(serenity::InteractionResponseType::DeferredUpdateMessage)
        })
        .await?;
    }

    Ok(())
}

/// Adds multiple numbers
#[poise::command(prefix_command, slash_command)]
pub async fn addmultiple(
    ctx: Context<'_>,
    #[description = "An operand"] a: i8,
    #[description = "An operand"] b: u64,
    #[description = "An operand"]
    #[min = 1234567890123456_i64]
    #[max = 1234567890987654_i64]
    c: i64,
) -> Result<(), Error> {
    ctx.say(format!("Result: {}", a as i128 + b as i128 + c as i128))
        .await?;

    Ok(())
}