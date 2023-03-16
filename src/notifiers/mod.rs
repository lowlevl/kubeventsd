//! A collection of notifiers to send k8s events to

use async_trait::async_trait;
use color_eyre::eyre;

mod matrix;
pub use matrix::Matrix;

mod webhook;
pub use webhook::Webhook;

use super::events::Event;

/// A handy type-alias to a boxed notifier
pub type DynNotifier = Box<dyn Notifier>;

#[async_trait]
pub trait Notifier {
    async fn send(&self, event: &Event) -> eyre::Result<()>;
}
