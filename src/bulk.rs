use anyhow::Result;
use btleplug::platform::Peripheral;

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
    async fn query_pinecil_info(&self) -> Result<String>;
    async fn query_bulk_data(&self) -> Result<PinecilBulkData>;
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
    async fn query_pinecil_info(&self) -> Result<String> {
        todo!()
    }

    async fn query_bulk_data(&self) -> Result<PinecilBulkData> {
        todo!()
    }
}
