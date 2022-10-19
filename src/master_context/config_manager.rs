use std::fs;
use lazy_static::lazy_static;
use log::info;
use crate::master_context::config::Config;

const DEFAULT_CONFIG_FILE: &str = "config/pos.toml";

lazy_static!{
    pub static ref ConfigManagerSingleton : ConfigManager = {
        ConfigManager::new(DEFAULT_CONFIG_FILE)
    };
}

pub struct ConfigManager {
    config: Config,
}

impl Default for ConfigManager {
    fn default() -> Self {
        ConfigManager {
            config: Config::new(),
        }
    }
}

impl ConfigManager {

    pub fn new(file_path: &str) -> ConfigManager {
        let mut c = ConfigManager::default();
        c.ReadFile(file_path);
        c
    }

    pub fn ReadFile(&mut self, file_path: &str) {
        match fs::read_to_string(file_path) {
            Ok(config_string) => {
                self.config = toml::from_str(&config_string).expect("Failed to parse toml");
            },
            Err(e) => {
                info!("Failed to read config file {file_path}, use default");
            }
        };
    }

    pub fn trType(&self) -> String {
        self.config.transport.trType.clone()
    }

    pub fn bufCacheSize(&self) -> u32 {
        self.config.transport.bufCacheSize.clone()
    }

    pub fn numSharedBuf(&self) -> u32 {
        self.config.transport.numSharedBuf.clone()
    }

    pub fn ioUnitSize(&self) -> u32 {
        self.config.transport.ioUnitSize.clone()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_config() {
        let mut config_manager = ConfigManager::new("invalid.file");

        assert_eq!(config_manager.trType(), "default-tr".to_string());
        assert_eq!(config_manager.bufCacheSize(), 0);
        assert_eq!(config_manager.numSharedBuf(), 0);
        assert_eq!(config_manager.ioUnitSize(), 0);
    }

    #[test]
    fn test_read_default_file() {
        let mut config_manager = ConfigManager::new(DEFAULT_CONFIG_FILE);

        assert_eq!(config_manager.trType(), "TCP".to_string());
        assert_eq!(config_manager.bufCacheSize(), 64);
        assert_eq!(config_manager.numSharedBuf(), 4096);
        assert_eq!(config_manager.ioUnitSize(), 512);
    }
}