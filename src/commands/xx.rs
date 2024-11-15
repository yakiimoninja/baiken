use crate::{check, Context, Error, Gids};
use colored::Colorize;
use std::io::Write;
use std::{fs, fs::OpenOptions};
use poise::{
    serenity_prelude as serenity,
    serenity_prelude::CreateSelectMenuKind,
    serenity_prelude::CreateSelectMenuOption,
    serenity_prelude::ComponentInteractionDataKind,
};

/// Disable or enable easter eggs for current server. Admin only.
#[poise::command(
    slash_command,
    ephemeral,
    hide_in_help,
    required_permissions = "ADMINISTRATOR", 
    default_member_permissions = "ADMINISTRATOR")]
pub async fn xx(
    ctx: Context<'_>,
    ) -> Result<(), Error> {

    // Check to see if users guild id exists
    if ctx.guild_id().is_none() {
        println!("{}", "This command is for servers only.".red());
        ctx.say("This command is for servers only.").await?;
        return Ok(());
    }
    
    if (check::adaptive_check(
        ctx,
        (false, &String::new()),
        (false, &String::new()),
        false,
        false,
        false,
        false,
        false,
        false,
        true).await).is_err() {
        
        return Ok(());
    }

    let user_option;

    let reply = {
        let components = vec![
            serenity::CreateActionRow::SelectMenu(serenity::CreateSelectMenu::new("dropdown",
                CreateSelectMenuKind::String { options: vec![
                    CreateSelectMenuOption::new("Disable", "true"),
                    CreateSelectMenuOption::new("Enable", "false")
            ].into()}))
        ];
        poise::CreateReply::default()
            .content(r#"**Disable easter eggs?**"#)
            .components(components)
    };

    ctx.send(reply).await?;

    while let Some(mci) = serenity::ComponentInteractionCollector::new(ctx.serenity_context())
        .timeout(std::time::Duration::from_secs(30))
        .filter(move |mci| mci.data.custom_id == "dropdown")
        .await
    {

        if let ComponentInteractionDataKind::StringSelect { ref values } = mci.data.kind {
            user_option = &values[0];
            println!("{:#?}", values);
        
            // Reading the gids json
            let data_from_file = fs::read_to_string("data/gids.json")
                .expect("\nFailed to read 'gids.json' file.");

            // Deserializing from gids json
            let mut vec_gids = serde_json::from_str::<Gids>(&data_from_file).unwrap();

            // Parse user guild id to string
            let guild_id = ctx.guild_id().unwrap().to_string();


            if user_option == "true"{
                // Hand to add guild id to exclusion list
                for x in vec_gids.id.iter() {
                    // Checking if guild is in the exclusion list
                    if guild_id == *x.to_string() {
                        println!("{}", "Guild id already exists.".magenta());
                        // Keeps modal from continuously loading
                        mci.create_response(ctx, serenity::CreateInteractionResponse::Acknowledge).await?;
                        ctx.say("Easter eggs for this server are already disabled.").await?;
                        return Ok(());
                    }
                }

                // Creating gids json file
                let mut file = OpenOptions::new()
                    .create(true)
                    .truncate(true)
                    .read(true)
                    .write(true)
                    .open("data/gids.json")
                    .expect("\nFailed to open 'gids.json' file.");

                // Adding guild id to exclusion list
                vec_gids.id.push(guild_id);
                let gid_to_json = serde_json::to_vec_pretty(&vec_gids).unwrap();
                file.write_all(&gid_to_json).unwrap();

                println!("{}", "Added new guild id in exclusion list.".magenta());
                // Keeps modal from continuously loading
                mci.create_response(ctx, serenity::CreateInteractionResponse::Acknowledge).await?;
                ctx.say("Easter eggs for this server have been disabled.").await?;
                return Ok(());
            }
            else if user_option == "false" {
                // Hand to remove guild id from exclusion list
                for x in 0..vec_gids.id.len() {
                    // Checking if guild is in the ee exclusion list
                    if guild_id == vec_gids.id[x] {

                        // Creating gids json file
                        let mut file = OpenOptions::new()
                            .create(true)
                            .read(true)
                            .write(true)
                            .truncate(true)
                            .open("data/gids.json")
                            .expect("\nFailed to open 'gids.json' file.");
                  
                        // Removing guild id from exclusion list
                        let _ = vec_gids.id.swap_remove(x);
                        let gid_to_json = serde_json::to_vec_pretty(&vec_gids).unwrap();
                        file.write_all(&gid_to_json).unwrap();

                        println!("{}", "Removed guild id from exclusion list.".magenta());
                        // Keeps modal from continuously loading
                        mci.create_response(ctx, serenity::CreateInteractionResponse::Acknowledge).await?;
                        ctx.say("Easter eggs for this server have been enabled.").await?;

                        return Ok(());
                    }
                }
      
                println!("{}", "Guild id doesnt exist in exclusion list.".magenta());
                // Keeps modal from continuously loading
                mci.create_response(ctx, serenity::CreateInteractionResponse::Acknowledge).await?;
                ctx.say("Easter eggs for this server are already enabled.").await?;
                return Ok(());
            }
            else {
                println!("{}", "Mega weird interaction with xx modal.".red());
                // Keeps modal from continuously loading
                mci.create_response(ctx, serenity::CreateInteractionResponse::Acknowledge).await?;
                return Ok(());
            }
        }
    }
    Ok(())
}
