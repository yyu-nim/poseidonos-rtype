use crate::array::meta::array_meta::ArrayMeta;
use crate::include::pos_event_id::PosEventId;

pub trait IAbrControl : Sync + Send {
    fn CreateAbr(&self, meta: ArrayMeta) -> Result<(), PosEventId>;
    fn DeleteAbr(&self, arrayName: String) -> Result<(), PosEventId>;
    fn LoadAbr(&self, meta: ArrayMeta) -> Result<(), PosEventId>;
    fn SaveAbr(&self, meta: ArrayMeta) -> Result<(), PosEventId>;
    fn ResetMbr(&self) -> Result<(), PosEventId>;
    fn FindArrayWithDeviceSN(&self, devSN: String) -> String;
    fn GetLastUpdatedDateTime(&self, arrayName: String) -> String;
    fn GetCreatedDateTime(&self, arrayNameL: String) -> String;
}