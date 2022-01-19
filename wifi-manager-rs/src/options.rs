extern crate alloc;
use alloc::string::String;
use serde::{Serialize, Deserialize};

// Cargo complains about this being dead code - but it isn't
#[allow(dead_code)]
pub const WIFI_OPTS_VERSION: u16 = 1;

#[derive(Debug)]
#[derive(Serialize, Deserialize)]
pub struct WifiOptions {
    pub ssid: String,
    pub encrypted: bool,
    pub password: String,
}

pub trait OptionSerializer<Err> {
    fn write(&mut self, data: &WifiOptions) -> Result<bool, Err>;
    fn read(&mut self) -> Result<Option<WifiOptions>, Err>;
}