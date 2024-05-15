use lazy_static::lazy_static;
use serenity::framework::standard::{
    macros::{command, group},
    Args, CommandResult,
};
use serenity::model::prelude::*;
use serenity::prelude::*;
use sqlx::SqlitePool;
use std::collections::HashMap;
use std::env;

lazy_static! {
    static ref DATABASE_URL: String =
        env::var("DATABASE_URL").expect("Expected a database URL in the environment");
    static ref POOL: SqlitePool =
        SqlitePool::connect_lazy(&DATABASE_URL).expect("Failed to create pool");
}

#[derive(Debug, sqlx::FromRow)]
pub struct ServerSettings {
    pub guild_id: String,
    pub warnings: i32,
    pub mute_time: String,
    pub use_warnings: bool,
    pub sensitivity: f32,
    pub logs_channel_id: Option<String>,
    pub mute_enabled: bool,
}

// Load server settings from the database
pub async fn load_server_settings(guild_id: &str) -> Option<ServerSettings> {
    sqlx::query_as::<_, ServerSettings>("SELECT * FROM server_settings WHERE guild_id = ?")
        .bind(guild_id)
        .fetch_optional(&*POOL)
        .await
        .ok()?
}

// Load all server settings from the database
pub async fn load_all_server_settings() -> HashMap<String, ServerSettings> {
    let rows: Vec<ServerSettings> =
        sqlx::query_as::<_, ServerSettings>("SELECT * FROM server_settings")
            .fetch_all(&*POOL)
            .await
            .expect("Failed to load server settings");
    rows.into_iter().map(|s| (s.guild_id.clone(), s)).collect()
}

// Save server settings to the database
pub async fn save_server_settings(settings: &ServerSettings) {
    sqlx::query(
        "INSERT INTO server_settings (guild_id, warnings, mute_time, use_warnings, sensitivity, logs_channel_id, mute_enabled) 
         VALUES (?, ?, ?, ?, ?, ?, ?) 
         ON CONFLICT(guild_id) DO UPDATE SET 
            warnings = excluded.warnings, 
            mute_time = excluded.mute_time, 
            use_warnings = excluded.use_warnings, 
            sensitivity = excluded.sensitivity, 
            logs_channel_id = excluded.logs_channel_id,
            mute_enabled = excluded.mute_enabled"
    )
    .bind(&settings.guild_id)
    .bind(settings.warnings)
    .bind(&settings.mute_time)
    .bind(settings.use_warnings)
    .bind(settings.sensitivity)
    .bind(&settings.logs_channel_id)
    .bind(settings.mute_enabled)  // Add this binding
    .execute(&*POOL)
    .await
    .expect("Failed to save server settings");
}
#[group]
#[commands(
    set_warnings,
    set_mute_time,
    use_warnings,
    set_sensitivity,
    set_logs_channel
)]
struct Moderation;

#[command]
#[only_in(guilds)]
async fn set_warnings(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    let warnings: i32 = args.single()?;
    let guild_id = msg.guild_id.unwrap().to_string();
    let mut settings = load_server_settings(&guild_id)
        .await
        .unwrap_or(ServerSettings {
            guild_id: guild_id.clone(),
            warnings: 3,
            mute_time: "10m".to_string(),
            use_warnings: false,
            sensitivity: 0.5,
            logs_channel_id: None,
            mute_enabled: false,
        });
    settings.warnings = warnings;
    save_server_settings(&settings).await;
    msg.channel_id
        .say(&ctx.http, format!("Warnings set to: {}", warnings))
        .await?;
    Ok(())
}

#[command]
#[only_in(guilds)]
async fn set_mute_enabled(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    let mute_enabled: bool = args.single()?;
    let guild_id = msg.guild_id.unwrap().to_string();
    let mut settings = load_server_settings(&guild_id)
        .await
        .unwrap_or(ServerSettings {
            guild_id: guild_id.clone(),
            warnings: 3,
            mute_time: "10m".to_string(),
            use_warnings: false,
            sensitivity: 0.5,
            logs_channel_id: None,
            mute_enabled: false,
        });
    settings.mute_enabled = mute_enabled;
    save_server_settings(&settings).await;
    msg.channel_id
        .say(&ctx.http, format!("Mute enabled set to: {}", mute_enabled))
        .await?;
    Ok(())
}

#[command]
#[only_in(guilds)]
async fn set_mute_time(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    let mute_time: String = args.single()?;
    let guild_id = msg.guild_id.unwrap().to_string();
    let mut settings = load_server_settings(&guild_id)
        .await
        .unwrap_or(ServerSettings {
            guild_id: guild_id.clone(),
            warnings: 3,
            mute_time: mute_time.clone(),
            use_warnings: false,
            sensitivity: 0.5,
            logs_channel_id: None,
            mute_enabled: false,
        });
    settings.mute_time = mute_time.clone();
    save_server_settings(&settings).await;
    msg.channel_id
        .say(&ctx.http, format!("Mute time set to: {}", mute_time))
        .await?;
    Ok(())
}

#[command]
#[only_in(guilds)]
async fn use_warnings(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    let use_warnings: bool = args.single()?;
    let guild_id = msg.guild_id.unwrap().to_string();
    let mut settings = load_server_settings(&guild_id)
        .await
        .unwrap_or(ServerSettings {
            guild_id: guild_id.clone(),
            warnings: 3,
            mute_time: "10m".to_string(),
            use_warnings,
            sensitivity: 0.5,
            logs_channel_id: None,
            mute_enabled: false,
        });
    settings.use_warnings = use_warnings;
    save_server_settings(&settings).await;
    msg.channel_id
        .say(&ctx.http, format!("Use warnings set to: {}", use_warnings))
        .await?;
    Ok(())
}

#[command]
#[only_in(guilds)]
async fn set_sensitivity(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    let sensitivity: f32 = args.single()?;
    if sensitivity < 0.0 || sensitivity > 1.0 {
        msg.channel_id
            .say(&ctx.http, "Sensitivity must be between 0 and 1")
            .await?;
        return Ok(());
    }
    let guild_id = msg.guild_id.unwrap().to_string();
    let mut settings = load_server_settings(&guild_id)
        .await
        .unwrap_or(ServerSettings {
            guild_id: guild_id.clone(),
            warnings: 3,
            mute_time: "10m".to_string(),
            use_warnings: false,
            sensitivity,
            logs_channel_id: None,
            mute_enabled: false,
        });
    settings.sensitivity = sensitivity;
    save_server_settings(&settings).await;
    msg.channel_id
        .say(&ctx.http, format!("Sensitivity set to: {}", sensitivity))
        .await?;
    Ok(())
}

#[command]
#[only_in(guilds)]
async fn set_logs_channel(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    let logs_channel_id: u64 = args.single()?;
    let guild_id = msg.guild_id.unwrap().to_string();
    let mut settings = load_server_settings(&guild_id)
        .await
        .unwrap_or(ServerSettings {
            guild_id: guild_id.clone(),
            warnings: 3,
            mute_time: "10m".to_string(),
            use_warnings: false,
            sensitivity: 0.5,
            logs_channel_id: Some(logs_channel_id.to_string()),
            mute_enabled: false,
        });
    settings.logs_channel_id = Some(logs_channel_id.to_string());
    save_server_settings(&settings).await;
    msg.channel_id
        .say(
            &ctx.http,
            format!("Logs channel set to: {}", logs_channel_id),
        )
        .await?;
    Ok(())
}
