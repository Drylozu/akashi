mod blur;
mod convert;
mod speech;

use akashi_shared::{AkashiData, AkashiErr};

pub fn register() -> Vec<poise::Command<AkashiData, AkashiErr>> {
    vec![blur::blur(), convert::convert(), speech::speech()]
}
