use anyhow::Result;

use crate::mbr::mbr_info::{ArrayBootRecord, masterBootRecord};
use crate::array::meta::array_meta::ArrayMeta;

pub struct MbrManager;

impl MbrManager {
    pub fn new() -> Self {
        MbrManager
    }

    pub fn GetMbr(&self) -> masterBootRecord { todo!(); }
    pub fn LoadMbr(&self) -> Result<()> { todo!();  }
    pub fn SaveMbr(&self) -> Result<()> { todo!();  }
    pub fn ResetMbr(&self) -> Result<()> { todo!(); }
    pub fn InitDisk(&self, /* dev: UblockSharedPtr*/ ) { todo!(); }
    pub fn CreateAbr(&self, meta: ArrayMeta) -> Result<()> { todo!(); }
    pub fn DeleteAbr(&self, name: &String) -> Result<()> { todo!(); }
    pub fn GetAbr(&self, name: &String) -> Option<(ArrayBootRecord, u32)> { todo!(); }
    pub fn GetAbrList(&self) -> Result<Vec::<ArrayBootRecord>> { todo!(); }
    pub fn GetMbrVersionInMemory(&self) -> Result<i32> { todo!(); }
    pub fn UpdateDeviceIndexMap(&self, arrayName: &String) -> Result<()> { todo!(); }
    pub fn FindArrayWithDeviceSN(&self, devSN: String) -> String { String::new() }
    pub fn Serialize(&self) -> String { todo!(); }
}