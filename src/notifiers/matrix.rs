use async_trait::async_trait;
use color_eyre::eyre;
use ruma::{Client, OwnedRoomId, TransactionId, UserId};

#[derive(Debug)]
pub struct Matrix {
    client: Client<reqwest::Client>,
    room_id: OwnedRoomId,
}

impl Matrix {
    pub async fn new(
        homeserver_url: url::Url,
        user_id: &UserId,
        password: &str,
        room_id: OwnedRoomId,
    ) -> eyre::Result<Self> {
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

        Ok(Self { client, room_id })
    }
}

#[async_trait]
impl super::Notifier for Matrix {
    async fn send(&self, text: &str) -> eyre::Result<()> {
        let message = ruma::events::room::message::RoomMessageEventContent::notice_plain(text);
        let request = ruma::api::client::message::send_message_event::v3::Request::new(
            self.room_id.clone(),
            TransactionId::new(),
            &message,
        )?;

        self.client.send_request(request).await?;

        Ok(())
    }
}
