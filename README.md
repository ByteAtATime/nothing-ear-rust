# nothing-ear-rust

A Rust command-line utility to fetch the battery information of Nothing Ear devices.

## Overview

This utility connects to Nothing Ear devices via Bluetooth RFCOMM, sends a specific command to query the battery status, and then parses the response to provide a JSON output containing the battery level and charging status of each connected device (left earbud, right earbud, and case).

## Features

- Connects to Nothing Ear devices using Bluetooth.
- Fetches battery level and charging status.
- Outputs data in a structured JSON format.
- Retries connection in case of initial failure.
- Identifies connected devices (Left Ear, Right Ear, Case).

## Prerequisites

- Rust programming language environment (install from [rustup.rs](https://rustup.rs/)).
- A Linux system with Bluetooth support.
- `bluez` Bluetooth stack installed on your system.
- A Nothing Ear device paired with your system.

## Installation

1. Clone the repository:

   ```bash
   git clone https://github.com/ByteAtATime/nothing-ear-rust.git
   cd nothing-ear-rust
   ```

2. Build the project:

   ```bash
   cargo build --release
   ```

## Usage

1. Ensure your Nothing Ear device is paired and connected to your system.
2. Run the utility:

   ```bash
   ./target/release/nothing-ear-rust
   ```

## Output

The utility outputs a JSON string to the console. For example:

```json
[
  {
    "device_type": "RightEar",
    "recharging": false,
    "battery_level": 90
  },
  {
    "device_type": "LeftEar",
    "recharging": false,
    "battery_level": 80
  },
  {
    "device_type": "Case",
    "recharging": true,
    "battery_level": 100
  }
]
```

-   `device_type`: Indicates the type of device (LeftEar, RightEar, Case, or Unknown).
-   `recharging`:  A boolean value indicating whether the device is currently charging.
-   `battery_level`: An integer representing the battery level (0-100).

If no Ear device is detected or the connection fails, the output will be:

```json
{"error": "Ear not detected"}
```

## Code Structure

-   `main.rs`: Contains the main logic for connecting to the device, sending the command, receiving the response, and parsing the battery information.
-   `device.rs`: Defines the `DeviceType` enum to represent the different Nothing Ear devices.

## Constants

-   `EAR_ADDRESS`: The first 3 bytes of the MAC address of the Nothing Ear device.
-   `EAR_CHANNEL`: The RFCOMM channel used for communication.
-   `EAR_BATTERY`: The command byte sequence sent to query the battery status.
-   `RETRY`: The number of times the utility will attempt to reconnect if the initial connection fails.
-   `BATTERY_STATUS_2`: A constant used to identify the specific battery status response.
-   `RECHARGING_MASK`: A bitmask used to extract the recharging status from the response.
-   `BATTERY_MASK`: A bitmask used to extract the battery level from the response.

## Dependencies

-   `bluer`: A Rust library for interacting with Bluetooth devices.
-   `tokio`: An asynchronous runtime for Rust.
-   `serde`: A framework for serializing and deserializing Rust data structures.
-   `serde_json`: A library for working with JSON data in Rust.

## Error Handling

The utility handles potential errors during:

-   Adapter initialization and powering.
-   Device discovery and address resolution.
-   RFCOMM connection establishment.
-   Reading from and writing to the Bluetooth stream.
-   JSON parsing and output.

## Limitations

-   Currently, the utility is designed for Linux systems with the `bluez` Bluetooth stack.
-   It relies on specific constants (`EAR_ADDRESS`, `EAR_CHANNEL`, `EAR_BATTERY`) that might be subject to change with firmware updates or different Nothing Ear models.

## Contributing

Contributions are welcome! Please feel free to open issues or submit pull requests for bug fixes, improvements, or new features.
