use color_eyre::eyre;

pub use k8s_openapi::api::core::v1::Event;
use k8s_openapi::apimachinery::pkg::apis::meta::v1::MicroTime;

use super::rules::Rule;

mod ext;
pub use ext::EventExt;

/// Process events and send them to the senders
pub async fn process(rules: &[Rule], mut event: Event) -> eyre::Result<()> {
    event.event_time = Some(MicroTime(event.event_time()));

    for Rule { destination, rule } in rules {
        if let (Some(reasons), Some(reason)) = (&rule.reason, &event.reason) {
            if !reasons.contains(reason) {
                continue;
            }
        }

        if let (Some(types), Some(type_)) = (&rule.type_, &event.type_) {
            if !types.contains(type_) {
                continue;
            }
        }

        if let (Some(namespaces), Some(namespace)) =
            (&rule.namespace, &event.involved_object.namespace)
        {
            if !namespaces.contains(namespace) {
                continue;
            }
        }

        for sender in destination {
            sender.send(&event).await?;
        }
    }

    Ok(())
}
