#![no_std]

#[cfg(any(feature = "esp32", test))]
#[cfg(target_arch="xtensa")]
pub mod esp32;
mod serialization;
mod options;

pub trait WifiManager {
    type Err;
    type WifiManagerType;
    type WifiType;

    fn start(&mut self) -> Result<(), Self::Err>;
    fn get_wifi(&self) -> Result<&Self::WifiType, Self::Err>;

}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
