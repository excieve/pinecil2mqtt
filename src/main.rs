use log::{debug, info};
use futures::stream::StreamExt;
use std::error::Error;

use btleplug::api::{Central, Manager as _, ScanFilter, CentralEvent, Peripheral as _};
use btleplug::platform::{Manager, Adapter, Peripheral};

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

    // Identify the Pinecil from the passed peripheral object by searching the name substring
    async fn is_pinecil(device: &Peripheral) -> Result<bool, Box<dyn Error>> {
        let properties = device.properties().await?.unwrap();
        let name = properties.local_name.unwrap_or_default();

        Ok(name.to_lowercase().contains("pinecil"))
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
                    if !Self::is_pinecil(&device).await? {
                        continue;

                    }
                    info!("Discovered Pinecil: {:?}", device.address());
                    debug!("Device properties: {:?}", device.properties().await?);

                    if !device.is_connected().await? {
                        device.connect().await?;
                    }
                }
                CentralEvent::DeviceConnected(addr) => {
                    let device = central.peripheral(&addr).await?;
                    if !Self::is_pinecil(&device).await? {
                        continue;
                    }

                    info!("Pinecil connected: {:?}", device.address());
                    debug!("Device properties: {:?}", device.properties().await?);
                }
                CentralEvent::DeviceDisconnected(addr) => {
                    let device = central.peripheral(&addr).await?;
                    if !Self::is_pinecil(&device).await? {
                        continue;
                    }

                    info!("Pinecil disconnected: {:?}", device.address());
                }
                _ => {}
            }
        }

        Ok(())
    }
}
