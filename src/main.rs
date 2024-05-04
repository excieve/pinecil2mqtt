use log::{debug, info, error};
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

    // Check that bulk, live and settings services are available
    fn has_required_services(device: &Peripheral) -> bool {
        let services = device.services();

        // Note that we only need the bulk service currently (might change in the future),
        // but IronOS recommends to check for all three
        services
            .iter()
            .filter(|s| {
                s.uuid == "9eae1000-9d0d-48c5-aa55-33e27f9bc533".parse().unwrap() ||
                s.uuid == "d85ef000-168e-4a71-aa55-33e27f9bc533".parse().unwrap() ||
                s.uuid == "f6d80000-5a10-4eba-aa55-33e27f9bc533".parse().unwrap()
            })
            .count() == 3
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

                    device.discover_services().await?;

                    if !Self::has_required_services(&device) {
                        error!("Pinecil does not have the required services,\
                            check the firmware version and update if necessary.");
                        continue;
                    }
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

struct PinecilBulkData {
    live_temp: u32,
    set_point: u32,
    voltage: u32,
    handle_temp: u32,
    power_level: u32,
    power_source: u32,
    tip_resistance: u32,
    uptime: u32,
    last_movement: u32,
    max_tip_temp: u32,
    tip_voltage: u32,
    hall_sensor: u32,
    operating_mode: u32,
    power: u32,
}

trait PinecilBulkQuery {
    async fn query_pinecil_info(&self) -> Result<String, Box<dyn Error>>;
    async fn query_bulk_data(&self) -> Result<PinecilBulkData, Box<dyn Error>>;
}

struct PinecilBulkQueryBtle {
    device: Peripheral,
}

impl PinecilBulkQueryBtle {
    fn new(device: Peripheral) -> Self {
        Self { device }
    }
}

impl PinecilBulkQuery for PinecilBulkQueryBtle {
    async fn query_pinecil_info(&self) -> Result<String, Box<dyn Error>> {
        todo!()
    }

    async fn query_bulk_data(&self) -> Result<PinecilBulkData, Box<dyn Error>> {
        todo!()
    }
}
