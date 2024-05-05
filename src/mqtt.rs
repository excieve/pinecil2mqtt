use crate::bulk::PinecilBulkData;

use anyhow::Result;
use tokio::sync::mpsc;
use rumqttc::{AsyncClient, MqttOptions, QoS};
use log::{debug, error, info};

#[derive(Debug, Clone)]
pub struct Message {
    topic: String,
    payload: String,
}

impl Message {
    pub fn from_pinecil_bulk_data(id: String, data: PinecilBulkData) -> Result<Message> {
        let payload = serde_json::to_string(&data)?;

        Ok(Message {
            topic: format!("pinecil/{}/bulk", id),
            payload,
        })
    }
}

pub struct MqttClient {
    channel_rx : mpsc::Receiver<Message>,
}

impl MqttClient {
    pub fn new(channel_rx: mpsc::Receiver<Message>) -> Self {
        Self {
            channel_rx,
        }
    }

    pub async fn run_sender(&mut self) -> Result<()> {
        let mut options = MqttOptions::new("pinecil2mqtt", "localhost", 1883);
        options.set_keep_alive(std::time::Duration::from_secs(60));

        let (client, mut eventloop) = AsyncClient::new(options, 10);

        tokio::spawn(async move {
            info!("Starting MQTT eventloop...");

            loop {
                match eventloop.poll().await {
                    Ok(event) => debug!("MQTT event: {:?}", event),
                    Err(e) => error!("MQTT eventloop error: {}", e),
                }
            }
        });

        while let Some(message) = self.channel_rx.recv().await {
            debug!("Sending message: {:?}", message);
            client.publish(message.topic, QoS::AtLeastOnce, false, message.payload).await?;
        }

        Ok(())
    }
}

