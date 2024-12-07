use rascam::*;
use reqwest::Client;
use std::env;
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;
use chrono::Local;
use std::{thread, time};
use serde_json::json;
use tracing::{error, info};
use dotenv::dotenv;

fn main() {
    dotenv().ok();

    let info = info().unwrap();
    if info.cameras.len() < 1 {
        error!("Found 0 cameras. Exiting");
        // note that this doesn't run destructors
        ::std::process::exit(1);
    }
    info!("{}", info);

    run(&info.cameras[0]);
}


async fn run(info: &CameraInfo) {
    let mut camera = SimpleCamera::new(info.clone()).unwrap();
    camera.activate().unwrap();

    let sleep_duration = time::Duration::from_millis(2000);
    thread::sleep(sleep_duration);

    let b = camera.take_one().unwrap();

    let current_directory = env::current_dir().unwrap();
    let mut static_path = PathBuf::from(current_directory);
    static_path.push("static");

    let timestamp = Local::now().format("%Y%m%d%H%M%S").to_string();
    let filename = format!("raspi-camera-{}.jpg", timestamp);
    static_path.push(&filename);

    println!("Creating file at {:?}", static_path);
    File::create(static_path).unwrap().write_all(&b).unwrap();

    info!("Saved image as {}", filename);

    let filepath = static_path.to_str().unwrap();
    send_discord_message(filepath).await?;

    println!("Done!");
}


async fn send_discord_message(filepath: &str) -> Result<()> {
    let discord_url = env::var("DISCORD_URL")?;

    if discord_url.is_empty() {
        return Ok(println!("No Discord URL provided... Skipping notification."));
    }

    let discord_client = reqwest::Client::new();

    let payload = json!({
        "file": File::open(filepath)?,
        "content": format!(
            "New photo taken by Raspberyy Pi Camera"
        )
    });

    let response = discord_client.post(&discord_url)
        .json(&payload)
        .send()
        .await;

    match response {
        Ok(_) => println!("Discord message sent!"),
        Err(err) => println!("Error: {}", err),
    }

    Ok(())
}
