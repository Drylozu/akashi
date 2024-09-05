use akashi_shared::strings::ansi::Ansi;
use akashi_shared::strings::markdown::Markdown;
use akashi_shared::strings::table::generate_table;
use akashi_shared::{AkashiContext, AkashiErr};

/// Display Akashi stats
#[poise::command(slash_command, prefix_command, category = "Info")]
pub async fn stats(ctx: AkashiContext<'_>) -> Result<(), AkashiErr> {
    let (memory_usage, cpu_usage, guild_count, commands_usage) = get_stats(ctx).await?;

    let table_values = vec![
        (
            "Memory usage".to_string(),
            format!("{} MB", memory_usage).fg_blue(),
        ),
        ("CPU usage".to_string(), format!("{}%", cpu_usage).fg_blue()),
        ("Guilds".to_string(), guild_count.fg_blue()),
        ("Used commands".to_string(), commands_usage.fg_blue()),
    ];

    let table = generate_table(&table_values, false).codeblock("ansi");

    ctx.say(format!(
        "{}\n{}",
        table,
        "Commands usages are tracked since Akashi's last restart".tiny()
    ))
    .await?;

    Ok(())
}

/// Returns Akashi's process stats
async fn get_stats(ctx: AkashiContext<'_>) -> Result<(u64, f32, usize, i32), AkashiErr> {
    let mut system_info = ctx.data().sysinfo.lock().await;
    let custom_cache = ctx.data().caches.lock().await;

    system_info.refresh_all();

    let akashi_pid = sysinfo::get_current_pid().map_err(AkashiErr::from).unwrap();
    let akashi_process = system_info
        .process(akashi_pid)
        .ok_or_else(|| AkashiErr::from("Process not found for some reason"))
        .unwrap();

    let memory_usage = akashi_process.memory() / 1024 / 1024;
    let cpu_usage = akashi_process.cpu_usage();
    let guild_count = ctx.cache().guild_count();
    let commands_usage = custom_cache.usage.total_usages();

    Ok((memory_usage, cpu_usage, guild_count, commands_usage))
}
