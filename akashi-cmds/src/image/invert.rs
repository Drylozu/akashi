use akashi_esi::job::SicJob;
use akashi_shared::utils::image::parse_command_image;
use akashi_shared::{AkashiContext, AkashiErr};
use poise::serenity_prelude::Attachment;

/// Invert an image colors
#[poise::command(
    slash_command,
    prefix_command,
    category = "Image",
    install_context = "Guild|User"
)]
pub async fn invert(
    ctx: AkashiContext<'_>,
    #[description = "Image url"] url: Option<String>,
    #[description = "Image attachment"] attachment: Option<Attachment>,
) -> Result<(), AkashiErr> {
    ctx.defer().await?;

    let (source, format) = parse_command_image(ctx, url, attachment)
        .await
        .map_err(AkashiErr::from)?;

    let mut job = SicJob::new(format);
    job.with_bytes(source).await;
    job.invert().run(ctx).await.map_err(AkashiErr::from)?;

    Ok(())
}
