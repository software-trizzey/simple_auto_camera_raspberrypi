use rascam::*;
use reqwest::Client;
use reqwest::multipart;
use std::env;
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;
use tokio::time;
use chrono::Local;
use std::{thread, time};
use serde_json::json;
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

async fn run(info: &CameraInfo) -> Result<()> {
    let mut camera = SimpleCamera::new(info.clone()).unwrap();
    camera.activate().unwrap();

    tokio::time::sleep(time::Duration::from_millis(2000)).await;

    let b = camera.take_one().unwrap();

    let current_directory = env::current_dir().unwrap();
    let mut static_path = PathBuf::from(current_directory);
    static_path.push("static");

    let timestamp = Local::now().format("%Y%m%d%H%M%S").to_string();
    let filename = format!("raspi-camera-{}.jpg", timestamp);
    static_path.push(&filename);

    println!("Creating file at {:?}", static_path);
    let mut file = File::create(&static_path).unwrap();
    file.write_all(&b).unwrap();

    info!("Saved image as {}", filename);

    let mut contents = String::new();
    send_discord_message(static_path.to_str().unwrap()).await?;

    info!("Done!");
}


async fn send_discord_message(file_contents: &str) -> Result<()> {
    let discord_url = env::var("DISCORD_URL")?;

    if discord_url.is_empty() {
        info!("No Discord URL provided... Skipping notification.");
        return Ok(());
    }

    let discord_client = reqwest::Client::new();

    let form = multipart::Form::new()
    .text("content", "New photo taken by Raspberry Pi Camera")
    .file("file", file_contents)?;

    let response = discord_client.post(&discord_url)
        .multipart(form)
        .send()
        .await;

    match response {
        Ok(_) => info!("Discord message sent!"),
        Err(err) => error!("Error: {}", err),
    }

    Ok(())
}
