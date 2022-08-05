#[cfg(test)]
pub mod tests {
    extern crate alloc;
    extern crate std;

    use alloc::string::String;
    use alloc::{string::ToString, vec::Vec};
    use log::info;
    use simplelog::*;
    use std::*;

    use crate::esp32::traits::WifiManagerInternalFuncs;
    use crate::WifiManager;
    use crate::options::{OptionSerializer, WifiOptions};
    struct DummyWifiManager {
        versiondata: Vec<u8>,
        rawdata: Vec<u8>,
    }

    impl WifiManager for DummyWifiManager {
        type Err = ();
        type WifiManagerType = DummyWifiManager;
        fn new() -> Result<Self::WifiManagerType, Self::Err> {
            Ok(DummyWifiManager {
                versiondata: Vec::new(),
                rawdata: Vec::new(),
            })
        }
        fn start(&mut self) -> Result<(), Self::Err> {
            Ok(())
        }
    }

    impl WifiManagerInternalFuncs<String> for DummyWifiManager {
        fn start(&mut self) -> Result<(), String> {
            todo!()
        }

        fn read_raw_data_from_storage(&mut self, name: &str) -> Option<alloc::vec::Vec<u8>> {
            info!("Reading Data {}", name);
            match name {
                "version" => Some(self.versiondata.clone()),
                "data" => Some(self.rawdata.clone()),
                _ => {
                    info!("Invalid segment name");
                    None
                }
            }
        }

        fn write_raw_data_to_storage(
            &mut self,
            name: &str,
            data: Option<alloc::vec::Vec<u8>>,
        ) -> Result<bool, String> {
            info!("Writing Data {}", name);
            match data {
                Some(bytes) => match name {
                    "version" => bytes.into_iter().for_each(|x| self.versiondata.push(x)),
                    "data" => bytes.into_iter().for_each(|x| self.rawdata.push(x)),
                    _ => info!("Invalid segment name"),
                },
                None => info!("No data provided"),
            }
            Ok(true)
        }

        fn remove(&mut self, name: &str) -> Result<bool, String> {
            info!("Removing Data {}", name);
            match name {
                "version" => self.versiondata.clone().clear(),
                "data" => self.rawdata.clone().clear(),
                _ => {
                    info!("Invalid segment name");
                    return Err("Invalid segment".to_string());
                }
            };

            Ok(true)
        }

        fn run_server_store_data(&mut self) -> bool {
            todo!()
        }

        fn try_connect_wifi(&mut self, opts: &crate::options::WifiOptions) -> bool {
            todo!()
        }
    }

    #[test]
    pub fn test_binary_serializer() {
        TermLogger::init(
            LevelFilter::Error,
            Config::default(),
            TerminalMode::Stdout,
            ColorChoice::Auto,
        )
        .unwrap();

        let mut testMgr = DummyWifiManager {
            versiondata: Vec::new(),
            rawdata: Vec::new(),
        };

        let opts = WifiOptions {
            ssid: "ssid".to_string(),
            encrypted: false,
            password: "pass".to_string(),
        };

        OptionSerializer::write(&mut testMgr, &opts).unwrap();

        let newopts_opt_res = OptionSerializer::read(&mut testMgr);
        
        let newopts = newopts_opt_res.unwrap().unwrap();

        assert_eq!(opts.ssid, newopts.ssid, "Testing SSID");
        assert_eq!(opts.encrypted, newopts.encrypted, "Testing Encrypted");
        assert_eq!(opts.password, newopts.password, "Testing Password");

        info!("Version: {:x?}", testMgr.versiondata);
        info!("Raw: {:x?}", testMgr.rawdata);
    }
}
