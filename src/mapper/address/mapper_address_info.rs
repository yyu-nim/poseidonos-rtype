use crate::{
    array_models::interface::i_array_info::ArrayInfo, include::partition_type::PartitionType,
};

pub struct MapperAddressInfo {
    pub maxVsid: u32,
    pub blksPerStripe: u32,
    pub numWbStripes: u32,
    pub mpageSize: u32,
}

impl MapperAddressInfo {
    pub fn new(arrayInfo: &ArrayInfo, mpageSize: u32) -> Self {
        let user_partition_size = arrayInfo
            .partitionSizInfo
            .get(&PartitionType::USER_DATA)
            .unwrap();
        let wb_partition_size = arrayInfo
            .partitionSizInfo
            .get(&PartitionType::WRITE_BUFFER)
            .unwrap();

        Self {
            maxVsid: user_partition_size.totalStripes,
            blksPerStripe: user_partition_size.blksPerStripe,
            numWbStripes: wb_partition_size.totalStripes,
            mpageSize,
        }
    }
}
