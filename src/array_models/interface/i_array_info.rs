// Only immutable data
#[derive(Clone)]
pub struct ArrayInfo {
    pub name: String,
    pub index: u32,
    pub metaRaidType: String,
    pub dataRaidType: String,
    pub uniqueId: u32,
    pub isWriteThroughEnabled: bool,
}
