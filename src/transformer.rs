use serde::Serialize;
use chrono::{DateTime, Utc};

use crate::bulk::PinecilBulkData;

pub trait FromPinecilBulkData {
    fn from_pinecil_bulk_data(data: &PinecilBulkData) -> Self;
    fn with_timestamp(self, timestamp: DateTime<Utc>) -> Self;
}

#[derive(Debug, Clone, Serialize)]
pub struct PinecilDataWithLabels {
    #[serde(flatten)]
    bulk_data: PinecilBulkData,
    timestamp: i64,
    power_source_label: String,
    operating_mode_label: String,
}

impl FromPinecilBulkData for PinecilDataWithLabels {
    fn from_pinecil_bulk_data(data: &PinecilBulkData) -> Self {
        Self {
            bulk_data: data.clone(),
            timestamp: 0,
            // https://github.com/Ralim/IronOS/blob/c308fe8cc2bd8e1e93e9441d7e8fc537a79a2259/source/Core/BSP/Pinecilv2/ble_handlers.cpp#L283-L315
            power_source_label: match data.power_source {
                0 => "DCIN".to_string(),
                1 => "USB".to_string(),
                2 => "PD+VBUS".to_string(),
                3 => "PD".to_string(),
                _ => "Unknown".to_string(),
            },
            // https://github.com/Ralim/IronOS/blob/c308fe8cc2bd8e1e93e9441d7e8fc537a79a2259/source/Core/Threads/OperatingModes/OperatingModes.h#L27-L35
            operating_mode_label: match data.operating_mode {
                0 => "Idle".to_string(),
                1 => "Soldering".to_string(),
                2 => "Boost".to_string(),
                3 => "Sleeping".to_string(),
                4 => "Settings".to_string(),
                5 => "Debug".to_string(),
                _ => "Unknown".to_string(),
            },
        }
    }

    fn with_timestamp(mut self, timestamp: DateTime<Utc>) -> Self {
        self.timestamp = timestamp.timestamp();
        self
    }
}
