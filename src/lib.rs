use chrono::Local;
use rascam::*;
use futures::stream::TryStreamExt;
use reqwest::{Body, Client};
use std::env;
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;
use std::path::Path;
use tokio::fs::File;
use tokio::time;
use tokio_util::codec::{BytesCodec, FramedRead};
use tracing::info;


fn file_to_body(file: File) -> Body {
    let stream = FramedRead::new(file, BytesCodec::new());
    let body = Body::wrap_stream(stream);
    body
}

async fn send_discord_message(file: &File) -> Result<(), Box<dyn std::error::Error>> {
    let discord_url = env::var("DISCORD_URL").unwrap_or_default();
    if discord_url.is_empty() {
        info!("No Discord URL provided... Skipping notification.");
        return Ok(());
    }

    let client = Client::new();
    let response = client.post(&discord_url)
        .body(file_to_body(file))
        .send()
        .await?;

    info!("Discord message sent");

    Ok(())
}

pub async fn run(info: &CameraInfo) -> Result<(), Box<dyn std::error::Error>> {
    let mut camera = SimpleCamera::new(info.clone()).unwrap();
    camera.activate().unwrap();

    time::sleep(time::Duration::from_millis(2000)).await;

    let b = camera.take_one().unwrap();

    let current_directory = env::current_dir().unwrap();
    let mut static_path = PathBuf::from(current_directory);
    static_path.push("static");

    let timestamp = Local::now().format("%Y%m%d%H%M%S").to_string();
    let filename = format!("raspi-camera-{}.jpg", timestamp);
    static_path.push(&filename);

    info!("Creating file at {:?}", static_path);
    let mut file = File::create(&static_path).unwrap();
    file.write_all(&b).unwrap();

    info!("Saved image as {}", filename);

    send_discord_message(&file)?;

    info!("Done!");
    Ok(())
}
