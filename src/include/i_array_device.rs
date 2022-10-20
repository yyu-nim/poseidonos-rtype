use std::fmt::{Debug, Formatter};
use crate::device::base::ublock_device::UBlockDevice;

pub trait IArrayDevice {

    fn GetUblock(&self) -> Box<dyn UBlockDevice>;
    fn SetUblock(&self, uBlock: Box<dyn UBlockDevice>);

}

impl Debug for Box<dyn IArrayDevice> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("IArrayDevice").finish()
    }
}