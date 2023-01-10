use std::collections::HashMap;

use crate::{include::partition_type::PartitionType, array_models::dto::partition_logical_size::PartitionLogicalSize};

// Only immutable data
#[derive(Clone, Debug)]
pub struct ArrayInfo {
    pub name: String,
    pub index: u32,
    pub metaRaidType: String,
    pub dataRaidType: String,
    pub uniqueId: u32,
    pub partitionSizInfo: HashMap<PartitionType, PartitionLogicalSize>,
    pub isWriteThroughEnabled: bool,
}

impl std::fmt::Display for ArrayInfo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}
