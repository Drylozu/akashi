use akashi_shared::strings::markdown::Markdown;
use akashi_shared::{AkashiContext, AkashiErr};
use pistones::client::Client;
use poise::CodeBlock;

/// Execute code with Piston engine
///
/// ;piston ```rs
/// fn main() {
///     println!("Hello, world!");
/// }
/// ```
#[poise::command(
    prefix_command,
    category = "Util",
    aliases("code", "e", "exec", "run"),
    track_edits,
    broadcast_typing
)]
pub async fn piston(
    ctx: AkashiContext<'_>,
    #[description = "Code to run (codeblock)"] code: CodeBlock,
) -> Result<(), AkashiErr> {
    let language = code.language.unwrap_or_else(|| "rust".to_string());
    let language = language.as_str();

    let piston_client = Client::new().await?;

    let start = std::time::Instant::now();
    match piston_client.run(language, code.code).await {
        Ok(output) => {
            ctx.say(format!(
                "took {}ms\n{}",
                start.elapsed().as_millis(),
                output.data().output().codeblock(language)
            ))
            .await?;
        }
        Err(e) => {
            Err(AkashiErr::from(e))?;
        }
    }

    Ok(())
}
