use rascam::*;
use tracing::{error, info};
use dotenv::dotenv;
use tracing_subscriber;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();

    tracing_subscriber::fmt()
    .with_target(false) // Hides module paths from logs
    .with_level(true)
    .with_thread_ids(true) // Optionally include thread IDs
    .init();

    let info = info().unwrap();
    if info.cameras.len() < 1 {
        error!("Found 0 cameras. Exiting");
        ::std::process::exit(1);
    }
    info!("{}", info);

    if let Err(e) = simple_auto_camera_raspberrypi::run(&info.cameras[0]).await {
        error!("Application error: {}", e);
        ::std::process::exit(1);
    }
    Ok(())
}
