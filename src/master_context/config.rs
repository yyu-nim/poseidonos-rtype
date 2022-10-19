use serde::Deserialize;

#[derive(Deserialize)]
pub struct Config {
    pub transport: Transport,
    pub journal: Journal,
}
impl Config {
    pub fn new() -> Self {
        Self {
            transport: Transport::new(),
            journal: Journal::new(),
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

#[derive(Deserialize, Clone)]
pub struct Journal {
    pub enable: bool,
    pub buffer_size_in_mb: u64,
    pub number_of_log_groups: i32,
    pub debug_mode: bool,
    pub interval_in_msec_for_metric: u64,
    pub enable_vsc: bool,
}
impl Journal {
    pub fn new() -> Self {
        Self {
            enable: true,
            buffer_size_in_mb: 0,
            number_of_log_groups: 2,
            debug_mode: false,
            interval_in_msec_for_metric: 1000,
            enable_vsc: true,
        }
    }
}