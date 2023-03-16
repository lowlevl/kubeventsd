//! Structures for the configuration of the daemon

use std::collections::HashSet;

use envconfig::Envconfig;
use serde::{Deserialize, Serialize};

#[derive(Debug, Envconfig)]
pub struct EnvConfig {
    #[envconfig(from = "KUBEVENTSD_CONFIG_PATH", default = "./config.example.yaml")]
    pub config_path: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub events: Vec<EventFilter>,
    pub senders: Vec<Sender>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EventFilter {
    pub reason: Option<Vec<String>>,
    #[serde(rename = "type")]
    pub type_: Option<Vec<String>>,
    pub namespace: Option<Vec<String>>,

    pub to: HashSet<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Sender {
    pub name: String,
    pub spec: SenderSpec,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "kind")]
pub enum SenderSpec {
    #[serde(rename_all = "camelCase")]
    Matrix {
        template: String,
        homeserver_url: url::Url,
        user_id: ruma::OwnedUserId,
        password_env: String,
        room_id: ruma::OwnedRoomId,
    },
    #[serde(rename_all = "camelCase")]
    Webhook { url: url::Url },
}
