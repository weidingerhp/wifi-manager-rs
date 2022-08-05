
#[cfg(target_arch="xtensa")]
use embedded_svc::storage::Storage;

#[cfg(target_arch="xtensa")]
use embedded_svc::storage::StorageBase;

extern crate alloc;
use alloc::vec::Vec;
use log::info;
#[cfg(target_arch="xtensa")]
use esp_idf_sys::EspError;


#[cfg(target_arch="xtensa")]
use crate::esp32::Esp32WifiManager;
#[cfg(target_arch="xtensa")]
use crate::WifiManager;

use crate::options::{WifiOptions, OptionSerializer, WIFI_OPTS_VERSION};
use crate::esp32::traits::*;

#[cfg(target_arch="xtensa")]
impl WifiManagerInternalFuncs<EspError> for Esp32WifiManager {
    fn start(&mut self) -> Result<(), EspError> {
        let options = match self.read() {
            Ok(options) => options,
            Err(err) => return Err(err),
        };
        
        loop {
            if match &options {
                None => self.run_server_store_data(),
                Some(opts) => self.try_connect_wifi(&opts), 
            } {
                break;
            }
    
        }
        Ok(())
    }

    fn remove(&mut self, name: &str) -> Result<bool, EspError> {
        self.storage.remove(name)
        
    }

    fn read_raw_data_from_storage(&mut self, name: &str) -> Option<Vec<u8> > {
        match self.storage.get_raw(name) {
            Ok(val) => val,
            Err(err) => {
                info!("Key {} read returned {}", name, err);
                Option::None
            },
        }
    }

    fn write_raw_data_to_storage(&mut self, name: &str, data: Option<Vec<u8> >) -> Result<bool, EspError> {
        if self.storage.contains(name).unwrap() {
            info!("Removing old item of {}", name);
            self.storage.remove(name).unwrap();
        }

        match data {
            Some(content) => self.storage.put_raw(name, content),
            None => Ok(true)
        }
    }

    fn run_server_store_data(&mut self) -> bool {
        true
    }
 
    fn try_connect_wifi(&mut self, _opts: &WifiOptions) -> bool {
        true
    }
       
}

#[cfg(target_arch="xtensa")]
impl WifiManager for Esp32WifiManager {
    type Err = EspError;
    type WifiManagerType = Esp32WifiManager;

    fn new() -> Result<Esp32WifiManager, EspError> {
        Esp32WifiManager::new()
    }

    fn start(&mut self) -> Result<(), EspError> {
        <Esp32WifiManager as WifiManagerInternalFuncs<EspError> >::start(self)
    }
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
