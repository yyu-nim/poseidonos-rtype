#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

use std::borrow::{Borrow, BorrowMut};
use crate::device::device_manager::DeviceManagerSingleton;
use crate::event_scheduler::event_scheduler::EventSchedulerSingleton;
use crate::include::pos_event_id::PosEventId;
use crate::io::frontend_io::unvmf_io_handler::UNVMfCompleteHandler;
use crate::io_scheduler::io_dispatcher::{IODispatcher, IODispatcherSingleton};
use crate::master_context::config_manager::ConfigManagerSingleton;
use crate::metafs::metafs_service::MetaFsServiceSingleton;
use crate::network::transport_configuration::TransportConfiguration;
use crate::qos::qos_manager::QosManagerSingleton;
use crate::spdk_wrapper;
use crate::spdk_wrapper::caller::spdk_caller::SpdkCallerSingleton;

pub struct Poseidonos;

impl Poseidonos {
    pub fn Init(&self, _args: Vec<&str>) -> PosEventId {
        if let Ok(conf) = self._LoadConfiguration() {
            self._InitSignalHandler(&conf);
            self._LoadVersion(&conf);
            self._InitSpdk(&conf);
            self._InitAffinity(&conf);
            self._SetupThreadModel(&conf);
            self._SetPerfImpact(&conf);
            self._InitDebugInfo(&conf);
            self._InitAIR(&conf);
            self._InitIOInterface(&conf);
            self._InitMemoryChecker(&conf);
            self._InitResourceChecker(&conf);
            self._InitReplicatorManager(&conf);
            self._InitTraceExporter(&conf);
            return PosEventId::SUCCESS;
        } else {
            return PosEventId::POS_TRACE_INIT_FAIL;
        }
    }

    pub fn Run(&self) {
        // TODO
    }

    pub fn Terminate(&self) {
        // TODO
    }

    fn _LoadConfiguration(&self) -> Result<PosConfiguration, PosEventId> {
        // TODO
        Ok(PosConfiguration)
    }
    fn _InitSignalHandler(&self, _conf: &PosConfiguration) {
        // TODO
    }
    fn _LoadVersion(&self, _conf: &PosConfiguration) {
        // TODO
    }
    fn _InitSpdk(&self, _conf: &PosConfiguration) {
        let mut spdk = spdk_wrapper::spdk::Spdk::new();
        spdk.Init(Vec::new());

        let transportConfig = TransportConfiguration::new(&ConfigManagerSingleton);
        transportConfig.CreateTransport();
    }
    fn _InitAffinity(&self, _conf: &PosConfiguration) {
        // TODO
    }
    fn _SetupThreadModel(&self, _conf: &PosConfiguration) {
        // TODO: AffinityManager isn't being introduced yet
        SpdkCallerSingleton.SpdkBdevPosRegisterPoller(Some(UNVMfCompleteHandler));
        QosManagerSingleton.InitializeSpdkManager();
        QosManagerSingleton.Initialize();

        EventSchedulerSingleton.Initialize(8 /* TODO */, Vec::new(), Vec::new());
        DeviceManagerSingleton.Initialize();
        MetaFsServiceSingleton.Initialize(0 /* TODO */, Vec::new(), Vec::new());
    }
    fn _SetPerfImpact(&self, _conf: &PosConfiguration) {
        // TODO
    }
    fn _InitDebugInfo(&self, _conf: &PosConfiguration) {
        // TODO
    }
    fn _InitAIR(&self, _conf: &PosConfiguration) {
        // TODO
    }
    fn _InitIOInterface(&self, _conf: &PosConfiguration) {
        // TODO
    }
    fn _InitMemoryChecker(&self, _conf: &PosConfiguration) {
        // TODO
    }
    fn _InitResourceChecker(&self, _conf: &PosConfiguration) {
        // TODO
    }
    fn _InitReplicatorManager(&self, _conf: &PosConfiguration) {
        // TODO
    }
    fn _InitTraceExporter(&self, _conf: &PosConfiguration) {
        // TODO
    }
}

pub struct PosConfiguration;