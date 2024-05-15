mod commands;
mod moderation;

use chrono::Utc;
use commands::moderation::*;
use dotenv::dotenv;
use env_logger;
use parse_duration;
use serenity::model::prelude::ChannelId;
use serenity::{
    async_trait,
    client::bridge::gateway::GatewayIntents,
    framework::standard::{macros::group, StandardFramework},
    model::{channel::Message, gateway::Ready},
    prelude::*,
    Client,
};
use std::env;
use std::time::Duration;

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn message(&self, ctx: Context, msg: Message) {
        if msg.author.bot {
            return;
        }

        let guild_id = msg.guild_id.unwrap().to_string();
        let settings = commands::moderation::load_server_settings(&guild_id)
            .await
            .unwrap_or_else(|| ServerSettings {
                guild_id: guild_id.clone(),
                warnings: 3,
                mute_time: "10m".to_string(),
                use_warnings: false,
                sensitivity: 0.5,
                logs_channel_id: None,
                mute_enabled: false,
            });

        // Check message safety and handle images
        if let Some(attachment) = msg.attachments.get(0) {
            if attachment
                .content_type
                .as_ref()
                .map_or(false, |ct| ct.starts_with("image"))
            {
                if !moderation::image::is_image_safe(attachment).await {
                    if let Some(logs_channel_id) = settings.logs_channel_id.as_ref() {
                        let logs_channel_id = logs_channel_id.parse::<u64>().unwrap();
                        let logs_channel = ChannelId(logs_channel_id);
                        let _ = logs_channel
                            .say(
                                &ctx.http,
                                format!("Deleted unsafe image message: {}", msg.id),
                            )
                            .await;
                    }
                    let _ = msg.delete(&ctx.http).await;
                    println!("Deleted unsafe image message: {}", msg.id);
                    tokio::time::sleep(Duration::from_secs(5)).await;
                    return;
                }
            }
        } else if !moderation::message::is_message_safe(&msg.content)
            .await
            .unwrap_or_default()
        {
            if let Some(logs_channel_id) = settings.logs_channel_id.as_ref() {
                let logs_channel_id = logs_channel_id.parse::<u64>().unwrap();
                let logs_channel = ChannelId(logs_channel_id);
                let _ = logs_channel
                    .say(
                        &ctx.http,
                        format!(
                            "Deleted unsafe image message: {} with content: {}",
                            msg.id, msg.content
                        ),
                    )
                    .await;
            }
            let warning_msg = msg
                .channel_id
                .say(
                    &ctx.http,
                    format!("Hey! Don't use that language! <@{}>", msg.author.id),
                )
                .await
                .unwrap();
            let _ = msg.delete(&ctx.http).await;
            tokio::time::sleep(Duration::from_secs(5)).await;
            warning_msg.delete(&ctx.http).await.unwrap();

            if settings.mute_enabled {
                let mute_time =
                    parse_duration::parse(&settings.mute_time).unwrap_or(Duration::from_secs(600));
                let mut member = msg
                    .guild_id
                    .unwrap()
                    .member(&ctx.http, msg.author.id)
                    .await
                    .unwrap();
                let mute_until = Utc::now() + chrono::Duration::from_std(mute_time).unwrap();
                member
                    .disable_communication_until_datetime(&ctx.http, mute_until)
                    .await
                    .unwrap();
            }
            println!("Deleted unsafe message: {}", msg.id);
            return;
        }
    }

    async fn ready(&self, _: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);
    }
}

#[group]
#[commands(
    set_warnings,
    set_mute_time,
    use_warnings,
    set_sensitivity,
    set_logs_channel
)]
struct General;

#[tokio::main]
async fn main() {
    env_logger::init();
    dotenv().ok();
    let token = env::var("DISCORD_BOT_TOKEN").expect("Expected a token in the environment");

    let framework = StandardFramework::new()
        .configure(|c| c.prefix("$"))
        .group(&GENERAL_GROUP)
        .group(&MODERATION_GROUP);

    let intents = GatewayIntents::all();

    let mut client = Client::builder(&token)
        .framework(framework)
        .event_handler(Handler)
        .intents(intents)
        .await
        .expect("Error creating client");

    // Load server settings from database on startup
    commands::moderation::load_all_server_settings().await;

    if let Err(why) = client.start().await {
        println!("Client error: {:?}", why);
    }
}
