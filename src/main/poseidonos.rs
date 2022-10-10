#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

use std::borrow::{Borrow, BorrowMut};
use std::sync::{Arc, Mutex};
use crate::device::device_manager::DeviceManagerSingleton;
use crate::event_scheduler::event_scheduler::EventSchedulerSingleton;
use crate::event_scheduler::io_completer;
use crate::event_scheduler::io_completer::IoCompleter;
use crate::event_scheduler::io_timeout_checker::IoTimeoutCheckerSingleton;
use crate::include::pos_event_id::PosEventId;
use crate::io::frontend_io::flush_command_manager::FlushCmdManagerSingleton;
use crate::io::frontend_io::unvmf_io_handler::UNVMfCompleteHandler;
use crate::io::general_io::io_recovery_event_factory::IoRecoveryEventFactory;
use crate::io_scheduler::io_dispatcher;
use crate::io_scheduler::io_dispatcher::{IODispatcher, IODispatcherSingleton};
use crate::io_submit_interface::i_io_submit_handler;
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
        SpdkCallerSingleton.SpdkBdevPosRegisterPoller(Some(UNVMfCompleteHandler));
        QosManagerSingleton.InitializeSpdkManager();
        QosManagerSingleton.Initialize();

        EventSchedulerSingleton.Initialize(8 /* TODO */, Vec::new(), Vec::new());
        DeviceManagerSingleton.Initialize();
        MetaFsServiceSingleton.Initialize(0 /* TODO */, Vec::new(), Vec::new());
        FlushCmdManagerSingleton.borrow(); // do nothing but instantiate
        IoTimeoutCheckerSingleton.Initialize();
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
        i_io_submit_handler::instance.borrow(); // just to instantiate

        // Note: IoRecoveryFactory는 내부 state를 가지고 있지 않기 때문에,
        // 굳이 arc::mutex로 감싸지 않고 객체를 새로 만들어서
        // IoCompleter, IODispatcher에 넘겨주어도 된다고 생각함.
        // 이러한 가정이 어긋나는 상황이 발견되면, 다른 객체들간 공유될 수 있는
        // 형태로 리팩토링 해야 할 것. 현재는, lazy_static에서는 의존성 주입이
        // 단순치 않아, IoCompleter/IODispatcher 에 singleton 객체로
        // factory를 들고 있도록 하여, 해당 .rs 내에서 사용하도록 하고,
        // 건네주는 인자는 무시하도록 함.
        let factory = IoRecoveryEventFactory::new();
        io_completer::RegisterRecoveryEventFactory(factory);

        let factory = IoRecoveryEventFactory::new();
        io_dispatcher::RegisterRecoveryEventFactory(factory);
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