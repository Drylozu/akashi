use akashi_shared::utils::image::parse_command_image;
use akashi_shared::{AkashiContext, AkashiErr};
use poise::serenity_prelude::Attachment;
use poise::ChoiceParameter;
use akashi_esi::job::SicJob;

#[derive(Debug, poise::ChoiceParameter)]
enum FormatChoice {
    #[name = "avif"]
    Avif,
    #[name = "bmp"]
    Bmp,
    #[name = "openexr"]
    OpenExr,
    #[name = "farbfeld"]
    Farbfeld,
    #[name = "gif"]
    Gif,
    #[name = "ico"]
    Ico,
    #[name = "jpeg"]
    Jpeg,
    #[name = "png"]
    Png,
    #[name = "pnm"]
    Pnm,
    #[name = "qoi"]
    Qoi,
    #[name = "tga"]
    Tga,
    #[name = "tiff"]
    Tiff,
    #[name = "webp"]
    Webp,
}

/// Convert an image to desired format
#[poise::command(slash_command, prefix_command, category = "Image")]
pub async fn convert(
    ctx: AkashiContext<'_>,
    #[description = "New format"] format: FormatChoice,
    #[description = "Image url"] url: Option<String>,
    #[description = "Image attachment"] attachment: Option<Attachment>,
) -> Result<(), AkashiErr> {
    ctx.defer().await?;

    let format = format.name().to_string();

    let (source, _) = parse_command_image(ctx, url, attachment)
        .await
        .map_err(AkashiErr::from)?;

    let mut job = SicJob::new(format);
    job.with_bytes(source).await;
    job.run(ctx).await.map_err(AkashiErr::from)?;

    Ok(())
}
