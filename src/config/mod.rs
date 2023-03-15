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
    pub notifiers: Vec<Notifier>,
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
pub struct Notifier {
    pub name: String,
    pub template: String,
    pub spec: NotifierSpec,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "kind")]
pub enum NotifierSpec {
    #[serde(rename_all = "camelCase")]
    Matrix {
        homeserver_url: url::Url,
        user_id: ruma::OwnedUserId,
        password_env: String,
        room_id: ruma::OwnedRoomId,
    },
}
