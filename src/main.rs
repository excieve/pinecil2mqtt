mod manager;
mod bulk;

use log::{debug, error, info};
use anyhow::Result;

use manager::{PinecilManager, PinecilManagerBtle};

#[tokio::main]
async fn main() -> Result<()> {
    env_logger::init();

    info!("started!");

    let manager = PinecilManagerBtle::new().await?;
    manager.process_events().await?;

    Ok(())
}
