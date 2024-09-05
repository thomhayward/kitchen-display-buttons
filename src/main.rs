mod clap;
mod config;

use config::{Config, Mapping};
use mqtt::QoS::AtMostOnce;

#[tokio::main(flavor = "current_thread")]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt::init();

    let Config {
        url,
        filter,
        mappings,
    } = clap::parse().configuration()?.leak();
    let (client, _) = mqtt::create_client(url.try_into()?);

    let mut buttons = client.subscribe(filter, 2).await?;
    while let Some(button) = buttons.recv().await {
        let Some(button_id) = button.topic.levels().last() else {
            tracing::error!("failed to identify button from topic '{}'", button.topic);
            continue;
        };

        tracing::debug!("button {button_id} pressed");
        let Some(responses) = mappings.get(button_id) else {
            tracing::warn!("no action(s) configured for button {button_id}");
            continue;
        };

        for Mapping { topic, payload } in responses {
            tracing::info!("publishing '{payload}' to '{topic}'");
            client
                .publish(*topic, payload.as_bytes(), AtMostOnce, false)
                .await?;
        }
    }

    Ok(())
}
