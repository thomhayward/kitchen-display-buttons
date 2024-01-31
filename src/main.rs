use mqtt::{Filter, QoS::AtMostOnce, Topic};
use serde::Deserialize;
use std::collections::HashMap;

#[derive(Deserialize)]
struct Config {
    #[serde(borrow)]
    url: &'static str,
    #[serde(borrow)]
    filter: &'static Filter,
    #[serde(borrow)]
    mappings: HashMap<&'static str, Vec<Mapping>>,
}

#[derive(Deserialize)]
struct Mapping {
    #[serde(borrow)]
    topic: &'static Topic,
    #[serde(borrow)]
    payload: &'static str,
}

#[tokio::main(flavor = "current_thread")]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt::init();

    let Config {
        url,
        filter,
        mappings,
    } = serde_yaml::from_str(include_str!("../config.yaml"))?;
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
