mod help;
mod usage;

use akashi_shared::{AkashiData, AkashiErr};

pub fn register() -> Vec<poise::Command<AkashiData, AkashiErr>> {
    vec![help::help(), usage::usage()]
}
