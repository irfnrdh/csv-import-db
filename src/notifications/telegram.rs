// src/notifications/telegram.rs
use reqwest::Client;
use std::error::Error;

pub struct TelegramNotifier {
    client: Client,
    token: String,
    chat_id: String,
}

impl TelegramNotifier {
    pub fn new(token: String, chat_id: String) -> Self {
        Self {
            client: Client::new(),
            token,
            chat_id,
        }
    }

    pub async fn send(&self, message: &str) -> Result<(), Box<dyn Error>> {
        let url = format!(
            "https://api.telegram.org/bot{}/sendMessage",
            self.token
        );
        
        self.client.post(&url)
            .form(&[
                ("chat_id", &self.chat_id),
                ("text", message)
            ])
            .send()
            .await?;
            
        Ok(())
    }
}