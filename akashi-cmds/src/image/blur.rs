use akashi_esi::job::SicJob;
use akashi_shared::utils::image::parse_command_image;
use akashi_shared::{AkashiContext, AkashiErr};
use poise::serenity_prelude::Attachment;

const DEFAULT_SIGMA: f32 = 5.0;

/// Blur an image
#[poise::command(
    slash_command,
    prefix_command,
    category = "Image",
    install_context = "Guild|User"
)]
pub async fn blur(
    ctx: AkashiContext<'_>,
    #[description = "Amount of blur to apply (default: 5)"] sigma: Option<f32>,
    #[description = "Image url"] url: Option<String>,
    #[description = "Image attachment"] attachment: Option<Attachment>,
) -> Result<(), AkashiErr> {
    ctx.defer_or_broadcast().await?;

    let (source, format) = parse_command_image(ctx, url, attachment)
        .await
        .map_err(AkashiErr::from)?;

    let mut job = SicJob::new(format);
    job.with_bytes(source).await;
    job.blur(sigma.unwrap_or(DEFAULT_SIGMA))
        .run(ctx)
        .await
        .map_err(AkashiErr::from)?;

    Ok(())
}
