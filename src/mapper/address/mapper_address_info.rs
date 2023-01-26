use crate::{
    array_models::interface::i_array_info::ArrayInfo, include::partition_type::PartitionType,
};

#[derive(Clone)]
pub struct MapperAddressInfo {
    pub max_vsid: u32,
    pub blks_per_stripe: u32,
    pub num_wb_stripes: u32,
    pub mpage_size: u32,
    pub array_id: u32,
}

impl MapperAddressInfo {
    pub fn new(arrayInfo: &ArrayInfo, mpageSize: u32) -> Self {
        let user_partition_size = arrayInfo
            .partition_size_info
            .get(&PartitionType::USER_DATA)
            .unwrap();
        let wb_partition_size = arrayInfo
            .partition_size_info
            .get(&PartitionType::WRITE_BUFFER)
            .unwrap();

        Self {
            max_vsid: user_partition_size.total_stripes,
            blks_per_stripe: user_partition_size.blks_per_stripe,
            num_wb_stripes: wb_partition_size.total_stripes,
            mpage_size: mpageSize,
            array_id: arrayInfo.index,
        }
    }
}
