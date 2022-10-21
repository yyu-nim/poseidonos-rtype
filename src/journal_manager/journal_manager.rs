use log::info;
use crate::array_models::interface::i_array_info::ArrayInfo;

use crate::journal_manager::config::journal_configuration::JournalConfiguration;

pub struct JournalManager {
    config: JournalConfiguration,
}

impl JournalManager {
    pub fn new(arrayInfo: ArrayInfo) -> JournalManager {
        info!("Creating JournalManager for {}", arrayInfo.name);
        JournalManager {
            config: JournalConfiguration::new(),
        }
    }
}