use dashmap::DashMap;
use poise::serenity_prelude::ChannelId;

pub struct ImageCache {
    pub cache: DashMap<ChannelId, String>,
}

impl ImageCache {
    pub fn new() -> Self {
        Self {
            cache: DashMap::with_capacity(15),
        }
    }

    pub fn get(&self, channel_id: ChannelId) -> Option<String> {
        self.cache.get(&channel_id).map(|v| v.value().clone())
    }

    pub fn insert(&self, channel_id: ChannelId, url: String) {
        self.cache.insert(channel_id, url);
    }

    pub fn clear(&self) {
        self.cache.clear();
    }
}
