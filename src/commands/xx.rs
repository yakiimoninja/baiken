use crate::{check, Context, Error, Gids};
use colored::Colorize;
use std::io::Write;
use std::{fs, fs::OpenOptions};

/// Disable or enable easter eggs for current server. Admin only.
#[poise::command(
    slash_command,
    ephemeral,
    hide_in_help,
    required_permissions = "ADMINISTRATOR",
    default_member_permissions = "ADMINISTRATOR"
)]
pub async fn xx(
    ctx: Context<'_>,
    #[description = "Disable or enable easter eggs."]
    #[choices("disable", "enable")]
    option: &'static str,
) -> Result<(), Error> {

    // Check to see if users guild id exists
    if ctx.guild_id().is_none() {
        println!("{}", "This command is for servers only.".red());
        ctx.say("This command is for servers only.").await?;
        return Ok(());
    }

    if (check::adaptive_check(
        ctx,
        false,
        false,
        false,
        false,
        false,
        false,
        true,).await).is_err() {
        return Ok(());
    }

    // Reading the gids json
    let data_from_file =
        fs::read_to_string("data/gids.json").expect("\nFailed to read 'gids.json' file.");

    // Deserializing from gids json
    let mut vec_gids = serde_json::from_str::<Gids>(&data_from_file).unwrap();

    // Parse user guild id to string
    let guild_id = ctx.guild_id().unwrap().to_string();

    if option == "disable" {
        // Hand to add guild id to exclusion list
        for x in vec_gids.id.iter() {
            // Checking if guild is in the exclusion list
            if guild_id == *x.to_string() {
                println!("{}", "Easter eggs are disabled.".purple());
                ctx.say("Easter eggs for this server are disabled.").await?;

                return Ok(());
            }
        }

        // Creating gids json file
        let mut file = OpenOptions::new()
            .create(true)
            .truncate(true)
            .write(true)
            .open("data/gids.json")
            .expect("\nFailed to open 'gids.json' file.");

        // Adding guild id to exclusion list
        vec_gids.id.push(guild_id);
        let gid_to_json = serde_json::to_vec_pretty(&vec_gids).unwrap();
        file.write_all(&gid_to_json).unwrap();

        println!("{}", "Easter eggs have been disabled.".purple());
        ctx.say("Easter eggs for this server have been disabled.").await?;
    }
    else if option == "enable" {
        // Hand to remove guild id from exclusion list
        for x in 0..vec_gids.id.len() {
            // Checking if guild is in the exclusion list
            if guild_id == vec_gids.id[x] {
                // Creating gids json file
                let mut file = OpenOptions::new()
                    .create(true)
                    .write(true)
                    .truncate(true)
                    .open("data/gids.json")
                    .expect("\nFailed to open 'gids.json' file.");

                // Removing guild id from exclusion list
                let _ = vec_gids.id.swap_remove(x);
                let gid_to_json = serde_json::to_vec_pretty(&vec_gids).unwrap();
                file.write_all(&gid_to_json).unwrap();

                println!("{}", "Easter eggs have been enabled".purple());
                ctx.say("Easter eggs for this server have been enabled.").await?;

                return Ok(());
            }
        }

        println!("{}", "Easter eggs are enabled.".purple());
        ctx.say("Easter eggs for this server are already enabled.").await?;
        return Ok(());
    }
    else {
        println!("{}", "Mega weird interaction with xx.".red());
        return Ok(());
    }
    Ok(())
}
