use std::any::Any;
use std::fmt::{Debug, Formatter};
use crate::bio::ubio::Ubio;

pub trait UBlockDevice : Sync + Send {
    fn SubmitAsyncIO(&self, bio: &mut Ubio) -> i32;
    fn CompleteIOs(&self) -> i32;
    fn Close(&self) -> u32;
    fn Open(&mut self) -> bool;
    fn clone_box(&self) -> Box<dyn UBlockDevice>;
}

pub struct UBlockDeviceBase;
// Implement default implementations for Base Class
// .cpp에서는 UBlockDevice를 class로 하여 default implementation을
// 두었으나, rust의 trait 은 상속 개념이 없어 default implementation
// 구현을 할 struct을 별도로 만들었다.
// uint32_t UBlockDevice::Close(void);
// bool UBlockDevice::Open(void);

impl Debug for Box<dyn UBlockDevice> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("UBlockDevice").finish()
    }
}
