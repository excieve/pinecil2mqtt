use anyhow::{anyhow, bail, Result};
use btleplug::api::Peripheral as _;
use btleplug::platform::Peripheral;

#[derive(Debug)]
pub struct PinecilBulkData {
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

pub trait PinecilBulkQuery {
    async fn query_pinecil_info(&self) -> Result<String>;
    async fn query_bulk_data(&self) -> Result<PinecilBulkData>;
}

pub struct PinecilBulkQueryBtle<'a> {
    device: &'a Peripheral,
}

impl<'a> PinecilBulkQueryBtle<'a> {
    pub fn new(device: &'a Peripheral) -> Self {
        Self { device }
    }
}

impl<'a> PinecilBulkQuery for PinecilBulkQueryBtle<'a> {
    // Query the Pinecil info from the device's bulk service characteristics
    async fn query_pinecil_info(&self) -> Result<String> {
        let crx = self.device.characteristics();

        if let Some(build_crx) = crx
            .iter()
            .find(|c| c.uuid == "9eae1003-9d0d-48c5-aa55-33e27f9bc533".parse().unwrap()) {
            let build = self.device.read(build_crx).await?;
            let build_str = String::from_utf8(build).expect("Could not convert build bytes to string");

            Ok(build_str)
        } else {
            Err(anyhow!("Could not find build characteristic"))
        }
    }

    async fn query_bulk_data(&self) -> Result<PinecilBulkData> {
        todo!()
    }
}
