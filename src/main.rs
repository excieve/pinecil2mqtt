mod manager;
mod bulk;
mod mqtt;

use log::{info, debug};
use anyhow::Result;
use tokio::sync::mpsc;

use manager::{PinecilManager, PinecilManagerBtle};

#[tokio::main]
async fn main() -> Result<()> {
    env_logger::init();

    info!("started!");

    let manager = PinecilManagerBtle::new().await?;

    let (manager_tx, mut manager_rx) = mpsc::channel(32);
    let (mqtt_tx, mqtt_rx) = mpsc::channel(32);

    let mut mqtt = mqtt::MqttClient::new(mqtt_rx);

    tokio::spawn(async move {
        mqtt.run_sender().await.unwrap();
    });

    tokio::spawn(async move {
        loop {
            let bulk_data = manager_rx.recv().await.unwrap();
            debug!("Received bulk data: {:?}", bulk_data);

            let message = mqtt::Message::from_pinecil_bulk_data("1".to_string(), bulk_data).unwrap();
            mqtt_tx.send(message).await.unwrap();
        }
    });

    manager.process_events(manager_tx).await?;

    Ok(())
}
