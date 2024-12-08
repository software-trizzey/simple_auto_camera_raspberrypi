use chrono::Local;
use rascam::*;
use reqwest::Client;
use reqwest::multipart;
use std::env;
use std::path::{ Path, PathBuf };
use tokio::fs::File;
use tokio::time;
use tokio::io::AsyncWriteExt;
use tracing::{ info, error };


async fn send_discord_message(file_path: &Path) -> Result<(), Box<dyn std::error::Error>> {
    let discord_url = env::var("DISCORD_URL").unwrap_or_default();
    if discord_url.is_empty() {
        info!("No Discord URL provided... Skipping notification.");
        return Ok(());
    }

    let mut file = File::open(file_path).await?;
    let mut file_contents = Vec::new();
    tokio::io::AsyncReadExt::read_to_end(&mut file, &mut file_contents).await?;

    let form = multipart::Form::new()
        .text("content", "**New photo from Raspberry Pi Camera!**")
        .part(
            "file",
            multipart::Part::bytes(file_contents).file_name(
                file_path.file_name().unwrap().to_string_lossy().to_string(),
            ),
        );

    let client = Client::new();
    let response = client.post(&discord_url).multipart(form).send().await?;

    if response.status().is_success() {
        info!("Message sent successfully");
    } else {
        error!(
            "Failed to send message: {:?}",
            response.text().await.unwrap_or_else(|_| "Unknown error".to_string())
        );
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
    file.write_all(&image_buffer).await?;
    file.flush().await?;
    drop(file); // ensure file is closed

    info!("Saved image as {}", filename);

    send_discord_message(&static_path).await?;

    info!("Done!");
    Ok(())
}
