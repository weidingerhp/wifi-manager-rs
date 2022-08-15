extern crate alloc;
use core::borrow::BorrowMut;

use alloc::boxed::Box;
use alloc::sync::Arc;
use esp_idf_svc::{nvs::{EspDefaultNvs, EspNvs}, wifi::EspWifi, netif::{EspNetifStack}, sysloop::EspSysLoopStack};
use esp_idf_svc::nvs_storage::{EspNvsStorage};
use esp_idf_sys::{EspError};
use log::info;

mod impls;

pub struct Esp32WifiManager {
    storage: EspNvsStorage,
    #[allow(dead_code)]
    wifi: Box<EspWifi>,
}

impl Esp32WifiManager {
    pub fn new(netif: Arc<EspNetifStack>, sysloop: Arc<EspSysLoopStack>, default_nvs: Arc<EspDefaultNvs>) -> Result<Esp32WifiManager, EspError> {
        info!("Creating EspNvsStorage");
        let storage = EspNvsStorage::new_default(default_nvs.clone(), "wmgrdta", true)?;
        info!("creating EspWifi");
        let wifi = Box::new(EspWifi::new(netif.clone(), sysloop.clone(), default_nvs.clone())?);
        info!("Finished - returning new Esp32WifiManager");
        Ok(Esp32WifiManager {
            storage: storage,
            wifi: wifi
        })
    }

    fn drop(&mut self) {
        info!("Dropping Esp32WifiManager");
        info!("dropping wifi");
        drop(self.wifi);
        info!("dropping storage");
        drop(self.storage);
    }
}