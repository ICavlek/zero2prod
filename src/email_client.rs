use reqwest::Client;

use crate::domain::SubscriberEmail;

pub struct EmailClient {
    #[allow(dead_code)]
    http_client: Client,
    #[allow(dead_code)]
    base_url: String,
    #[allow(dead_code)]
    sender: SubscriberEmail,
}

impl EmailClient {
    pub fn new(base_url: String, sender: SubscriberEmail) -> Self {
        Self {
            http_client: Client::new(),
            base_url,
            sender,
        }
    }

    pub async fn send_email(
        &self,
        _recipient: SubscriberEmail,
        _subject: &str,
        _html_content: &str,
        _text_content: &str,
    ) -> Result<(), String> {
        todo!()
    }
}