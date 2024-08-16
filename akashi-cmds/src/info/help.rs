use akashi_shared::{AkashiContext, AkashiErr};

/// Extended information about commands
#[poise::command(slash_command, prefix_command, category = "Info")]
pub async fn help(ctx: AkashiContext<'_>, command: Option<String>) -> Result<(), AkashiErr> {
    let help_cfg = poise::builtins::HelpConfiguration {
        include_description: true,
        extra_text_at_bottom: &format!("version {}", 1),
        ..Default::default()
    };

    poise::builtins::help(ctx, command.as_deref(), help_cfg).await?;
    Ok(())
}
