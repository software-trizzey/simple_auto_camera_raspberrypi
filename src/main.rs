use rascam::*;
use std::env;
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;
use chrono::Local;
use std::{thread, time};
use tracing::{error, info};

fn main() {

    let info = info().unwrap();
    if info.cameras.len() < 1 {
        error!("Found 0 cameras. Exiting");
        // note that this doesn't run destructors
        ::std::process::exit(1);
    }
    info!("{}", info);

    run(&info.cameras[0]);
}


// TODO: move this to lib.rs
fn run(info: &CameraInfo) {
    let mut camera = SimpleCamera::new(info.clone()).unwrap();
    camera.activate().unwrap();

    let sleep_duration = time::Duration::from_millis(2000);
    thread::sleep(sleep_duration);

    let b = camera.take_one().unwrap();

    let current_directory = env::current_dir().unwrap();
    // Construct the path to the static folder
    let mut static_path = PathBuf::from(current_directory);
    static_path.push("static");

    // Generate a unique filename using the current timestamp
    let timestamp = Local::now().format("%Y%m%d%H%M%S").to_string();
    let filename = format!("raspi-camera-{}.jpg", timestamp);
    static_path.push(&filename);

    println!("Creating file at {:?}", static_path);
    File::create(static_path).unwrap().write_all(&b).unwrap();

    info!("Saved image as {}", filename);
}
