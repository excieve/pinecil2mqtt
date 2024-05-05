mod manager;
mod bulk;

use log::{debug, error, info};
use std::error::Error;

use manager::{PinecilManager, PinecilManagerBtle};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    env_logger::init();

    info!("started!");

    let manager = PinecilManagerBtle::new().await?;
    manager.process_events().await?;

    Ok(())
}
