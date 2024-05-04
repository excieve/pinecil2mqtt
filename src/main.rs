use log::info;
use futures::stream::StreamExt;
use std::error::Error;

use btleplug::api::{Central, Manager as _, ScanFilter, CentralEvent, Peripheral};
use btleplug::platform::{Manager, Adapter};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    env_logger::init();

    info!("started!");

    let manager = PinecilManagerBtle::new().await?;
    manager.process_events().await?;

    Ok(())
}

trait PinecilManager {
    async fn process_events(&self) -> Result<(), Box<dyn Error>>;
}

// PinecilManagerBtle encapsulates the Pinecil BLE peripherals though the btleplug Manager API
struct PinecilManagerBtle {
    manager: Manager,
}

impl PinecilManagerBtle {
    async fn new() -> Result<Self, Box<dyn Error>> {
        let manager = Manager::new().await?;

        Ok(Self { manager })
    }

    async fn get_first_adapter(&self) -> Result<Adapter, Box<dyn Error>> {
        let adapter_list = self.manager.adapters().await?;
        if adapter_list.is_empty() {
            return Err("No adapters found".into());
        }

        Ok(adapter_list.into_iter().next().unwrap())
    }
}

impl PinecilManager for PinecilManagerBtle {
    // Process the events from the adapter continuously
    async fn process_events(&self) -> Result<(), Box<dyn Error>> {
        let central = self.get_first_adapter().await?;
        central.start_scan(ScanFilter::default()).await?;

        let mut events = central.events().await?;

        while let Some(event) = events.next().await {
            match event {
                CentralEvent::DeviceDiscovered(addr) => {
                    let device = central.peripheral(&addr).await?;
                    info!("Discovered device: {:?}", device.properties().await?);
                }
                CentralEvent::DeviceConnected(addr) => {
                    info!("Device connected: {:?}", addr);
                }
                CentralEvent::DeviceDisconnected(addr) => {
                    info!("Device disconnected: {:?}", addr);
                }
                _ => {}
            }
        }

        Ok(())
    }
}
