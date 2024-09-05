mod piston;

use akashi_shared::{AkashiData, AkashiErr};

pub fn register() -> Vec<poise::Command<AkashiData, AkashiErr>> {
    vec![piston::piston()]
}
