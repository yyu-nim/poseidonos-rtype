use std::mem;

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

pub struct deviceInfo
{
    pub deviceType: u32,
    pub pad0: [u32; DEVICE_INFO_PADDING_0_NUM],
    pub deviceUid: [u8; DEVICE_UID_SIZE],
    pub deviceState: u32,
    pub pad1: [u32; DEVICE_INFO_PADDING_1_NUM],
}
const_assert_eq!(mem::size_of::<deviceInfo>(), DEVICE_INFO_SIZE);

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