//! A collection of senders to send k8s events to

use async_trait::async_trait;
use color_eyre::eyre;

mod matrix;
pub use matrix::Matrix;

mod webhook;
pub use webhook::Webhook;

use super::events::Event;

/// A handy type-alias to a boxed sender
pub type DynSender = Box<dyn Sender>;

#[async_trait]
pub trait Sender {
    async fn send(&self, event: &Event) -> eyre::Result<()>;
}
