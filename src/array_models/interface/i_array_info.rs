use std::collections::HashMap;

use crate::{include::partition_type::PartitionType, array_models::dto::partition_logical_size::PartitionLogicalSize};

// Only immutable data
#[derive(Clone, Debug)]
pub struct ArrayInfo {
    pub name: String,
    pub index: u32,
    pub meta_raid_type: String,
    pub data_raid_type: String,
    pub unique_id: u32,
    pub partition_size_info: HashMap<PartitionType, PartitionLogicalSize>,
    pub is_write_through_enabled: bool,
}

impl std::fmt::Display for ArrayInfo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl Default for ArrayInfo {
    fn default() -> Self {
        ArrayInfo {
            name: "array-name".to_string(),
            index: 0,
            meta_raid_type: "no-raid".to_string(),
            data_raid_type: "no-raid".to_string(),
            unique_id: 0,
            partition_size_info: Default::default(),
            is_write_through_enabled: false
        }
    }
}