# Pinecil V2 BLE to MQTT Gateway

**This is actively being developed and is not yet ready for use.**

This project is a BLE to MQTT gateway for the Pinecil V2 soldering iron.
It is based on the [Pinecil V2 BLE Services](https://github.com/Ralim/IronOS/blob/dev/Documentation/Bluetooth.md) from IronOS.

## Features

- Automatically discover and connect to the Pinecil V2 soldering iron via BLE
- Publish data provided by the soldering iron to an MQTT broker
- Optionally authenticate with the MQTT broker

## Installation

1. Clone this repository
2. Run `cargo build --release` to build the project
3. Set up a configuration file (see [Configuration](#configuration))
4. Run the binary from the `target/release` directory
5. Enjoy!

## Configuration

The configuration file is a simple TOML file. Here is an example configuration:

```toml
log_level = "info"

[mqtt]
host = "mqtt.example.com"
port = 1883
username = "username"
password = "password"
```

# Roadmap

- [ ] Containerize the application and publish it to Docker Hub
- [ ] Write unit tests (I know, I know)
- [ ] Add support for the settings service read characteristics
- [ ] Add support for the settings service write characteristics via MQTT
- [ ] Home Assistant integration
- [ ] Multiple Pinecils support
- [ ] Test on more platforms (currently only tested on Linux)
