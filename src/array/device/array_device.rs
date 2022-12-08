use log::info;
use crate::device::base::ublock_device::UBlockDevice;
use crate::include::array_device_state::ArrayDeviceState;
use crate::include::i_array_device::IArrayDevice;
use crate::include::pos_event_id::PosEventId;

pub struct ArrayDevice {
    prevUblockInfo: String,
    uBlock: Box<dyn UBlockDevice>,
    state: ArrayDeviceState,
    dataIndex: u32,
}

impl ArrayDevice {
    pub fn new(uBlock: Box<dyn UBlockDevice>, state: ArrayDeviceState, dataIndex: u32) -> Self {
        Self {
            prevUblockInfo: String::new(),
            uBlock,
            state,
            dataIndex,
        }
    }

    pub fn GetUblock(&self) -> Box<dyn UBlockDevice> {
        return self.uBlock.clone_box()
    }

    pub fn SetUblock(&mut self, uBlock: Box<dyn UBlockDevice>) {
        self.prevUblockInfo = format!("{}({})", self.uBlock.GetName(), self.uBlock.GetSN());
        self.uBlock = uBlock;
    }

    pub fn GetName(&self) -> String {
        self.uBlock.GetName()
    }

    pub fn GetSerial(&self) -> String {
        self.uBlock.GetSN()
    }

    pub fn GetState(&self) -> ArrayDeviceState {
        self.state.clone()
    }

    pub fn SetState(&mut self, input: ArrayDeviceState) {
        let devName = self.uBlock.GetName();
        self.state = input;

        info!("[{}] Array device [ {} ]'s state is changed to {}",
            PosEventId::ARRAY_EVENT_DEV_STATE_CHANGED.to_string(),
            devName,
            self.state.to_string()
        );
    }
}

impl IArrayDevice for ArrayDevice {
    fn GetUblock(&self) -> Box<dyn UBlockDevice> {
        self.uBlock.clone_box()
    }

    fn SetUblock(&mut self, uBlock: Box<dyn UBlockDevice>) {
        self.uBlock = uBlock;
    }
}

#[cfg(test)]
mod tests {
    use std::sync::{Arc, Mutex};
    use crate::bio::ubio::Ubio;
    use crate::device::base::device_property::DeviceClass;
    use super::*;

    #[derive(Clone)]
    struct MockDevice {
        name: String,
        sn: String,
    }
    impl UBlockDevice for MockDevice {
        fn SubmitAsyncIO(&self, bio: Arc<Mutex<Ubio>>) -> i32 { 0 }
        fn CompleteIOs(&self) -> i32 { 0 }
        fn Close(&self) -> u32 { 0 }
        fn Open(&mut self) -> bool { true }
        fn clone_box(&self) -> Box<dyn UBlockDevice> { Box::new(self.clone()) }
        fn GetName(&self) -> String { self.name.clone() }
        fn GetSN(&self) -> String { self.sn.clone() }
        fn SetClass(&mut self, class: DeviceClass) {}
    }

    #[test]
    fn test_set_state() {
        let device = MockDevice {
            name: "MockDevice".to_string(),
            sn: "0".to_string(),
        };
        let mut array_device = ArrayDevice::new(Box::new(device),
                                            ArrayDeviceState::NORMAL, 0);

        assert_eq!(array_device.GetName(), "MockDevice".to_string());
        assert_eq!(array_device.GetSerial(), "0".to_string());

        let new_device = MockDevice {
            name: "NewMockDevice".to_string(),
            sn: "1".to_string(),
        };
        array_device.SetUblock(Box::new(new_device));

        assert_eq!(array_device.prevUblockInfo, "MockDevice(0)".to_string());
        assert_eq!(array_device.GetName(), "NewMockDevice".to_string());
        assert_eq!(array_device.GetSerial(), "1".to_string());
    }
}