mod manager;
mod bulk;

use log::info;
use anyhow::Result;
use tokio::sync::mpsc;

use manager::{PinecilManager, PinecilManagerBtle};

#[tokio::main]
async fn main() -> Result<()> {
    env_logger::init();

    info!("started!");

    let manager = PinecilManagerBtle::new().await?;
    
    let (tx, mut rx) = mpsc::channel(32);

    tokio::spawn(async move {
        loop {
            let bulk_data = rx.recv().await.unwrap();
            info!("Received bulk data: {:?}", bulk_data);
        }
    });
    
    manager.process_events(tx).await?;

    Ok(())
}
