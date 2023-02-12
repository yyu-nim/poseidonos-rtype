use log::error;
use crate::array_models::dto::partition_logical_size::PartitionLogicalSize;
use crate::array_models::interface::i_array_info::ArrayInfo;
use crate::include::partition_type::PartitionType;

pub struct AllocatorAddressInfo {
    pub blks_per_stripe: u32,
    pub chunks_per_stripe: u32,
    pub num_wb_stripes: u32,
    pub num_userarea_stripes: u32,
    pub blks_per_segment: u32,
    pub stripes_per_segment: u32,
    pub num_userarea_segments: u32,
}

impl Default for AllocatorAddressInfo {
    fn default() -> Self {
        AllocatorAddressInfo {
            blks_per_stripe: 0,
            chunks_per_stripe: 0,
            num_wb_stripes: 0,
            num_userarea_stripes: 0,
            blks_per_segment: 0,
            stripes_per_segment: 0,
            num_userarea_segments: 0
        }
    }
}

impl AllocatorAddressInfo {
    pub fn Init(&mut self, array_info: &ArrayInfo) {
        let ud_size = array_info.partition_size_info.get(&PartitionType::USER_DATA);
        let wb_size = array_info.partition_size_info.get(&PartitionType::WRITE_BUFFER);

        match (ud_size, wb_size) {
            (Some(ud_size), Some(wb_size)) => {
                self.blks_per_stripe = ud_size.blks_per_stripe;
                self.chunks_per_stripe = ud_size.chunks_per_stripe;
                self.num_wb_stripes = wb_size.total_stripes;
                self.num_userarea_stripes = ud_size.total_stripes;
                self.blks_per_segment = ud_size.blks_per_stripe * ud_size.stripes_per_segment;
                self.stripes_per_segment = ud_size.stripes_per_segment;
                self.num_userarea_segments = ud_size.total_segments;
            }
            (None, _) => {
                error!("The partition size for USER_DATA isn't provided!");
            }
            (_, None) => {
                error!("Theh partition size for WRITE_BUFFER isn't provided!");
            }
        }
    }
}