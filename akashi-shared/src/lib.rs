use poise::futures_util::lock::Mutex;
use std::sync::Arc;

pub type AkashiErr = Box<dyn std::error::Error + Send + Sync>;
pub type AkashiContext<'a> = poise::Context<'a, AkashiData, AkashiErr>;

pub struct AkashiData {
    pub caches: Arc<Mutex<akashi_cache::caches::Cache>>,
    pub req: Arc<Mutex<reqwest::Client>>,
}

pub mod framework;
pub mod strings;
pub mod utils;
