use chrono::Local;
use rppal::gpio::{ Gpio, InputPin };
use rascam::*;
use reqwest::Client;
use reqwest::multipart;
use std::env;
use std::path::{ Path, PathBuf };
use tokio::fs::File;
use tokio::io::AsyncWriteExt;
use tracing::{ info, error };
use std::time::{Duration, Instant};


/// Helper function to initialize and monitor motion detection using a PIR sensor.
///
/// # Arguments
/// * `pir_pin` - GPIO pin number (BCM numbering) for the PIR sensor.
fn setup_motion_detection(pir_pin: u8) -> Result<InputPin, Box<dyn std::error::Error>> {
    let gpio = Gpio::new()?;
    let pir_input = gpio.get(pir_pin)?.into_input();
    info!("Motion detection initialized on GPIO pin {}", pir_pin);
    Ok(pir_input)
}


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
        .text("content", "**New activity detected by Raspberry Pi Camera!**")
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
    // GPIO pin connected to the PIR sensor (BCM numbering)
    let pir_pin = 17;
    let pir_sensor = setup_motion_detection(pir_pin)?;
    
    let mut camera = SimpleCamera::new(info.clone()).unwrap();
    camera.activate().unwrap();

    info!("Camera activated. Waiting for motion...");

    let buffer_duration = Duration::from_secs(30);
    let mut last_capture_time = Instant::now() - buffer_duration; // Initialize as elapsed

    loop {
        if pir_sensor.is_high() {
            info!("Motion detected! Capturing image...");
            if last_capture_time.elapsed() >= buffer_duration {
                info!("Motion detected! Capturing image...");
                last_capture_time = Instant::now();

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
            } else {
                info!("Motion detected, but within buffer period. Skipping capture.");
            }
        }

        // Polling interval to avoid constant CPU usage
        tokio::time::sleep(Duration::from_millis(500)).await;
    }
}
