use async_trait::async_trait;
use color_eyre::eyre;

use super::Event;

#[derive(Debug)]
pub struct Webhook {
    client: reqwest::Client,
    url: url::Url,
}

impl Webhook {
    pub async fn new(url: url::Url) -> eyre::Result<Self> {
        tracing::info!("Setting up Webhook sender to '{url}'");

        Ok(Self {
            client: reqwest::Client::new(),
            url,
        })
    }
}

#[async_trait]
impl super::Sender for Webhook {
    async fn send(&self, event: &Event) -> eyre::Result<()> {
        tracing::debug!("Sending event as to '{}': {event:?}", self.url);

        self.client
            .post(self.url.as_ref())
            .json(event)
            .send()
            .await?;

        Ok(())
    }
}
