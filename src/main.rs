use log::info;
use futures::stream::StreamExt;

use btleplug::api::{Central, Manager as _, ScanFilter, CentralEvent, Peripheral};
use btleplug::platform::Manager;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();

    info!("started!");

    // Initialise the btleplug manager and central
    let manager = Manager::new().await?;

    let adapter_list = manager.adapters().await?;
    if adapter_list.is_empty() {
        return Err("No Bluetooth adapters found".into());
    }

    let central = adapter_list.into_iter().nth(0).unwrap();

    // Start the event loop and scan for devices
    let mut events = central.events().await?;

    central.start_scan(ScanFilter::default()).await?;

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
