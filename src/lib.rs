use chrono::Local;
use rascam::*;
use reqwest::{Body, Client};
use std::env;
use std::io::Write;
use std::path::{ Path, PathBuf };
use tokio::fs::File;
use tokio::time;
use tokio_util::codec::{BytesCodec, FramedRead};
use tracing::{ info, error };


async fn send_discord_message(file_path: &Path) -> Result<(), Box<dyn std::error::Error>> {
    let discord_url = env::var("DISCORD_URL").unwrap_or_default();
    if discord_url.is_empty() {
        info!("No Discord URL provided... Skipping notification.");
        return Ok(());
    }

    let file = File::open(file_path).await?;
    let stream = FramedRead::new(file, BytesCodec::new());
    let wrapped_file_body = Body::wrap_stream(stream);

    let client = Client::new();
    let response = client.post(&discord_url)
        .body(wrapped_file_body)
        .send()
        .await?;

    if response.status().is_success() {
        info!("Message sent successfully");
    } else {
        error!("Failed to send message: {:?}", response.text().await?);
    }

    Ok(())
}

pub async fn run(info: &CameraInfo) -> Result<(), Box<dyn std::error::Error>> {
    let mut camera = SimpleCamera::new(info.clone()).unwrap();
    camera.activate().unwrap();

    time::sleep(time::Duration::from_millis(2000)).await;

    let image_buffer = camera.take_one().unwrap();

    let current_directory = env::current_dir().unwrap();
    let mut static_path = PathBuf::from(current_directory);
    static_path.push("static");

    let timestamp = Local::now().format("%Y%m%d%H%M%S").to_string();
    let filename = format!("raspi-camera-{}.jpg", timestamp);
    static_path.push(&filename);

    info!("Creating file at {:?}", static_path);
    let mut file = File::create(&static_path).await.unwrap();
    file.write_all(image_buffer).await?;
    file.flush().await?;
    drop(file); // ensure file is closed

    info!("Saved image as {}", filename);

    send_discord_message(&static_path).await?;

    info!("Done!");
    Ok(())
}
