use std::os::raw::c_int;
use lazy_static::lazy_static;
use crate::generated::bindings::spdk_pci_device;
use crate::spdk_wrapper::spdk::spdk_clib;
lazy_static!{
    pub static ref SpdkEnvCallerSingleton : SpdkEnvCaller = {
        SpdkEnvCaller
    };
}

pub struct SpdkEnvCaller;

impl SpdkEnvCaller {

    pub fn SpdkPciDeviceGetSocketId(&self, dev: *mut spdk_pci_device) -> i32 {
        spdk_clib::spdk_pci_device_get_socket_id(dev)
    }

    pub fn SpdkEnvGetCoreCount(&self) -> u32 {
        spdk_clib::spdk_env_get_core_count()
    }

    pub fn SpdkGetTicksHz(&self) -> u64 {
        spdk_clib::spdk_get_ticks_hz()
    }

    pub fn SpdkGetTicks(&self) -> u64 {
        spdk_clib::spdk_get_ticks()
    }

    pub fn SpdkEnvGetFirstCore(&self) -> u32 {
        spdk_clib::spdk_env_get_first_core()
    }

    pub fn SpdkEnvGetLastCore(&self) -> u32 {
        spdk_clib::spdk_env_get_last_core()
    }

    pub fn SpdkEnvGetCurrentCore(&self) -> u32 {
        spdk_clib::spdk_env_get_current_core()
    }

    pub fn SpdkEnvGetNextCore(&self, core: u32) -> u32 {
        spdk_clib::spdk_env_get_next_core(core)
    }
}