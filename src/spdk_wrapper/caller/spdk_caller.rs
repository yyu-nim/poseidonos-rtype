use lazy_static::lazy_static;
use crate::spdk_wrapper::spdk::spdk_clib;

lazy_static!{
    pub static ref SpdkCallerSingleton : SpdkCaller = {
        SpdkCaller
    };
}

pub struct SpdkCaller;

impl SpdkCaller {
    pub fn new() -> SpdkCaller {
        SpdkCaller
    }

    pub fn SpdkBdevPosRegisterPoller(&self, func: ::std::option::Option<unsafe extern "C" fn()>) {
        spdk_clib::spdk_bdev_pos_register_poller(func);
    }

    pub fn SpdkBdevPosUnRegisterPoller(&self, func: ::std::option::Option<unsafe extern "C" fn()>) {
        spdk_clib::spdk_bdev_pos_unregister_poller(func);
    }
}