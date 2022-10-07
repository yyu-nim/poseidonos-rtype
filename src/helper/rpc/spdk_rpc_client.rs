use log::info;
use crate::spdk_wrapper::caller::spdk_env_caller::SpdkEnvCaller;

const defaultSpdkSocketPath: &str = "/var/tmp/spdk.sock";

pub struct SpdkRpcClient {
    core_count: u32,
}

impl SpdkRpcClient {

    pub fn new(spdkEnvCaller: &SpdkEnvCaller) -> SpdkRpcClient {
        SpdkRpcClient {
            core_count: spdkEnvCaller.SpdkEnvGetCoreCount(),
        }
    }

    pub fn TransportCreate(&self, trType: String, bufCacheSize: u32,
                           numSharedBuf: u32, ioUnitSize: u32) {
        info!("SpdkRpcClient is about to create a transport {} {} {} {}", trType, bufCacheSize, numSharedBuf, ioUnitSize);

        let trType = trType.to_lowercase();
        let coreCount = self.core_count.clone();

        // TODO: send json message to domain socket on defaultSpdkSocketPath
        info!("TODO: send json message to domain socket on {}", defaultSpdkSocketPath);
    }
}