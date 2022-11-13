use std::mem;
use std::mem::size_of;
use log::error;
use serde::{Serialize, Deserialize};
extern crate byteorder;
use byteorder::{ByteOrder, LittleEndian, WriteBytesExt};
use crate::array::array::Array;

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
#[derive(Copy, Clone, Serialize, Deserialize, Debug)]
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
impl deviceInfo {
    fn from_vec_u8(vec_u8: Vec<u8>) -> deviceInfo {
        let devInfo = deviceInfo {
            deviceType: {
                let from = DEVICE_TYPE_OFFSET;
                let to = from + mem::size_of::<u32>();
                LittleEndian::read_u32( &vec_u8[from..to] )
            },
            pad0: {
                let from = DEVICE_INFO_PADDING_0_OFFSET;
                let to = from + (mem::size_of::<u32>() * DEVICE_INFO_PADDING_0_NUM);
                &utils::transform_vec8_to_vec32( &vec_u8[from..to] )
            }.clone().try_into().unwrap(),
            deviceUid: {
                let from = DEVICE_UID_OFFSET;
                let to = from + (mem::size_of::<u8>() * DEVICE_UID_SIZE);
                &vec_u8[from..to]
            }.clone().try_into().unwrap(),
            deviceState: {
                let from = DEVICE_STATE_OFFSET;
                let to = from + mem::size_of::<u32>();
                LittleEndian::read_u32( &vec_u8[from..to] )
            },
            pad1: {
                let from = DEVICE_INFO_PADDING_1_OFFSET;
                let to = from + (mem::size_of::<u32>() * DEVICE_INFO_PADDING_1_NUM);
                &utils::transform_vec8_to_vec32( &vec_u8[from..to] )
            }.clone().try_into().unwrap(),
        };
        devInfo
    }
}

#[derive(Copy, Clone, Debug)]
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
impl ArrayBootRecord {
    fn from_vec_u8(vec_u8: Vec<u8>) -> ArrayBootRecord {
        let abr = ArrayBootRecord {
            arrayName: {
                let from = ARRAY_NAME_OFFSET;
                let to = from + ARRAY_NAME_SIZE;
                &vec_u8[from..to]
            }.clone().try_into().unwrap(),
            abrVersion: {
                let from = ABR_VERSION_OFFSET;
                let to = from + mem::size_of::<i32>();
                LittleEndian::read_i32(&vec_u8[from..to])
            },
            pad0: {
                let from = ABR_PADDING_0_OFFSET;
                let to = from + (mem::size_of::<u32>() * ABR_PADDING_0_SIZE);
                &utils::transform_vec8_to_vec32( &vec_u8[from..to] )
            }.clone().try_into().unwrap(),
            metaRaidType: {
                let from = META_RAID_TYPE_OFFSET;
                let to = from + (mem::size_of::<u8>() * META_RAID_TYPE_SIZE);
                &vec_u8[from..to]
            }.clone().try_into().unwrap(),
            dataRaidType: {
                let from = DATA_RAID_TYPE_OFFSET;
                let to  = from + (mem::size_of::<u8>() * DATA_RAID_TYPE_SIZE);
                &vec_u8[from..to]
            }.clone().try_into().unwrap(),
            totalDevNum: {
                let from = TOTAL_DEV_NUM_OFFSET;
                let to = from + mem::size_of::<u32>();
                LittleEndian::read_u32( &vec_u8[from..to] )
            },
            nvmDevNum: {
                let from = NVM_DEV_NUM_OFFSET;
                let to = from + mem::size_of::<u32>();
                LittleEndian::read_u32( &vec_u8[from..to] )
            },
            dataDevNum: {
                let from = DATA_DEV_NUM_OFFSET;
                let to = from + mem::size_of::<u32>();
                LittleEndian::read_u32( &vec_u8[from..to] )
            },
            spareDevNum: {
                let from = SPARE_DEV_NUM_OFFSET;
                let to = from + mem::size_of::<u32>();
                LittleEndian::read_u32( &vec_u8[from..to] )
            },
            mfsInit: {
                let from = MFS_INIT_OFFSET;
                let to = from + mem::size_of::<u32>();
                LittleEndian::read_u32( &vec_u8[from..to] )
            },
            createDatetime: {
                let from = CREATE_DATE_OFFSET;
                let to = from + (mem::size_of::<u8>() * DATE_SIZE);
                &vec_u8[from..to]
            }.clone().try_into().unwrap(),
            updateDatetime: {
                let from = MODIFIED_DATE_OFFSET;
                let to = from + (mem::size_of::<u8>() * DATE_SIZE);
                &vec_u8[from..to]
            }.clone().try_into().unwrap(),
            uniqueId: {
                let from = INSTANCE_ID_OFFSET;
                let to = from + mem::size_of::<u32>();
                LittleEndian::read_u32( &vec_u8[from..to] )
            },
            pad1: {
                let from = ABR_PADDING_1_OFFSET;
                let to = from + (mem::size_of::<u32>() * ABR_PADDING_1_NUM);
                &utils::transform_vec8_to_vec32( &vec_u8[from..to] )
            }.clone().try_into().unwrap(),
            devInfo: {
                let mut devInfoVec = Vec::new();
                let from = ABR_DEVICE_INFO_OFFSET;
                let to = from + (mem::size_of::<deviceInfo>() * MAX_ARRAY_DEVICE_CNT);
                for chunk_boundary in (from..to).step_by( mem::size_of::<deviceInfo>() ) {
                    let chunk = &vec_u8[chunk_boundary..(chunk_boundary+mem::size_of::<deviceInfo>())];
                    let info = deviceInfo::from_vec_u8(chunk.to_vec());
                    devInfoVec.push(info);
                }
                devInfoVec
            }.try_into().unwrap(),
            reserved: {
                let from = ABR_RESERVED_OFFSET;
                let to = from + (mem::size_of::<u32>() * ABR_RESERVED_NUM);
                &utils::transform_vec8_to_vec32( &vec_u8[from..to] )
            }.clone().try_into().unwrap(),
        };
        abr
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

impl masterBootRecord {
    pub fn from_vec_u8(vec_u8: Vec<u8>) -> Option<Box<masterBootRecord>> {
        if vec_u8.len() != MBR_SIZE {
            error!("cannot deserialize MBR! expected = {}, actual = {}", MBR_SIZE, vec_u8.len());
            return None
        }
        // Box가 없으면, MBR을 local stack에 할당하다 overflow 발생함. Heap 사용해야 함.
        let mut mbr = Box::new(masterBootRecord::default());
        mbr.posVersion.copy_from_slice({
            let from = POS_VERSION_OFFSET;
            let to = from + (mem::size_of::<u8>() * POS_VERSION_SIZE);
            &vec_u8[from..to]
        });
        mbr.pad0.copy_from_slice({
            let from = MBR_PADDING_0_OFFSET;
            let to = from + (mem::size_of::<u32>() * MBR_PADDING_0_NUM);
            &utils::transform_vec8_to_vec32(&vec_u8[from..to])
        });
        mbr.mbrVersion = {
            let from = MBR_VERSION_OFFSET;
            let to = from + 4;
            LittleEndian::read_u32( &vec_u8[from..to])
        };
        mbr.pad1.copy_from_slice({
            let from = MBR_PADDING_1_OFFSET;
            let to = from + (mem::size_of::<u32>() * MBR_PADDING_1_NUM);
            &utils::transform_vec8_to_vec32(&vec_u8[from..to])
        });
        mbr.systemUuid.copy_from_slice({
            let from = SYSTEM_UUID_OFFSET;
            let to = from + (mem::size_of::<u8>() * SYSTEM_UUID_SIZE);
            &vec_u8[from..to]
        });
        mbr.arrayNum = {
            let from = ARRAY_NUM_OFFSET;
            let to = from + mem::size_of::<u32>();
            LittleEndian::read_u32( &vec_u8[from..to] )
        };
        mbr.pad2.copy_from_slice({
            let from = MBR_PADDING_2_OFFSET;
            let to = from + (mem::size_of::<u32>() * MBR_PADDING_2_NUM);
            &utils::transform_vec8_to_vec32( &vec_u8[from..to] )
        });
        mbr.arrayValidFlag.copy_from_slice({
            let from = ARRAY_FLAG_OFFSET;
            let to = from + (mem::size_of::<u32>() * MAX_ARRAY_CNT);
            &utils::transform_vec8_to_vec32( &vec_u8[from..to] )
        });
        mbr.pad3.copy_from_slice({
            let from = MBR_PADDING_3_OFFSET;
            let to = from + (mem::size_of::<u32>() * MBR_PADDING_3_NUM);
            &utils::transform_vec8_to_vec32( &vec_u8[from..to] )
        });
        mbr.arrayDevFlag.copy_from_slice({
            let from = ARRAY_DEVICE_FLAG_OFFSET;
            let to = from + (mem::size_of::<u32>() * MAX_ARRAY_DEVICE_CNT);
            &utils::transform_vec8_to_vec32( &vec_u8[from..to] )
        });
        mbr.pad4.copy_from_slice({
            let from = MBR_PADDING_4_OFFSET;
            let to = from + (mem::size_of::<u32>() * MBR_PADDING_4_NUM);
            &utils::transform_vec8_to_vec32( &vec_u8[from..to] )
        });
        {
            let mut arrayIdx = 0;
            let from = MBR_ABR_OFFSET;
            let to = from + (mem::size_of::<ArrayBootRecord>() * MAX_ARRAY_CNT);
            for chunk_boundary in (from..to).step_by( mem::size_of::<ArrayBootRecord>() ) {
                let chunk = &vec_u8[chunk_boundary..(chunk_boundary + mem::size_of::<ArrayBootRecord>())];
                let abr = ArrayBootRecord::from_vec_u8(chunk.to_vec());
                mbr.arrayInfo[arrayIdx] = abr;
                arrayIdx += 1;
            }
        }
        mbr.reserved.copy_from_slice({
            let from = MBR_RESERVED_OFFSET;
            let to = from + (mem::size_of::<u32>() * MBR_RESERVED_NUM);
            &utils::transform_vec8_to_vec32( &vec_u8[from..to] )
        });
        mbr.mbrParity = {
            let from = MBR_PARITY_OFFSET;
            let to = from + MBR_PARITY_SIZE;
            LittleEndian::read_u32( &vec_u8[from..to] )
        };
        Some(mbr)
    }

    pub fn to_string(&self) -> String {
        let mut str_buf = String::new();
        str_buf.push_str(format!("pos_version: {}",
                                 String::from_utf8(self.posVersion.to_vec()).unwrap_or("not a string".to_string())).as_str());
        str_buf.push_str(format!("mbr_version: {}", self.mbrVersion).as_str());
        str_buf.push_str(format!("system_uuid: {}",
                                 String::from_utf8(self.systemUuid.to_vec()).unwrap_or("not a string".to_string())).as_str());
        str_buf.push_str(format!("num_of_array: {}", self.arrayNum).as_str());
        let arrayInfo = &self.arrayInfo;
        for i in 0..MAX_ARRAY_CNT {
            if self.arrayValidFlag[i] != 1 {
                continue;
            }
            let abr = &arrayInfo[i];
            str_buf.push_str(format!("array_name: {}", String::from_utf8(abr.arrayName.to_vec()).unwrap()).as_str());
            str_buf.push_str(format!("uniqueId: {}", abr.uniqueId).as_str());
            str_buf.push_str(format!("abr_version: {}", abr.abrVersion).as_str());
            str_buf.push_str(format!("total_dev_num: {}", abr.totalDevNum).as_str());
            str_buf.push_str(format!("data_dev_num: {}", abr.dataDevNum).as_str());
            str_buf.push_str(format!("spare_dev_num: {}", abr.spareDevNum).as_str());
            str_buf.push_str(format!("create_datetime: {}", String::from_utf8(abr.createDatetime.to_vec()).unwrap()).as_str());
            str_buf.push_str(format!("update_datetime: {}", String::from_utf8(abr.updateDatetime.to_vec()).unwrap()).as_str());
        }

        str_buf
    }
}

mod utils {
    use byteorder::{ByteOrder, LittleEndian};

    pub fn transform_vec32_to_vec8(from: Vec<u32>) -> Vec<u8> {
        let mut accumulated = Vec::new();
        for the_u32 in from.iter() {
            accumulated.append( the_u32.to_le_bytes().to_vec().as_mut() );
        }
        accumulated
    }

    pub fn transform_vec8_to_vec32(from: &[u8]) -> Vec<u32> {
        let mut accumulated: Vec<u32> = Vec::new();
        for byte_pos in (0..from.len()).step_by(4) {
            // TODO: what if from isn't 4-bytes aligned?
            accumulated.push( LittleEndian::read_u32(&from[byte_pos..]) );
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