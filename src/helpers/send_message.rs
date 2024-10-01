use reqwest::Client;
use serde::Serialize;
use tracing::{error, info};

#[derive(Serialize)]
struct SendMessageParams {
    chat_id: String,
    text: String,
    parse_mode: String,
}

pub struct Sender {
    token: String,
    chat_id: String,
}

impl Sender {
    pub async fn send_message(&self, message: String) -> anyhow::Result<()> {
        let url = format!("https://api.telegram.org/bot{}/sendMessage", self.token);

        let params = SendMessageParams {
            chat_id: self.chat_id.clone(),
            text: message.to_string(),
            parse_mode: "MarkdownV2".to_string(),
        };

        let client = Client::new();
        let response = client.post(&url).json(&params).send().await?;
        let status = response.status();
        let text = response.text().await?;

        if status.is_success() {
            info!("Sending: {}", message);
            Ok(())
        } else {
            error!("Error sending message: {} | Status: {}", text, status);
            Err(anyhow::anyhow!(text))
        }
    }
}
