use std::borrow::{Borrow, BorrowMut};
use crate::helper::rpc::spdk_rpc_client::SpdkRpcClient;
use crate::master_context::config_manager::ConfigManager;
use crate::spdk_wrapper::caller::spdk_env_caller::{SpdkEnvCallerSingleton};

pub struct TransportConfiguration {
    trType: String,
    bufCacheSize: u32,
    numSharedBuf: u32,
    ioUnitSize: u32,
    rpcClient: SpdkRpcClient,
}

impl TransportConfiguration {
    pub fn new(configManager: &ConfigManager) -> TransportConfiguration {
        TransportConfiguration {
            trType: configManager.trType(),
            bufCacheSize: configManager.bufCacheSize(),
            numSharedBuf: configManager.numSharedBuf(),
            ioUnitSize: configManager.ioUnitSize(),
            rpcClient: SpdkRpcClient::new(&SpdkEnvCallerSingleton),
        }
    }

    pub fn CreateTransport(&self) {
        self.rpcClient.borrow().TransportCreate(self.trType.clone(),
                                       self.bufCacheSize.clone(),
                                       self.numSharedBuf.clone(),
                                       self.ioUnitSize.clone());
    }
}