use rascam::*;
use reqwest::multipart;
use std::env;
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;
use chrono::Local;
use tokio::time;
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

    run(&info.cameras[0]).await?;
    Ok(())
}

async fn run(info: &CameraInfo) -> Result<(), Box<dyn std::error::Error>> {
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

    send_discord_message(static_path.to_str().unwrap()).await?;

    info!("Done!");
    Ok(())
}

async fn send_discord_message(file_path: &str) -> Result<(), Box<dyn std::error::Error>> {
    let discord_url = env::var("DISCORD_URL").unwrap_or_default();

    if discord_url.is_empty() {
        info!("No Discord URL provided... Skipping notification.");
        return Ok(());
    }

    let discord_client = reqwest::Client::new();

    let file_part = multipart::Part::file(file_path)?
        .mime_str("image/jpeg")?; // Specify the MIME type (optional, but helpful)

    let form = multipart::Form::new()
        .text("content", "New photo taken by Raspberry Pi Camera")
        .part("file", file_part);

    let response = discord_client.post(&discord_url).multipart(form).send().await;

    match response {
        Ok(_) => println!("Discord message sent!"),
        Err(err) => println!("Error: {}", err),
    }

    Ok(())
}