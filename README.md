# A Simple Automated Raspberry Pi Camera

## Overview
Simple rust program that captures images on a Raspberry Pi.

## Running program

_Note: This was tested using a Raspberry Pi 4 Model B Rev 1.4 and Ardu Camera_

1. Clone repo to Raspberry Pi `git clone https://github.com/software-trizzey/simple_auto_camera_raspberrypi.git`
1. Navigate to project `cd simple_auto_camera_raspberrypi`
1. Build program `cargo build --release`
1. Run production version `./release/simple_auto_camera_raspberrypi.bin`

Assuming the camera is setup correctly and no errors were encountered, a new image should appear in the `static` directory.

### Transfer images from Raspberry Pi to host machine via `ssh`

**Run command below on host machine:**
```bash
scp -r  [pi_username]@[raspberrypi_ip_address]:[path_to_program_on_pi]/simple_auto_camera_raspberrypi/static [directory_on_host_machine]
```

**Example:**
```bash
scp -r  pi@[ip-address-redacted]:/home/pi/Projects/simple_auto_camera_raspberrypi/static ~/Desktop/pi-images/
```

## References:
- This [project](https://github.com/pedrosland/rascam) provides an API for our program to interact with the raspberry pi camera.