#[derive(Debug)]
pub enum DeviceType {
    LeftEar,
    RightEar,
    Case,
    Unknown,
}

impl From<u8> for DeviceType {
    fn from(value: u8) -> Self {
        match value {
            2 => DeviceType::LeftEar,
            3 => DeviceType::RightEar,
            4 => DeviceType::Case,
            _ => DeviceType::Unknown,
        }
    }
}
