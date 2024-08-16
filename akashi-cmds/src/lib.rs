mod image;
mod info;

use akashi_shared::{AkashiData, AkashiErr};

macro_rules! register_commands {
    ($($module:ident), *) => {
        {
            let mut cmds = Vec::new();
            $(
                cmds.extend($module::register());
            )*
            cmds
        }
    };
}

pub fn register_all_commands() -> Vec<poise::Command<AkashiData, AkashiErr>> {
    register_commands!(info, image)
}
