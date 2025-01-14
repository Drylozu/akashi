mod help;
mod stats;

use akashi_shared::{AkashiData, AkashiErr};

pub fn register() -> Vec<poise::Command<AkashiData, AkashiErr>> {
    vec![help::help(), stats::stats()]
}
