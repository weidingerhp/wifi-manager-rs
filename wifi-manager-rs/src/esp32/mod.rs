extern crate alloc;
use alloc::boxed::Box;
use alloc::sync::Arc;
#[cfg(target_arch="xtensa")]
use esp_idf_svc::{nvs::{EspDefaultNvs, EspNvs}, wifi::EspWifi, netif::{EspNetifStack}, sysloop::EspSysLoopStack};
#[cfg(target_arch="xtensa")]
use esp_idf_svc::nvs_storage::{EspNvsStorage};
#[cfg(target_arch="xtensa")]
use esp_idf_sys::EspError;

pub mod traits; 
mod impls;
#[cfg(test)] 
pub mod test;

#[cfg(target_arch="xtensa")]
pub struct Esp32WifiManager {
    storage: EspNvsStorage,
    #[allow(dead_code)]
    wifi: Box<EspWifi>,
}

#[cfg(target_arch="xtensa")]
impl Esp32WifiManager {
    fn new() -> Result<Esp32WifiManager, EspError> {
        let nvs_storage = Arc::new(EspNvs::new("wifi-manager").unwrap());
        let storage = EspNvsStorage::new(nvs_storage, "data", true).unwrap();
        let nvs_default = Arc::new(EspDefaultNvs::new().unwrap());
        let netif = Arc::new(EspNetifStack::new().unwrap());
        let sysloop = Arc::new(EspSysLoopStack::new().unwrap());
        let wifi = Box::new(EspWifi::new(netif.clone(), sysloop.clone(), nvs_default.clone()).unwrap());
        Ok(Esp32WifiManager {
            storage: storage,
            wifi: wifi
        })
    }
}