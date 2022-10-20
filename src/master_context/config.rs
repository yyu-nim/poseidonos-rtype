use serde::Deserialize;

#[derive(Deserialize, Default)]
pub struct Config {
    pub transport: Transport,
    pub journal: Journal,
}

#[derive(Deserialize, Default)]
pub struct Transport {
    pub trType: String,
    pub bufCacheSize: u32,
    pub numSharedBuf: u32,
    pub ioUnitSize: u32,
}

#[derive(Deserialize, Clone, Default)]
pub struct Journal {
    pub enable: bool,
    pub buffer_size_in_mb: u64,
    pub number_of_log_groups: i32,
    pub debug_mode: bool,
    pub interval_in_msec_for_metric: u64,
    pub enable_vsc: bool,
}