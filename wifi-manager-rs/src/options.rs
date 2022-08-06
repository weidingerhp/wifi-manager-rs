extern crate alloc;
use alloc::{string::String, vec::Vec};
use log::info;
use serde::{Serialize, Deserialize};

use crate::serialization::WifiManagerInternalFuncs;

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


impl<T:WifiManagerInternalFuncs<Err>, Err:Sized> OptionSerializer<Err> for T {
    fn write(&mut self, data: &WifiOptions) -> Result<bool, Err> {

        let version: Vec<u8> = [(WIFI_OPTS_VERSION >> 8) as u8, WIFI_OPTS_VERSION as u8].into();
        self.write_raw_data_to_storage("version", Some(version)).unwrap_or(false);

        let mut vec: Vec<u8> = Vec::new();
        let options_data: Vec<u8> = match ciborium::ser::into_writer(data, &mut vec) {
            Ok(_) => vec,
            Err(err) => {
                info!("Deserialize failed : {}", err);
                return Ok(false);
            },
        };

        self.write_raw_data_to_storage("data", Some(options_data))
    }

    fn read(&mut self) -> Result<Option<WifiOptions>, Err> {

        let data = match self.read_raw_data_from_storage("version") {
            Some(val) => val,
            None => return Ok(Option::None),
        };

        if data.len() != 2 {
            self.remove("version").unwrap_or(false);
            info!("Version data length invalid - was {} instead of 2", data.len());
            return Ok(Option::None)
        }

        let version:u16 = ((data[0] as u16) << 8) + (data[1] as u16);
        if version != WIFI_OPTS_VERSION {
            info!("Version information invalid - was {} instead of {}", version, WIFI_OPTS_VERSION);
            self.remove("version").unwrap_or(false);
            return Ok(Option::None);
        }
        
        let data = match self.read_raw_data_from_storage("data") {
            Some(val) => val,
            None => return Ok(Option::None),
        };

        let options: WifiOptions = match ciborium::de::from_reader(data.as_slice()) {
            Ok(val) => val,
            Err(_) => return Ok(None) ,
        };

        Ok(Some(options))


    }
}
