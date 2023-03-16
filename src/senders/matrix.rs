use async_trait::async_trait;
use color_eyre::eyre;
use ruma::{Client, OwnedRoomId, OwnedUserId, TransactionId};

use super::Event;

pub struct Matrix {
    client: Client<reqwest::Client>,
    template: liquid::Template,
    user_id: OwnedUserId,
    room_id: OwnedRoomId,
}

impl Matrix {
    pub async fn new(
        template: &str,
        homeserver_url: url::Url,
        user_id: OwnedUserId,
        password: &str,
        room_id: OwnedRoomId,
    ) -> eyre::Result<Self> {
        tracing::info!("Setting up Matrix sender as '{user_id}' on room '{room_id}'");

        let liquid = liquid::ParserBuilder::new().stdlib().build()?;
        let template = liquid.parse(template)?;

        let client = Client::builder()
            .homeserver_url(homeserver_url.into())
            .http_client(reqwest::Client::new())
            .await?;

        client
            .log_in(user_id.as_str(), password, None, None)
            .await?;

        // Ensure we logged-in and we have access to the said room
        client
            .send_request(
                ruma::api::client::membership::join_room_by_id::v3::Request::new(room_id.clone()),
            )
            .await?;

        Ok(Self {
            template,
            client,
            user_id,
            room_id,
        })
    }
}

#[async_trait]
impl super::Sender for Matrix {
    async fn send(&self, event: &Event) -> eyre::Result<()> {
        let object = liquid::to_object(&event)?;
        let message = self.template.render(&object)?;

        tracing::debug!(
            "Sending event as '{}' on room '{}': {message:?}",
            self.user_id,
            self.room_id
        );

        let message = ruma::events::room::message::RoomMessageEventContent::text_markdown(message);
        let request = ruma::api::client::message::send_message_event::v3::Request::new(
            self.room_id.clone(),
            TransactionId::new(),
            &message,
        )?;

        self.client.send_request(request).await?;

        Ok(())
    }
}
