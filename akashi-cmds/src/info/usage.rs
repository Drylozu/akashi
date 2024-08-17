use akashi_shared::strings::{ansi::Ansi, markdown::Markdown};
use akashi_shared::{strings::table::generate_list_fixed_delim, AkashiContext, AkashiErr};

/// Display how many uses has every command
#[poise::command(slash_command, prefix_command, category = "Info")]
pub async fn usage(ctx: AkashiContext<'_>, command: Option<String>) -> Result<(), AkashiErr> {
    let caches = ctx.data().caches.lock().await;

    match command {
        Some(command) => {
            let command_usage = caches.usage.get(command.clone());

            match command_usage {
                Some(usage) => {
                    let message = format!("`{command}` command was used `{usage}` times");

                    ctx.say(message).await?;
                }
                None => {
                    ctx.say(format!("No usage found for `{}`", command)).await?;
                }
            }
        }
        None => {
            let ordered_entries = caches.usage.ordered_entries();
            let mut list_entries: Vec<(String, String)> = Vec::new();

            if ordered_entries.is_empty() {
                ctx.say("No commands were ran since bot restart").await?;
            } else {
                for (command, usage) in ordered_entries {
                    list_entries.push((command.fg_green(), usage.fg_white()));
                }

                let list = generate_list_fixed_delim(
                    &"Command".fg_red(),
                    &"Uses".fg_red(),
                    list_entries.as_slice(),
                    7,
                    4,
                );

                ctx.say(format!(
                    "{}\n\nData measured since Akashi's last restart.",
                    list.codeblock("ansi")
                ))
                .await?;
            }
        }
    };

    Ok(())
}
