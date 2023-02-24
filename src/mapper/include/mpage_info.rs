use std::collections::HashSet;

use crate::include::address_type::VirtualBlkAddr;
use crate::bio::ubio::MAX_PROCESSABLE_BLOCK_COUNT;

pub type MpageNum = u64;
pub type MpageList = HashSet<MpageNum>;
pub type VsaArray = [VirtualBlkAddr; MAX_PROCESSABLE_BLOCK_COUNT as usize];