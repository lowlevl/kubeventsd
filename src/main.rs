use color_eyre::eyre::{self, Context};
use envconfig::Envconfig;
use futures::StreamExt;

use kube::{runtime::WatchStreamExt, Api};

mod config;

mod events;
use events::{Event, EventExt};

mod rules;
use rules::Rule;

mod senders;

#[tokio::main]
async fn main() -> eyre::Result<()> {
    // Install pretty panic-handler, with colored backtrace
    color_eyre::install()?;

    // Install logger formatter for [`tracing`]
    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .try_init()
        .map_err(|err| eyre::eyre!(err))?;

    let env = config::EnvConfig::init_from_env()?;

    tracing::info!("Loading configuration file at '{}'", env.config_path);

    let config: config::Config = serde_yaml::from_reader(std::fs::File::open(env.config_path)?)
        .wrap_err("While reading the configuration file")?;
    let rules = Rule::from_config(config).await?;

    let kube = kube::Client::try_default().await?;
    let info = kube.apiserver_version().await?;

    tracing::info!(
        "Successfully connected to kube-apiserver v{}.{}({}) on {}",
        info.major,
        info.minor,
        info.git_version,
        info.platform
    );

    let events: Api<Event> = Api::all(kube);
    let events = kube::runtime::watcher(events, Default::default()).applied_objects();

    let mut events = std::pin::pin!(events);

    let start = chrono::Utc::now();

    tracing::info!("Starting listening to events..");

    loop {
        match events.next().await {
            Some(Ok(event)) => {
                if event.event_time() < start {
                    // Skip events observed before the startup of the daemon to avoid re-sending old ones
                    continue;
                }

                // Process the event and log errors
                if let Err(err) = events::process(&rules, event).await {
                    tracing::error!("Error while processing event: {err}");
                }
            }
            Some(Err(err)) => tracing::error!("Received an error while polling for events: {err}"),
            None => {
                tracing::error!("Reached end-of-stream while polling for events, exiting");

                break Err(eyre::eyre!(
                    "Reached end-of-stream, this should never happen"
                ));
            }
        }
    }
}
