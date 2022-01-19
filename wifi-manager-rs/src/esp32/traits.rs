extern crate alloc;
use alloc::vec::Vec;

use crate::{options::WifiOptions, WifiManager};


pub trait WifiManagerInternalFuncs<Err> : WifiManager {
    fn start(&mut self) -> Result<(), Err>;

    fn read_raw_data_from_storage(&mut self, name: &str) -> Option<Vec<u8> >;
    fn write_raw_data_to_storage(&mut self, name: &str, data: Option<Vec<u8> >) -> Result<bool, Err> ;
    fn remove(&mut self, name: &str) -> Result<bool, Err>;
    fn run_server_store_data(&mut self) -> bool;
    fn try_connect_wifi(&mut self, opts: &WifiOptions) -> bool;
}