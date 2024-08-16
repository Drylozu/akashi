use akashi_shared::{AkashiContext, AkashiErr};

/// Display how many uses has every command
#[poise::command(slash_command, prefix_command, category = "Info")]
pub async fn usage(ctx: AkashiContext<'_>, command: Option<String>) -> Result<(), AkashiErr> {
    let caches = ctx.data().caches.lock().await;

    match command {
        Some(command) => {
            let command_usage = caches.usage.get(command);

            match command_usage {
                Some(usage) => {
                    let message = format!("`{command}`: used {usage} times");

                    ctx.say(message).await?;
                }
                None => {
                    ctx.say(format!("No usage found for `{}`", command)).await?;
                }
            }
        },
        None => {
            let ordered_entries = caches.usage.ordered_entries();
            let mut message = "Commands usages\n".to_string();

            for (command, usage) in ordered_entries {
                message += &format!("`{command}`: used {usage} times\n");
            }

            ctx.say(message).await?;
        },
    };

    Ok(())
}
