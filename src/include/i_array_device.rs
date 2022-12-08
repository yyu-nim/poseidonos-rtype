use std::fmt::{Debug, Formatter};
use crate::device::base::ublock_device::UBlockDevice;

pub trait IArrayDevice: Send {

    fn GetUblock(&self) -> Box<dyn UBlockDevice>;
    fn SetUblock(&mut self, uBlock: Box<dyn UBlockDevice>);

}

impl Debug for Box<dyn IArrayDevice> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("IArrayDevice").finish()
    }
}

impl Clone for Box<dyn IArrayDevice> {
    fn clone(&self) -> Self {
        todo!()
    }
}
