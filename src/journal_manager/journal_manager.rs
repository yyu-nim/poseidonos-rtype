use log::info;

use crate::journal_manager::config::journal_configuration::JournalConfiguration;

pub struct JournalManager {
    config: JournalConfiguration,
}

impl JournalManager {
    pub fn new(arrayName: String, arrayIdx: u32) -> JournalManager {
        info!("Creating JournalManager for {}", arrayName);
        JournalManager {
            config: JournalConfiguration::new(),
        }
    }
}