use color_eyre::eyre;

pub use k8s_openapi::api::core::v1::Event;

use super::filter::Filter;

mod ext;
pub use ext::EventExt;

/// Process events and send them to the notifiers
pub async fn process(filters: &[Filter], event: Event) -> eyre::Result<()> {
    tracing::info!(
        "{:?} {:?} at {}: {:?}",
        event.type_,
        event.reason,
        event.event_time(),
        event.message
    );

    let message = format!(
        "{:?} {:?} at {}: {:?}",
        event.type_,
        event.reason,
        event.event_time(),
        event.message
    );

    for filter in filters {
        for notifier in &filter.to {
            notifier.send(&message).await?;
        }
    }

    Ok(())
}
