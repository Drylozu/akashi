use akashi_shared::strings::ansi::Ansi;
use akashi_shared::strings::markdown::Markdown;
use akashi_shared::strings::table::generate_table;
use akashi_shared::{AkashiContext, AkashiData, AkashiErr};
use poise::serenity_prelude::InstallationContext;

// Uhm, I customized the Help builtin a bit.

/// Extended information about commands
///
/// ;help <command>
/// ;help usage
/// ;help
#[poise::command(slash_command, prefix_command, category = "Info")]
pub async fn help(ctx: AkashiContext<'_>, command: Option<String>) -> Result<(), AkashiErr> {
    let commands = &ctx.framework().options().commands;

    match command {
        Some(command) => {
            let command_help = commands.iter().find(|c| c.name == command);

            match command_help {
                Some(cmd) => {
                    if cmd.hide_in_help {
                        return Err(AkashiErr::from("No command found with that name"));
                    }

                    let mut message = format!("{} command\n\n", cmd.name.fg_green());

                    message += &match (&cmd.description, &cmd.help_text) {
                        (Some(description), Some(help_text)) => {
                            format!("{}\n\n{}", description, help_text.fg_blue())
                        }
                        (Some(description), None) => description.to_owned(),
                        (None, Some(help_text)) => help_text.clone().fg_blue(),
                        (None, None) => "No help available".fg_red(),
                    };

                    message += &format!(
                        "\n\nuser installable? {}",
                        if cmd
                            .install_context
                            .as_deref()
                            .unwrap_or(&vec![])
                            .contains(&InstallationContext::User)
                        {
                            "yes".fg_green()
                        } else {
                            "no".fg_red()
                        }
                    );

                    if !cmd.parameters.is_empty() {
                        message += &format!("\n\n{}", "options\n".fg_green());
                        // A "options" variable that contains a Vec, like vec![(<name>, <description>)]
                        let options: Vec<(String, String)> = cmd
                            .parameters
                            .iter()
                            .map(|p| {
                                let description =
                                    p.description.as_deref().unwrap_or("No description");
                                (
                                    p.name.clone().fg_blue(),
                                    format!(
                                        "{} ({})",
                                        description,
                                        if p.required {
                                            "required".fg_red()
                                        } else {
                                            "optional".fg_green()
                                        }
                                    ),
                                )
                            })
                            .collect();
                        let options_table = generate_table(options.as_slice(), true);

                        message += &options_table;
                    }

                    ctx.say(message.codeblock("ansi")).await?;

                    Ok(())
                }
                None => Err(AkashiErr::from("No command found with that name")),
            }
        }
        None => {
            let mut message = format!("Displaying {} commands\n\n", commands.len().fg_green());

            let categories =
                dashmap::DashMap::<Option<&str>, Vec<&poise::Command<AkashiData, AkashiErr>>>::new(
                );
            for cmd in &ctx.framework().options().commands {
                categories
                    .entry(cmd.category.as_deref())
                    .or_default()
                    .push(cmd);
            }

            for (category, commands) in categories {
                message += &format!(
                    "{} {}: {}\n",
                    category.unwrap_or("Uncategorized"),
                    format!("({})", commands.len()).fg_green(),
                    commands
                        .iter()
                        .filter(|c| !c.hide_in_help)
                        .map(|c| c.name.clone())
                        .collect::<Vec<_>>()
                        .join(", ")
                        .fg_blue()
                );
            }

            message += &format!(
                "\nRun {} to see detailed information about a command",
                ";help [command]".fg_green()
            );

            ctx.say(message.codeblock("ansi")).await?;

            Ok(())
        }
    }
}
