use color_eyre::eyre;

pub use k8s_openapi::api::core::v1::Event;
use k8s_openapi::apimachinery::pkg::apis::meta::v1::MicroTime;

use super::rule::{Rule, RuleDestination};

mod ext;
pub use ext::EventExt;

/// Process events and send them to the notifiers
pub async fn process(rules: &[Rule], mut event: Event) -> eyre::Result<()> {
    event.event_time = Some(MicroTime(event.event_time()));

    let object = liquid::to_object(&event)?;

    for rule in rules {
        for RuleDestination { notifier, template } in &rule.destination {
            let message = template.render(&object)?;
            notifier.send(&message).await?;
        }
    }

    Ok(())
}
