use log::{debug, error, info};
use futures::StreamExt;
use anyhow::{anyhow, Result};
use tokio::sync::mpsc;
use chrono::{DateTime, Utc};

use btleplug::api::{Central, CentralEvent, Manager as _, Peripheral as _, ScanFilter};
use btleplug::platform::{Adapter, Manager, Peripheral};

use crate::bulk::{PinecilBulkQuery, PinecilBulkQueryBtle, PinecilBulkData};


#[derive(Debug, Clone)]
pub struct PinecilBulkDataMessage {
    pub id: String,
    pub timestamp: DateTime<Utc>,
    pub data: PinecilBulkData,
}

pub trait PinecilManager {
    async fn process_events(&self, tx: mpsc::Sender<PinecilBulkDataMessage>) -> Result<()>;
}

// PinecilManagerBtle encapsulates the Pinecil BLE peripherals though the btleplug Manager API
pub struct PinecilManagerBtle {
    manager: Manager,
}

impl PinecilManagerBtle {
    pub async fn new() -> Result<Self> {
        let manager = Manager::new().await?;

        Ok(Self { manager })
    }

    async fn get_first_adapter(&self) -> Result<Adapter> {
        let adapter_list = self.manager.adapters().await?;
        if adapter_list.is_empty() {
            return Err(anyhow!("No adapters found"));
        }

        Ok(adapter_list.into_iter().next().unwrap())
    }

    // Identify the Pinecil from the passed peripheral object by searching the initial services
    async fn is_pinecil(device: &Peripheral) -> Result<bool> {
        let properties = device.properties().await?.unwrap();

        Ok(properties.services.iter().any(|s| s.to_string().contains("9eae1000-9d0d-48c5-aa55-33e27f9bc533")))
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

    // Launch a poller to query the Pinecil bulk data from the device characteristics
    async fn poll_bulk_data(&self, device: &Peripheral, id: String, tx: &mpsc::Sender<PinecilBulkDataMessage>) -> Result<()> {
        let mut interval = tokio::time::interval(tokio::time::Duration::from_secs(1));
        let bulk = PinecilBulkQueryBtle::new(device);

        loop {
            interval.tick().await;
            let bulk_data = bulk.query_bulk_data().await?;
            debug!("Bulk data: {:?}", bulk_data);

            tx.send(PinecilBulkDataMessage{
                id: id.clone(),
                timestamp: Utc::now(),
                data: bulk_data,
            }).await?;
        }
    }
}

impl PinecilManager for PinecilManagerBtle {
    // Process the events from the adapter continuously
    async fn process_events(&self, tx: mpsc::Sender<PinecilBulkDataMessage>) -> Result<()> {
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
                    debug!("Discovered device properties: {:?}", device.properties().await?);

                    if !device.is_connected().await? {
                        device.connect().await?;
                    }
                }
                CentralEvent::DeviceUpdated(addr) => {
                    let device = central.peripheral(&addr).await?;
                    if !Self::is_pinecil(&device).await? {
                        continue;
                    }

                    debug!("Updated device properties: {:?}", device.properties().await?);

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
                    debug!("Connected device properties: {:?}", device.properties().await?);

                    device.discover_services().await?;

                    if !Self::has_required_services(&device) {
                        error!("Pinecil does not have the required services,\
                            check the firmware version and update if necessary.");
                        continue;
                    }

                    let bulk_query = PinecilBulkQueryBtle::new(&device);

                    // Fetch and print the Pinecil info (build version, device ID), skip on error
                    let pinecil_id = match bulk_query.query_pinecil_info().await {
                        Ok((version, pinecil_id)) => {
                            info!("Pinecil firmware verison: {}, device ID: {}", version, pinecil_id);
                            pinecil_id
                        }
                        Err(e) => {
                            error!("Failed to fetch Pinecil info: {}", e);
                            continue;
                        }
                    };

                    // Periodically poll the Pinecil bulk data as IronOS doesn't support
                    // notifications. At the same time, it's more efficient to poll in bulk rather
                    // than querying each value separately in the live data service.
                    if let Err(e) = self.poll_bulk_data(&device, pinecil_id, &tx).await {
                        error!("Failed to poll bulk data: {}", e);
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
