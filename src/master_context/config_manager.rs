use lazy_static::lazy_static;
lazy_static!{
    pub static ref ConfigManagerSingleton : ConfigManager = {
        ConfigManager::new()
    };
}

pub struct ConfigManager {
    /***
    TransportConfiguration
     */
    trType: String,
    bufCacheSize: u32,
    numSharedBuf: u32,
    ioUnitSize: u32,

}

impl Default for ConfigManager {
    fn default() -> Self {
        ConfigManager {
            trType: "default-tr".to_string(),
            bufCacheSize: 0,
            numSharedBuf: 0,
            ioUnitSize: 0
        }
    }
}

impl ConfigManager {

    pub fn new() -> ConfigManager {
        let mut c = ConfigManager::default();
        c.ReadFile();
        c
    }

    pub fn ReadFile(&mut self) {
        // Fake
        self.trType = "TCP".to_string();
        self.bufCacheSize = 64;
        self.numSharedBuf = 4096;
        self.ioUnitSize = 512;
    }

    pub fn trType(&self) -> String {
        self.trType.clone()
    }

    pub fn bufCacheSize(&self) -> u32 {
        self.bufCacheSize.clone()
    }

    pub fn numSharedBuf(&self) -> u32 {
        self.numSharedBuf.clone()
    }

    pub fn ioUnitSize(&self) -> u32 {
        self.ioUnitSize.clone()
    }
}