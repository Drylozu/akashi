use crate::{AkashiContext, AkashiData, AkashiErr};
use poise::serenity_prelude::{Attachment, ChannelId, Message};
use reqwest::header::CONTENT_TYPE;
use reqwest::Response;

fn format_content_type(content_type: String) -> String {
    content_type.split("/").last().unwrap_or("png").to_string()
}

pub fn parse_message_media(msg: &Message) -> Option<String> {
    if let Some(attachment) = msg.attachments.first() {
        return Some(attachment.proxy_url.clone());
    }

    if let Some(embed) = msg.embeds.first() {
        if let Some(image) = &embed.image {
            return Some(image.proxy_url.clone().unwrap());
        } else if let Some(thumbnail) = &embed.thumbnail {
            return Some(thumbnail.proxy_url.clone().unwrap());
        }
    }

    if let Some(sticker) = msg.sticker_items.first() {
        return sticker.image_url().clone();
    }

    None
}

pub async fn get_cached_media(ctx: AkashiContext<'_>, channel_id: ChannelId) -> Option<String> {
    let caches = ctx.data().caches.lock().await;

    if let Some(url) = caches.image.get(channel_id) {
        return Some(url.to_string());
    }

    None
}

pub async fn fetch_image(data: &AkashiData, url: String) -> Result<Response, AkashiErr> {
    let client = data.req.lock().await;

    client.get(url).send().await.map_err(AkashiErr::from)
}

pub async fn parse_media_response(response: Response) -> Result<(Vec<u8>, String), AkashiErr> {
    let content_type = format_content_type(
        response
            .headers()
            .get(CONTENT_TYPE)
            .ok_or_else(|| AkashiErr::from("Content-Type header missing"))?
            .to_str()
            .map_err(AkashiErr::from)?
            .to_string(),
    );

    let bytes = response
        .bytes()
        .await
        .map_err(|e| AkashiErr::from(e.to_string()))?
        .to_vec();

    Ok((bytes, content_type))
}

pub async fn download_media(
    data: &AkashiData,
    url: String,
) -> Result<(Vec<u8>, String), AkashiErr> {
    let res = fetch_image(data, url).await?;

    parse_media_response(res).await
}

pub async fn parse_command_image(
    ctx: AkashiContext<'_>,
    url: Option<String>,
    attachment: Option<Attachment>,
) -> Result<(Vec<u8>, String), AkashiErr> {
    if let Some(url) = url {
        let (bytes, content_type) = download_media(ctx.data(), url).await?;

        return Ok((bytes, content_type));
    }

    if let Some(attachment) = attachment {
        let bytes = attachment.download().await?;
        let format = match attachment.content_type {
            Some(content_type) => format_content_type(content_type),
            None => {
                return Err(AkashiErr::from(
                    "Attachment returned an invalid `Content-Type` header",
                ))
            }
        };

        return Ok((bytes, format));
    }

    if let AkashiContext::Prefix(prefix) = ctx {
        if let Some(referenced) = prefix.msg.clone().referenced_message {
            if let Some(media) = parse_message_media(&referenced) {
                let (bytes, content_type) = download_media(ctx.data(), media).await?;

                return Ok((bytes, content_type));
            }
        }
    }

    if let Some(media) = get_cached_media(ctx, ctx.channel_id()).await {
        let (bytes, content_type) = download_media(ctx.data(), media).await?;

        return Ok((bytes, content_type));
    }

    Err(AkashiErr::from(
        "I couldn't find any media cached or provided",
    ))
}
