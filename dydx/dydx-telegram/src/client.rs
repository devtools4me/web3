use reqwest;
use reqwest::{Error, Response};
use async_trait::async_trait;

#[async_trait]
pub trait TelegramApi {
    async fn send_message(&self, msg: &str) -> Result<Response, Error>;
}

pub struct TelegramClient {
    pub token: String,
    pub chat_id: String
}

#[async_trait]
impl TelegramApi for TelegramClient {
    async fn send_message(&self, msg: &str) -> Result<Response, Error> {
        let client = reqwest::Client::new();
        let url = format!("https://api.telegram.org/bot{}/sendMessage?chat_id={}&text={}", self.token, self.chat_id, msg);
        client.get(url)
            .send()
            .await
    }
}