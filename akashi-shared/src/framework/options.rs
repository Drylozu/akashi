use crate::framework::events::event_handler;
use crate::{AkashiContext, AkashiData, AkashiErr};
use akashi_cache::caches::initialize_cache;
use poise::futures_util::lock::Mutex;
use poise::Command;
use std::sync::Arc;
use std::time::Duration;
use tracing::debug;

pub fn initialize_poise_options(
    commands: Vec<Command<AkashiData, AkashiErr>>,
) -> poise::FrameworkOptions<AkashiData, AkashiErr> {
    poise::FrameworkOptions {
        commands,
        prefix_options: poise::PrefixFrameworkOptions {
            prefix: Some(";".into()),
            edit_tracker: Some(Arc::new(poise::EditTracker::for_timespan(
                Duration::from_secs(10),
            ))),
            additional_prefixes: vec![
                poise::Prefix::Literal("akashi"),
                #[cfg(debug_assertions)]
                poise::Prefix::Literal("dev"),
            ],
            mention_as_prefix: true,
            ..Default::default()
        },
        // on_error:  |error| Box::pin(poise::builtins::on_error(error)),
        pre_command: |ctx| {
            Box::pin(async move {
                debug!("Executing command: {:#?}", ctx.command());
            })
        },
        post_command: |ctx: AkashiContext| {
            Box::pin(async move {
                let cache = ctx.data().caches.lock().await;
                let command_name = ctx.command().name.clone();

                // Increment the usage count for the executed command
                cache.usage.increment(command_name.clone());

                debug!("Executed command: {:#?}", command_name);
            })
        },
        initialize_owners: true,
        skip_checks_for_owners: false,
        event_handler: |ctx, event, framework, data| {
            Box::pin(event_handler(ctx, event, framework, data))
        },
        ..Default::default()
    }
}

pub fn initialize_poise_framework(
    options: poise::FrameworkOptions<AkashiData, AkashiErr>,
) -> poise::Framework<AkashiData, AkashiErr> {
    poise::Framework::builder()
        .setup(move |ctx, _ready, framework| {
            Box::pin(async move {
                poise::builtins::register_globally(ctx, &framework.options().commands).await?;

                Ok(initialize_data())
            })
        })
        .options(options)
        .build()
}

pub fn initialize_data() -> AkashiData {
    AkashiData {
        sysinfo: Arc::new(Mutex::new(sysinfo::System::new())),
        caches: Arc::new(Mutex::new(initialize_cache())),
        req: Arc::new(Mutex::new(reqwest::Client::new())),
    }
}
