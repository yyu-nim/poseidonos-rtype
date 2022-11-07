use std::mem;
use serde::{Serialize, Deserialize};

pub const MAX_ARRAY_CNT: usize = 8;
pub const MAX_ARRAY_DEVICE_CNT: usize = 128;
pub const POS_VERSION_SIZE: usize = 16;
pub const SYSTEM_UUID_SIZE: usize = 64;

pub const ARRAY_NAME_SIZE: usize = 64;
pub const META_RAID_TYPE_SIZE: usize = 16;
pub const DATA_RAID_TYPE_SIZE: usize = 16;
pub const DEVICE_UID_SIZE: usize = 32;
pub const DATE_SIZE: usize = 32;

pub const DEVICE_TYPE_OFFSET: usize = 0;
pub const DEVICE_INFO_PADDING_0_OFFSET: usize = DEVICE_TYPE_OFFSET + mem::size_of::<u32>();
pub const DEVICE_INFO_PADDING_0_NUM: usize = 3;
pub const DEVICE_UID_OFFSET: usize = DEVICE_INFO_PADDING_0_OFFSET + mem::size_of::<u32>() * DEVICE_INFO_PADDING_0_NUM;
pub const DEVICE_STATE_OFFSET: usize = DEVICE_UID_OFFSET + mem::size_of::<u8>() * DEVICE_UID_SIZE;
pub const DEVICE_INFO_SIZE: usize = 128;
pub const DEVICE_INFO_PADDING_1_OFFSET: usize = DEVICE_STATE_OFFSET + mem::size_of::<u32>();
pub const DEVICE_INFO_PADDING_1_NUM: usize = (DEVICE_INFO_SIZE - DEVICE_INFO_PADDING_1_OFFSET) / mem::size_of::<u32>();

pub const ARRAY_NAME_OFFSET: usize = 0;
pub const ABR_VERSION_OFFSET: usize = ARRAY_NAME_OFFSET + mem::size_of::<u8>() * ARRAY_NAME_SIZE;
pub const ABR_PADDING_0_OFFSET: usize = ABR_VERSION_OFFSET + mem::size_of::<u32>();
pub const ABR_PADDING_0_SIZE: usize = 7;
pub const META_RAID_TYPE_OFFSET: usize = ABR_PADDING_0_OFFSET + mem::size_of::<u32>() * ABR_PADDING_0_SIZE;
pub const DATA_RAID_TYPE_OFFSET: usize = META_RAID_TYPE_OFFSET + mem::size_of::<u8>() * META_RAID_TYPE_SIZE;
pub const TOTAL_DEV_NUM_OFFSET: usize = DATA_RAID_TYPE_OFFSET + mem::size_of::<u8>() * DATA_RAID_TYPE_SIZE;
pub const NVM_DEV_NUM_OFFSET: usize = TOTAL_DEV_NUM_OFFSET + mem::size_of::<u32>();
pub const DATA_DEV_NUM_OFFSET: usize = NVM_DEV_NUM_OFFSET + mem::size_of::<u32>();
pub const SPARE_DEV_NUM_OFFSET: usize = DATA_DEV_NUM_OFFSET + mem::size_of::<u32>();
pub const MFS_INIT_OFFSET: usize = SPARE_DEV_NUM_OFFSET + mem::size_of::<u32>();
pub const CREATE_DATE_OFFSET: usize = MFS_INIT_OFFSET + mem::size_of::<u32>();
pub const MODIFIED_DATE_OFFSET: usize = CREATE_DATE_OFFSET + mem::size_of::<u8>() * DATE_SIZE;
pub const INSTANCE_ID_OFFSET: usize = MODIFIED_DATE_OFFSET + mem::size_of::<u8>() * DATE_SIZE;
pub const ABR_PADDING_1_OFFSET: usize = INSTANCE_ID_OFFSET + mem::size_of::<u32>();
pub const ABR_DEVICE_INFO_OFFSET: usize = 1024;
pub const ABR_PADDING_1_NUM: usize = (ABR_DEVICE_INFO_OFFSET - ABR_PADDING_1_OFFSET) / mem::size_of::<u32>();
pub const ABR_RESERVED_OFFSET: usize = ABR_DEVICE_INFO_OFFSET + DEVICE_INFO_SIZE * MAX_ARRAY_DEVICE_CNT;
pub const ABR_RESERVED_NUM: usize = 128;
pub const ABR_SIZE: usize = ABR_RESERVED_OFFSET + mem::size_of::<u32>() * ABR_RESERVED_NUM;

pub const POS_VERSION_OFFSET: usize = 0;
pub const MBR_PADDING_0_OFFSET: usize = POS_VERSION_OFFSET + mem::size_of::<u8>() * POS_VERSION_SIZE;
pub const MBR_PADDING_0_NUM: usize = 4;
pub const MBR_VERSION_OFFSET: usize = MBR_PADDING_0_OFFSET + mem::size_of::<u32>() * MBR_PADDING_0_NUM;
pub const MBR_PADDING_1_OFFSET: usize = MBR_VERSION_OFFSET + mem::size_of::<u32>();
pub const MBR_PADDING_1_NUM: usize = 7;
pub const SYSTEM_UUID_OFFSET: usize = MBR_PADDING_1_OFFSET + mem::size_of::<u32>() * MBR_PADDING_1_NUM;
pub const ARRAY_NUM_OFFSET: usize = SYSTEM_UUID_OFFSET + mem::size_of::<u8>() * SYSTEM_UUID_SIZE;
pub const MBR_PADDING_2_OFFSET: usize = ARRAY_NUM_OFFSET + mem::size_of::<u32>();
pub const MBR_PADDING_2_NUM: usize = 7;
pub const ARRAY_FLAG_OFFSET: usize = MBR_PADDING_2_OFFSET + mem::size_of::<u32>() * MBR_PADDING_2_NUM;
pub const MBR_PADDING_3_OFFSET: usize = ARRAY_FLAG_OFFSET + mem::size_of::<u32>() * MAX_ARRAY_CNT;
pub const MBR_PADDING_3_NUM: usize = 4;
pub const ARRAY_DEVICE_FLAG_OFFSET: usize = MBR_PADDING_3_OFFSET + mem::size_of::<u32>() * MBR_PADDING_3_NUM;
pub const MBR_PADDING_4_OFFSET: usize = ARRAY_DEVICE_FLAG_OFFSET + mem::size_of::<u32>() * MAX_ARRAY_DEVICE_CNT;
pub const MBR_ABR_OFFSET: usize = 4096;
pub const MBR_PADDING_4_NUM: usize = (MBR_ABR_OFFSET - MBR_PADDING_4_OFFSET) / mem::size_of::<u32>();
pub const MBR_RESERVED_OFFSET: usize = MBR_ABR_OFFSET + ABR_SIZE * MAX_ARRAY_CNT;
pub const MBR_SIZE: usize = 262144;
pub const MBR_PARITY_SIZE: usize = mem::size_of::<u32>();
pub const MBR_RESERVED_NUM: usize = (MBR_SIZE - MBR_PARITY_SIZE - MBR_RESERVED_OFFSET) / mem::size_of::<u32>();
pub const MBR_PARITY_OFFSET: usize = MBR_RESERVED_OFFSET + mem::size_of::<u32>() * MBR_RESERVED_NUM;

pub trait IntoVecOfU8 {
    fn to_vec_u8(&self) -> Vec<u8>;
}
#[derive(Copy, Clone, Serialize, Deserialize)]
pub struct deviceInfo
{
    pub deviceType: u32,
    pub pad0: [u32; DEVICE_INFO_PADDING_0_NUM],
    pub deviceUid: [u8; DEVICE_UID_SIZE],
    pub deviceState: u32,
    pub pad1: [u32; DEVICE_INFO_PADDING_1_NUM],
}
const_assert_eq!(mem::size_of::<deviceInfo>(), DEVICE_INFO_SIZE);
impl Default for deviceInfo {
    fn default() -> Self {
        deviceInfo {
            deviceType: 0,
            pad0: [0; DEVICE_INFO_PADDING_0_NUM],
            deviceUid: [0; DEVICE_UID_SIZE],
            deviceState: 0,
            pad1: [0; DEVICE_INFO_PADDING_1_NUM]
        }
    }
}
impl IntoVecOfU8 for deviceInfo {
    fn to_vec_u8(&self) -> Vec<u8> {
        bincode::serialize(&self).unwrap()
    }
}

#[derive(Copy, Clone)]
pub struct ArrayBootRecord
{
    pub arrayName: [u8; ARRAY_NAME_SIZE],
    pub abrVersion: i32,
    pub pad0: [u32; ABR_PADDING_0_SIZE],
    pub metaRaidType: [u8; META_RAID_TYPE_SIZE],
    pub dataRaidType: [u8; DATA_RAID_TYPE_SIZE],
    pub totalDevNum: u32,
    pub nvmDevNum: u32,
    pub dataDevNum: u32,
    pub spareDevNum: u32,
    pub mfsInit: u32,
    pub createDatetime: [u8; DATE_SIZE],
    pub updateDatetime: [u8; DATE_SIZE],
    pub uniqueId: u32,
    pub pad1: [u32; ABR_PADDING_1_NUM],
    pub devInfo: [deviceInfo; MAX_ARRAY_DEVICE_CNT],
    pub reserved: [u32; ABR_RESERVED_NUM],
}
const_assert_eq!(mem::size_of::<ArrayBootRecord>(), ABR_SIZE);
impl Default for ArrayBootRecord {
    fn default() -> Self {
        ArrayBootRecord {
            arrayName: [0; ARRAY_NAME_SIZE],
            abrVersion: 0,
            pad0: [0; ABR_PADDING_0_SIZE],
            metaRaidType: [0; META_RAID_TYPE_SIZE],
            dataRaidType: [0; DATA_RAID_TYPE_SIZE],
            totalDevNum: 0,
            nvmDevNum: 0,
            dataDevNum: 0,
            spareDevNum: 0,
            mfsInit: 0,
            createDatetime: [0; DATE_SIZE],
            updateDatetime: [0; DATE_SIZE],
            uniqueId: 0,
            pad1: [0; ABR_PADDING_1_NUM],
            devInfo: [Default::default(); MAX_ARRAY_DEVICE_CNT],
            reserved: [0; ABR_RESERVED_NUM]
        }
    }
}
impl IntoVecOfU8 for ArrayBootRecord {
    fn to_vec_u8(&self) -> Vec<u8> {
        // bincode::serialize(&self).unwrap() // TODO => master boot record의 serialize()와 동일한 문제
        // 일단은 수동으로 serialize
        let mut v = Vec::new();
        v.append( self.arrayName.to_vec().as_mut() );
        v.append( self.abrVersion.to_le_bytes().to_vec().as_mut() );
        v.append( &mut utils::transform_vec32_to_vec8( self.pad0.to_vec()) );
        v.append( self.metaRaidType.to_vec().as_mut() );
        v.append( self.dataRaidType.to_vec().as_mut() );
        v.append( self.totalDevNum.to_le_bytes().to_vec().as_mut() );
        v.append( self.nvmDevNum.to_le_bytes().to_vec().as_mut() );
        v.append( self.dataDevNum.to_le_bytes().to_vec().as_mut() );
        v.append( self.spareDevNum.to_le_bytes().to_vec().as_mut() );
        v.append( self.mfsInit.to_le_bytes().to_vec().as_mut() );
        v.append( self.createDatetime.to_vec().as_mut() );
        v.append( self.updateDatetime.to_vec().as_mut() );
        v.append( self.uniqueId.to_le_bytes().to_vec().as_mut() );
        v.append( &mut utils::transform_vec32_to_vec8( self.pad1.to_vec()) );
        for single_dev_info in self.devInfo.iter() {
            v.append( &mut single_dev_info.to_vec_u8() );
        }
        v.append( &mut utils::transform_vec32_to_vec8( self.reserved.to_vec()) );
        v
    }
}

pub struct masterBootRecord
{
    pub posVersion: [u8; POS_VERSION_SIZE],
    pub pad0: [u32; MBR_PADDING_0_NUM],
    pub mbrVersion: u32,
    pub pad1: [u32; MBR_PADDING_1_NUM],
    pub systemUuid: [u8; SYSTEM_UUID_SIZE],
    pub arrayNum: u32,
    pub pad2: [u32; MBR_PADDING_2_NUM],
    pub arrayValidFlag: [u32; MAX_ARRAY_CNT],
    pub pad3: [u32; MBR_PADDING_3_NUM],
    pub arrayDevFlag: [u32; MAX_ARRAY_DEVICE_CNT],
    pub pad4: [u32; MBR_PADDING_4_NUM],
    pub arrayInfo: [ArrayBootRecord; MAX_ARRAY_CNT],
    pub reserved: [u32; MBR_RESERVED_NUM],
    pub mbrParity: u32,
}
const_assert_eq!(mem::size_of::<masterBootRecord>(), MBR_SIZE);
impl Default for masterBootRecord {
    fn default() -> Self {
        masterBootRecord {
            posVersion: [0; POS_VERSION_SIZE],
            pad0: [0; MBR_PADDING_0_NUM],
            mbrVersion: 0,
            pad1: [0; MBR_PADDING_1_NUM],
            systemUuid: [0; SYSTEM_UUID_SIZE],
            arrayNum: 0,
            pad2: [0; MBR_PADDING_2_NUM],
            arrayValidFlag: [0; MAX_ARRAY_CNT],
            pad3: [0; MBR_PADDING_3_NUM],
            arrayDevFlag: [0; MAX_ARRAY_DEVICE_CNT],
            pad4: [0; MBR_PADDING_4_NUM],
            arrayInfo: [Default::default(); MAX_ARRAY_CNT],
            reserved: [0; MBR_RESERVED_NUM],
            mbrParity: 0
        }
    }
}
impl IntoVecOfU8 for masterBootRecord {
    fn to_vec_u8(&self) -> Vec<u8> {
        // bincode::serialize(&self).unwrap() // TODO => not sure why we run into error[E0277]: the trait bound `[u32; 28671]: Deserialize<'_>` is not satisfied
        // bincode 이슈를 해결하지 못해, 일단은 수동으로 serialize 함.
        let mut v = Vec::with_capacity(mem::size_of::<masterBootRecord>());
        v.append( self.posVersion.to_vec().as_mut() );
        v.append( &mut utils::transform_vec32_to_vec8(self.pad0.to_vec()) );
        v.append( self.mbrVersion.to_le_bytes().to_vec().as_mut() ); // TODO: need any support for big endian CPU?
        v.append( &mut utils::transform_vec32_to_vec8(self.pad1.to_vec()) );
        v.append( self.systemUuid.to_vec().as_mut() );
        v.append( self.arrayNum.to_le_bytes().to_vec().as_mut() );
        v.append( &mut utils::transform_vec32_to_vec8(self.pad2.to_vec()) );
        v.append( &mut utils::transform_vec32_to_vec8(self.arrayValidFlag.to_vec()) );
        v.append( &mut utils::transform_vec32_to_vec8(self.pad3.to_vec()) );
        v.append( &mut utils::transform_vec32_to_vec8(self.arrayDevFlag.to_vec()) );
        v.append( &mut utils::transform_vec32_to_vec8(self.pad4.to_vec()) );
        for one_array_info in self.arrayInfo.iter() {
            v.append( one_array_info.to_vec_u8().as_mut() );
        }
        v.append( &mut utils::transform_vec32_to_vec8(self.reserved.to_vec()) );
        v.append( self.mbrParity.to_le_bytes().to_vec().as_mut() );
        v
    }

}

mod utils {
    pub fn transform_vec32_to_vec8(from: Vec<u32>) -> Vec<u8> {
        let mut accumulated = Vec::new();
        for the_u32 in from.iter() {
            accumulated.append( the_u32.to_le_bytes().to_vec().as_mut() );
        }
        accumulated
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_arr_to_string() {
        // TODO test with std::ffi::CString
        let mut array_name: [u8; ARRAY_NAME_SIZE] = [0; ARRAY_NAME_SIZE];

        let input = String::from("TestArray");
        let slice = &input.as_bytes()[..];
        assert!(slice.len() < ARRAY_NAME_SIZE);

        for i in 0..slice.len() {
            array_name[i] = slice[i];
        }

        assert!(std::str::from_utf8(&array_name).unwrap().to_string().starts_with("TestArray"));
        assert_eq!(std::mem::size_of_val(&array_name), ARRAY_NAME_SIZE);
    }
}