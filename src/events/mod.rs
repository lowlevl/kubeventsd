use color_eyre::eyre;

pub use k8s_openapi::api::core::v1::Event;

use super::config::file::Config;

mod ext;
pub use ext::EventExt;

/// Process events and send them to the notifiers
pub async fn process(config: &Config, event: Event) -> eyre::Result<()> {
    tracing::info!(
        "{:?} {:?} at {}: {:?}",
        event.type_,
        event.reason,
        event.event_time(),
        event.message
    );

    Ok(())
}
