pub struct AbrManager;

use anyhow::Result;
use crate::mbr::mbr_info::ArrayBootRecord;

impl AbrManager {
    pub fn new() -> Self {
        AbrManager
    }
    pub fn GetAbrList(&self) -> Option<&Vec::<ArrayBootRecord>> {
        // TODO
        None
    }

    pub fn LoadAbr(&self, name: &String) -> Result<()> {
        // TODO
        Ok(())
    }

    pub fn DeleteAbr(&self, name: &String) -> Result<()> {
        // TODO
        Ok(())
    }
}
