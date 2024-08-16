pub mod image;
pub mod usage;

pub struct Cache {
    pub image: image::ImageCache,
    pub usage: usage::UsageCache,
}

pub fn initialize_cache() -> Cache {
    Cache {
        image: image::ImageCache::new(),
        usage: usage::UsageCache::new(),
    }
}
