
#[cfg(target_arch="xtensa")]
use embedded_svc::storage::*;

extern crate alloc;
use alloc::vec::Vec;
use log::info;
use esp_idf_sys::EspError;

use crate::esp32::Esp32WifiManager;
use crate::WifiManager;

use crate::options::{WifiOptions, OptionSerializer};
use crate::serialization::WifiManagerInternalFuncs;

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

    fn read_raw_data_from_storage(&mut self, name: &str) -> Option<Vec<u8>> {
        match self.storage.contains(name) {
            Ok(val) => {
                if val {
                    let mut buf = Vec::<u8>::new();
                    let len: usize = match self.storage.len(name) {
                        Ok(len) => match len {
                            Some(len) => len,
                            None => 0 as usize
                        },
                        Err(err) => {
                            info!("Key {} read returned {}", name, err);
                            0 as usize
                        },
                    };
                    
                    buf.reserve(len);

                    match self.storage.get_raw(name, &mut buf) {
                        Ok(_val) => Option::Some(buf), 
                        Err(err) => {
                            info!("Key {} read returned {}", name, err);
                            Option::None
                        },
                    }
                } else {
                    Option::None
                }
            },
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
            Some(content) => self.storage.put_raw(name, content.as_slice()),
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
