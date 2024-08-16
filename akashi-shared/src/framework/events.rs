use crate::utils::image::parse_message_media;
use crate::{AkashiData, AkashiErr};
use poise::serenity_prelude as serenity;
use serenity::FullEvent::*;
use tracing::{debug, info};

pub async fn event_handler(
    _: &serenity::Context,
    event: &serenity::FullEvent,
    _: poise::FrameworkContext<'_, AkashiData, AkashiErr>,
    data: &AkashiData,
) -> Result<(), AkashiErr> {
    match event {
        Ready { data_about_bot, .. } => {
            info!("Logged in as {}", data_about_bot.user.name);
        }
        Message { new_message } => {
            if let Some(media) = parse_message_media(&new_message) {
                let caches = data.caches.lock().await;

                caches.image.insert(new_message.channel_id, media.clone());
                info!(
                    channel_id = new_message.channel_id.to_string(),
                    url = media,
                    "Cached image from message"
                );
            }
        }
        _ => {
            debug!("Unhandled event: {:?}", event)
        }
    }
    Ok(())
}
