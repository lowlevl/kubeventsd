//! Structures related to the YAML configuration file

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub events: Vec<EventConfig>,
    pub notifiers: Vec<NotifierConfig>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EventConfig {
    pub reason: Option<Vec<String>>,
    pub type_: Option<Vec<String>>,
    pub namespace: Option<Vec<String>>,

    pub to: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NotifierConfig {
    pub name: String,
    pub spec: NotifierSpecConfig,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "kind")]
pub enum NotifierSpecConfig {
    Matrix { room: String },
}
