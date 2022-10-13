use crate::array::meta::array_meta::ArrayMeta;

pub trait IAbrControl : Sync {
    fn CreateAbr(&self, meta: ArrayMeta) -> i32;
    fn DeleteAbr(&self, arrayName: String) -> i32;
    fn LoadAbr(&self, meta: ArrayMeta) -> i32;
    fn SaveAbr(&self, meta: ArrayMeta) -> i32;
    fn ResetMbr(&self) -> i32;
    fn FindArrayWithDeviceSN(&self, devSN: String) -> String;
    fn GetLastUpdatedDateTime(&self, arrayName: String) -> String;
    fn GetCreatedDateTime(&self, arrayNameL: String) -> String;
}