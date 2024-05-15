use reqwest::Client;
use serenity::model::channel::Attachment;
use std::env;

pub async fn is_image_safe(attachment: &Attachment) -> bool {
    let api_key =
        env::var("YOUR_IMAGE_MODERATION_API_KEY").expect("Expected an API key in the environment");
    let client = Client::new();
    let res = client
        .post("YOUR_IMAGE_MODERATION_API_ENDPOINT")
        .header("Authorization", format!("Bearer {}", api_key))
        .json(&serde_json::json!({
            "url": attachment.url
        }))
        .send()
        .await;

    match res {
        Ok(response) => {
            if let Ok(json) = response.json::<serde_json::Value>().await {
                if let Some(is_safe) = json["is_safe"].as_bool() {
                    return is_safe;
                }
            }
        }
        Err(_) => {}
    }
    false
}
