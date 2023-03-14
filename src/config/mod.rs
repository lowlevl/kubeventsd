//! Structures for the configuration of the daemon

use envconfig::Envconfig;

pub mod file;

#[derive(Debug, Envconfig)]
pub struct Config {
    #[envconfig(from = "KUBEVENTSD_CONFIG_PATH", default = "./config.example.yaml")]
    pub config_path: String,
}
