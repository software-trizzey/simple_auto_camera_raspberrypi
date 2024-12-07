use rascam::*;
use tracing::{error, info};
use dotenv::dotenv;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();

    let info = info().unwrap();
    if info.cameras.len() < 1 {
        error!("Found 0 cameras. Exiting");
        ::std::process::exit(1);
    }
    info!("{}", info);

    simple_auto_camera_raspberrypi::run(&info.cameras[0]).await?;
    Ok(())
}
