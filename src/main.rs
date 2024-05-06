mod manager;
mod bulk;
mod mqtt;
mod config;
mod transformer;

use log::{info, debug};
use anyhow::Result;
use tokio::sync::mpsc;
use clap::Parser;

use manager::{PinecilManager, PinecilManagerBtle, PinecilBulkDataMessage};
use config::Settings;
use transformer::{PinecilDataWithLabels, FromPinecilBulkData};


#[derive(Parser, Debug)]
#[clap(version, about)]
struct Cli {
    /// Path to the configuration file
    #[clap(short, long)]
    config: Option<std::path::PathBuf>,

    /// Log level
    #[clap(short, long)]
    log_level: Option<String>,
}

#[tokio::main]
async fn main() -> Result<()> {
    let args = Cli::parse();

    let mut config = match args.config {
        Some(config_path) => Settings::new(config_path)?,
        None => Settings::new(std::path::PathBuf::default())?,
    };

    if let Some(log_level) = args.log_level {
        config.set_log_level(log_level);
    }

    env_logger::builder()
        .filter_level(config.log_level().parse()?)
        .init();

    info!("{} v{} started!", env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION"));

    let manager = PinecilManagerBtle::new().await?;

    let (manager_tx, mut manager_rx) = mpsc::channel::<PinecilBulkDataMessage>(32);
    let (mqtt_tx, mqtt_rx) = mpsc::channel(32);

    let mut mqtt = mqtt::MqttClient::new(mqtt_rx, config.mqtt().clone());

    tokio::spawn(async move {
        mqtt.run_sender().await.unwrap();
    });

    // Main reactor loop that receives bulk data messages from the manager,
    // transforms them and passes them to the MQTT sender
    tokio::spawn(async move {
        while let Some(bulk_data_message) = manager_rx.recv().await {
            debug!("Received bulk data message: {:?}", bulk_data_message);

            let bulk_data = bulk_data_message.data;
            let data = PinecilDataWithLabels::from_pinecil_bulk_data(&bulk_data)
                .with_timestamp(bulk_data_message.timestamp);

            let message = mqtt::Message::from_serialize(bulk_data_message.id, &data).unwrap();
            mqtt_tx.send(message).await.unwrap();
        }
    });

    manager.process_events(manager_tx).await?;

    Ok(())
}
