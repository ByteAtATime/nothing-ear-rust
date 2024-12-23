mod device;

use bluer::rfcomm::Stream;
use bluer::{Address, Session};
use serde::{Deserialize, Serialize};
use serde_json::json;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::time::sleep;
use crate::device::DeviceType;

#[derive(Serialize, Deserialize, Debug)]
struct DeviceInfo {
    device_type: String,
    recharging: bool,
    battery_level: u8,
}

const EAR_ADDRESS: [u8; 3] = [0x2C, 0xBE, 0xEB];
const EAR_CHANNEL: u8 = 15;
const EAR_BATTERY: [u8; 10] = [0x55, 0x60, 0x01, 0x07, 0xc0, 0x00, 0x00, 0x01, 0xac, 0xdf];
const RETRY: u64 = 3;
const BATTERY_STATUS_2: u16 = 16391;
const RECHARGING_MASK: u8 = 0x80;
const BATTERY_MASK: u8 = 0x7F;

async fn set_powered_adapter(session: Session) -> Result<bluer::Adapter, Box<dyn std::error::Error>> {
    let adapter = session.default_adapter().await?;
    adapter.set_powered(true).await?;
    Ok(adapter)
}

async fn find_address(adapter: bluer::Adapter, address: [u8; 3]) -> Result<Address, Box<dyn std::error::Error>> {
    let device_addresses = adapter.device_addresses().await?;
    let ear_address = device_addresses.iter().find(|&addr| match addr.0 {
        [a, b, c, _, _, _] => a == address[0] && b == address[1] && c == address[2],
    }).ok_or_else(|| "Couldn't find any Ear devices connected. Make sure you're paired with your Ear.")?;
    Ok(*ear_address)
}

pub async fn connect(address: [u8; 3], channel: u8) -> Result<Stream, Box<dyn std::error::Error>> {
    let session = Session::new().await?;
    let adapter = set_powered_adapter(session).await?;
    let ear_address = find_address(adapter, address).await?;
    let stream = Stream::connect(bluer::rfcomm::SocketAddr { addr: ear_address, channel }).await?;
    Ok(stream)
}

async fn fetch_stream() -> Stream {
    let mut stream = connect(EAR_ADDRESS, EAR_CHANNEL).await;
    for i in 1..=RETRY {
        sleep(std::time::Duration::from_millis(i * 500)).await;
        match stream {
            Ok(s) => {
                stream = Ok(s);
                break;
            }
            Err(e) => {
                eprintln!("Error: {}", e);
                stream = connect(EAR_ADDRESS, EAR_CHANNEL).await;
            }
        }
    }
    stream.expect("Failed to connect to Ear")
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut stream = fetch_stream().await;
    stream.write_all(&EAR_BATTERY).await?;
    let mut buf = [0_u8; 17]; // output may be either 17 bytes (earbuds + case), 15 (earbuds only) or 13 (only 1 bud)
    stream.read(&mut buf).await?;
    let command = u16::from_le_bytes(buf[3..5].try_into().unwrap());

    if command == BATTERY_STATUS_2 {
        let connected_device_count = buf[8] as usize;
        let mut devices: Vec<DeviceInfo> = Vec::new();

        for i in 0..connected_device_count {
            let device_type = DeviceType::from(buf[9 + i * 2]);
            let battery_level = buf[10 + i * 2] & BATTERY_MASK;
            let is_recharging = buf[10 + i * 2] & RECHARGING_MASK == RECHARGING_MASK;
            devices.push(DeviceInfo {
                device_type: format!("{:?}", device_type),
                recharging: is_recharging,
                battery_level,
            });
        }

        let json_output = json!(devices);
        println!("{}", serde_json::to_string_pretty(&json_output)?);

    } else {
        let json_output = json!({"error": "Ear 2 not detected"});
        println!("{}", serde_json::to_string(&json_output)?);
    }

    Ok(())
}