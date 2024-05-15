use log::{error, info};
use reqwest::Client;
use serde_json::Value;
use std::env;

pub async fn is_message_safe(message: &str) -> Result<bool, reqwest::Error> {
    let api_key = env::var("OPENAI_API_KEY").expect("Expected an API key in the environment");
    let client = Client::new();
    let response = client
        .post("https://api.openai.com/v1/moderations")
        .header("Authorization", format!("Bearer {}", api_key))
        .header("Content-Type", "application/json")
        .json(&serde_json::json!({ "input": message }))
        .send()
        .await?;

    if response.status().is_success() {
        match response.json::<Value>().await {
            Ok(json) => {
                info!("Received response: {:?}", json);
                if let Some(results) = json.get("results").and_then(|r| r.as_array()) {
                    if let Some(first_result) = results.first() {
                        if let Some(flagged) = first_result.get("flagged").and_then(|f| f.as_bool())
                        {
                            return Ok(!flagged); // Return true if not flagged, false if flagged
                        }
                    }
                }
                error!("Unexpected JSON structure: {:?}", json);
            }
            Err(e) => {
                error!("Failed to parse JSON response: {}", e);
            }
        }
    } else {
        error!("Request failed with status: {}", response.status());
    }

    Ok(false)
}
