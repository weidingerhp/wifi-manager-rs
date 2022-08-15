use alloc::boxed::Box;
use alloc::sync::Arc;
use embedded_svc::httpd::Response;
use embedded_svc::httpd::registry::Registry;
use embedded_svc::storage::*;

extern crate alloc;
use alloc::vec::Vec;
use esp_idf_svc::httpd::ServerRegistry;
use log::info;
use esp_idf_sys::EspError;
use esp_idf_svc::{wifi::EspWifi, netif::*, sysloop::*, nvs::*};
use embedded_svc::wifi::{Wifi, Configuration, AccessPointConfiguration};
use embedded_svc::ipv4::{Mask, Ipv4Addr, Subnet, RouterConfiguration};
use spin::RwLock;

use crate::esp32::Esp32WifiManager;
use crate::WifiManager;

use crate::options::{WifiOptions, OptionSerializer};
use crate::serialization::WifiManagerInternalFuncs;

struct WifiSettingsStore {
    wifi_settings: WifiOptions,
    finished: bool,
}

impl WifiManagerInternalFuncs<EspError> for Esp32WifiManager {
    fn start(&mut self) -> Result<(), EspError> {
        info!("Esp32WifiManager::start - initializing options");
        let options = match self.read() {
            Ok(options) => options,
            Err(err) => return Err(err),
        };
        
        info!("Esp32WifiManager::start - running startup loop");
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
        if self.storage.contains(name).expect("write_raw_data_to_storage - could not open storage") {
            info!("Removing old item of {}", name);
            self.storage.remove(name).expect("write_raw_data_to_storage - storage remove failed");
        }

        match data {
            Some(content) => self.storage.put_raw(name, content.as_slice()),
            None => Ok(true)
        }
    }

    fn remove(&mut self, name: &str) -> Result<bool, EspError> {
        self.storage.remove(name)?;
        Ok(true)
        
    }

    fn run_server_store_data(&mut self) -> bool {
        let ap_list = self.wifi.scan().expect("Scan failed");

        self.wifi.set_configuration(&Configuration::AccessPoint(AccessPointConfiguration {
            ssid: "rust-ap".into(),
            ssid_hidden: false,
            channel: 2,
            ip_conf: Option::Some(RouterConfiguration {
                subnet: Subnet {
                    gateway: Ipv4Addr::new(192, 168, 8, 1),
                    mask: Mask(24),
                }, 
                dhcp_enabled: true,
                ..Default::default()
            }),
            ..Default::default()
        })).expect("setting ip config");

        let opts = Arc::new(RwLock::new(WifiSettingsStore {
            wifi_settings: WifiOptions {
                ssid: "".into(),
                password: "".into(),
                encrypted: false,},
            finished: false,
        }));

        let servers = ServerRegistry::new();
        {
            let setopts = Arc::clone(&opts);
            let servers = servers.at("/").get(move |req| {
                let mut resp = Response::new(200);
                resp.body = embedded_svc::httpd::Body::from("Hello World");
                setopts.write().finished = true;
                Ok(resp)
            }).expect("Registering main route");
            let config = esp_idf_svc::httpd::Configuration::default();
            let servers = servers.start(&config).expect("Starting server");

            loop {
                if opts.read().finished {
                    break;
                }
            }
        }
        
        
        true
    }
 
    fn try_connect_wifi(&mut self, _opts: &WifiOptions) -> bool {
        true
    }
       
}

fn my_handler(req: &embedded_svc::httpd::Request) -> Response {
    let mut res = embedded_svc::httpd::Response::new(200);
    res.body = embedded_svc::httpd::Body::from("Hello World");
    res
}

impl WifiManager for Esp32WifiManager {
    type Err = EspError;
    type WifiManagerType = Esp32WifiManager;
    type WifiType = EspWifi;

    fn start(&mut self) -> Result<(), EspError> {
        <Esp32WifiManager as WifiManagerInternalFuncs<EspError> >::start(self)
    }

    fn get_wifi(&self) -> Result<&EspWifi, EspError> {
        Ok(&self.wifi)
    }
}
