#![no_std]

#[cfg(any(feature = "esp32", test))]
pub mod esp32;
mod options;

pub trait WifiManager {
    type Err;
    type WifiManagerType;

    fn new() -> Result<Self::WifiManagerType, Self::Err>;
    fn start(&mut self) -> Result<(), Self::Err>;

}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
