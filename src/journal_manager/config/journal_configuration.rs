use crate::master_context::config_manager::{ConfigManager, ConfigManagerSingleton};
use crate::master_context::config::Journal;

pub struct JournalConfiguration {
    config: Journal,
}

impl JournalConfiguration {
    pub fn new() -> Self {
        Self {
            config: ConfigManagerSingleton.journalConfig(),
        }
    }

    pub fn IsEnabled(&self) -> bool { self.config.enable }
    pub fn IsDebugEnabled(&self) -> bool { self.config.debug_mode }
    pub fn IsVscEnabled(&self) -> bool { self.config.enable_vsc }
    pub fn GetIntervalForMetric(&self) -> u64 { self.config.interval_in_msec_for_metric }
    pub fn GetNumLogGroups(&self) -> i32 { self.config.number_of_log_groups }
    pub fn GetLogBufferSize(&self) -> u64 { todo!(); }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_config_read() {
        let config = JournalConfiguration::new();

        assert_eq!(config.IsEnabled(), true);
        assert_eq!(config.IsDebugEnabled(), false);
        assert_eq!(config.IsVscEnabled(), true);
        assert_eq!(config.GetIntervalForMetric(), 1000);
        assert_eq!(config.GetNumLogGroups(), 2);
    }
}