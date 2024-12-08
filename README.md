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

## Automating the camera via Systemctl

_Note: This will run the program every 30 minutes. Adjust the `OnCalendar` value (ex. OnCalendar=hourly or every minute: OnCalendar=*:0/1) in the timer file for a different interval._

1. ***Build the Rust Program**

```bash
cargo build --release
sudo cp target/release/simple_auto_camera_raspberrypi /usr/local/bin/simple_auto_camera_raspberrypi
```

2. **Create the Service File**
```bash
sudo nano /etc/systemd/system/run_camera.service
```

File Content:

```ini
[Unit]
Description=Run Raspberry Pi Camera
After=network.target

[Service]
ExecStart=/usr/local/bin/simple_auto_camera_raspberrypi
Restart=on-failure
Environment="DISCORD_URL=<your_discord_server_url"
WorkingDirectory=/home/pi/Projects/simple_auto_camera_raspberrypi

[Install]
WantedBy=multi-user.target
```

3. **Create the Timer File**

```bash
sudo nano /etc/systemd/system/run_camera.timer
```

File Content:

```ini
[Unit]
Description=Run Raspberry Pi Camera Every 30 minutes

[Timer]
OnCalendar=*:0/30
Persistent=true

[Install]
WantedBy=timers.target
```

4. **Start and Enable the Timer**

```bash
sudo systemctl daemon-reload
sudo systemctl enable run_camera.timer
sudo systemctl start run_camera.timer
```

Note: You can manully test the service using `sudo systemctl start run_camera.service`

5. **Verify the Timer**

```bash
systemctl list-timers --all
```


## References:
- This [project](https://github.com/pedrosland/rascam) provides an API for our program to interact with the raspberry pi camera.

