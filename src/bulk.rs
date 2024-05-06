use anyhow::{bail, Result};
use btleplug::api::Peripheral as _;
use btleplug::platform::Peripheral;
use serde::{Serialize, Deserialize};
use bincode::deserialize;
use uuid::Uuid;


const PINECIL_BULK_LIVE_CHAR_UUID: Uuid = Uuid::from_u128(0x9eae1001_9d0d_48c5_aa55_33e27f9bc533);
const PINECIL_BULK_BUILD_CHAR_UUID: Uuid = Uuid::from_u128(0x9eae1003_9d0d_48c5_aa55_33e27f9bc533);
const PINECIL_BULK_DEVICE_ID_CHAR_UUID: Uuid = Uuid::from_u128(0x9eae1005_9d0d_48c5_aa55_33e27f9bc533);

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PinecilBulkData {
    pub live_temp: u32,
    pub set_point: u32,
    pub voltage: u32,
    pub handle_temp: u32,
    pub power_level: u32,
    pub power_source: u32,
    pub tip_resistance: u32,
    pub uptime: u32,
    pub last_movement: u32,
    pub max_tip_temp: u32,
    pub tip_voltage: u32,
    pub hall_sensor: u32,
    pub operating_mode: u32,
    pub power: u32,
}

pub trait PinecilBulkQuery {
    async fn query_pinecil_info(&self) -> Result<(String, String)>;
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
    async fn query_pinecil_info(&self) -> Result<(String, String)> {
        let crx = self.device.characteristics();

        let build_str: String;
        let device_id_str: String;

        if let Some(build_crx) = crx
            .iter()
            .find(|c| c.uuid == PINECIL_BULK_BUILD_CHAR_UUID) {
            let build = self.device.read(build_crx).await?;
            build_str = String::from_utf8(build)?;
        } else {
            bail!("Could not find build characteristic")
        }

        if let Some(device_id_crx) = crx
            .iter()
            .find(|c| c.uuid == PINECIL_BULK_DEVICE_ID_CHAR_UUID) {
            let device_id = self.device.read(device_id_crx).await?;
            device_id_str = format!("{:08X}", u32::from_le_bytes(device_id.try_into().unwrap()));
        } else {
            bail!("Could not find device ID characteristic")
        }

        Ok((build_str, device_id_str))
    }

    // Query and decode the Pinecil bulk data from the device's bulk service characteristics
    // into a `PinecilBulkData` struct
    async fn query_bulk_data(&self) -> Result<PinecilBulkData> {
        let crx = self.device.characteristics();

        let bulk_crx = match crx
            .iter()
            .find(|c| c.uuid == PINECIL_BULK_LIVE_CHAR_UUID) {
            Some(crx) => crx,
            None => bail!("Could not find bulk characteristic")
        };

        let raw_bulk_data = self.device.read(bulk_crx).await?;
        let bulk_data = deserialize(&raw_bulk_data[..])?;

        Ok(bulk_data)
    }
}
