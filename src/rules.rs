use std::{collections::HashMap, sync::Arc};

use color_eyre::eyre;
use futures::stream::{StreamExt, TryStreamExt};

use super::{
    config,
    senders::{self, DynSender},
};

pub struct Rule {
    pub rule: config::EventFilter,
    pub destination: Vec<Arc<DynSender>>,
}

impl Rule {
    pub async fn from_config(config: config::Config) -> eyre::Result<Vec<Self>> {
        // Instantiate all the senders as Box<dyn Sender>
        let senders = futures::stream::iter(config.senders)
            .then(|config| async move {
                let sender = match config.spec {
                    config::SenderSpec::Matrix {
                        template,
                        homeserver_url,
                        user_id,
                        password_env,
                        room_id,
                    } => Arc::new(Box::new(
                        senders::Matrix::new(
                            &template,
                            homeserver_url,
                            user_id,
                            &std::env::var(password_env)?,
                            room_id,
                        )
                        .await?,
                    ) as DynSender),
                    config::SenderSpec::Webhook { url } => {
                        Arc::new(Box::new(senders::Webhook::new(url).await?) as DynSender)
                    }
                };

                Ok::<_, eyre::Report>((config.name, sender))
            })
            .try_collect::<HashMap<_, _>>()
            .await?;

        config
            .events
            .into_iter()
            .map(|filter| {
                let senders = filter
                    .to
                    .iter()
                    .map(|name| {
                        senders
                            .get(name)
                            .ok_or_else(|| {
                                eyre::eyre!(
                                    "Sender named '{name}' was not defined in the configuration",
                                )
                            })
                            .map(Clone::clone)
                    })
                    .collect::<eyre::Result<Vec<_>>>()?;

                Ok(Self {
                    rule: filter,
                    destination: senders,
                })
            })
            .collect()
    }
}
