use lazy_static::lazy_static;
use crate::metafs::config::metafs_config_manager::MetaFsConfigManager;

lazy_static!{
    pub static ref MetaFsServiceSingleton: MetaFsService = {
        MetaFsService::new()
    };
}

pub struct MetaFsService {
    configManager: MetaFsConfigManager,
}

impl MetaFsService {

    pub fn new() -> MetaFsService {
        MetaFsService {
            configManager: MetaFsConfigManager::new(),
        }
    }

    pub fn Initialize(&self, totalCoreCount: u32, schedSet: Vec<u8>, mioSet: Vec<u8>) {
        self.configManager.Init();
        self._CreateScheduler(totalCoreCount, schedSet, mioSet);
    }

    fn _CreateScheduler(&self, _totalCoreCount: u32, _schedSet: Vec<u8>, _mioSet: Vec<u8>) {
        // TODO
    }
}