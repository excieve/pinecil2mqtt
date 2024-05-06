mod manager;
mod bulk;
mod mqtt;
mod config;

use log::{info, debug};
use anyhow::Result;
use tokio::sync::mpsc;

use manager::{PinecilManager, PinecilManagerBtle, PinecilBulkDataMessage};
use config::Config;


#[tokio::main]
async fn main() -> Result<()> {
    let config = Config::from_file("config.toml")?;

    env_logger::builder()
        .filter_level(config.log_level().parse()?)
        .init();

    info!("started!");

    let manager = PinecilManagerBtle::new().await?;

    let (manager_tx, mut manager_rx) = mpsc::channel(32);
    let (mqtt_tx, mqtt_rx) = mpsc::channel(32);

    let mut mqtt = mqtt::MqttClient::new(mqtt_rx, config.mqtt().clone());

    tokio::spawn(async move {
        mqtt.run_sender().await.unwrap();
    });

    tokio::spawn(async move {
        loop {
            let bulk_data_message: PinecilBulkDataMessage = manager_rx.recv().await.unwrap();
            debug!("Received bulk data message: {:?}", bulk_data_message);

            let message = mqtt::Message::from_pinecil_bulk_data(bulk_data_message.id, bulk_data_message.data).unwrap();
            mqtt_tx.send(message).await.unwrap();
        }
    });

    manager.process_events(manager_tx).await?;

    Ok(())
}
