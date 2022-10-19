use serde::Deserialize;

#[derive(Deserialize)]
pub struct Config {
    pub transport: Transport,
}
impl Config {
    pub fn new() -> Self {
        Self {
            transport: Transport::new(),
        }
    }
}

#[derive(Deserialize)]
pub struct Transport {
    pub trType: String,
    pub bufCacheSize: u32,
    pub numSharedBuf: u32,
    pub ioUnitSize: u32,
}
impl Transport {
    pub fn new() -> Self {
        Transport {
            trType: "default-tr".to_string(),
            bufCacheSize: 0,
            numSharedBuf: 0,
            ioUnitSize: 0,
        }
    }
}