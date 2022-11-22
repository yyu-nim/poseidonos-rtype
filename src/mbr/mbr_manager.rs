use std::any::Any;
use std::borrow::BorrowMut;
use std::collections::HashMap;
use std::ops::{Deref, DerefMut};
use std::rc::Rc;
use std::str::FromStr;
use std::string::FromUtf8Error;
use std::sync::{Arc, Mutex, MutexGuard};

use crossbeam::sync::Parker;
use log::{error, info, warn};
use uuid::Uuid;

use crate::array::array_name_policy;
use crate::array::meta::array_meta::ArrayMeta;
use crate::bio::ubio::{CallbackClosure, Ubio, UbioDir};
use crate::device::base::ublock_device::UBlockDevice;
use crate::device::device_manager::{DeviceManager, DeviceManagerConfig};
use crate::event_scheduler::callback::Callback;
use crate::include::meta_const::CHUNK_SIZE;
use crate::include::pos_event_id::PosEventId;
use crate::include::pos_event_id::PosEventId::{MBR_ABR_ALREADY_EXIST, MBR_DATA_NOT_FOUND, MBR_MAX_ARRAY_CNT_EXCEED, MBR_WRONG_ARRAY_INDEX_MAP, MBR_WRONG_ARRAY_VALID_FLAG};
use crate::io_scheduler::io_dispatcher::IODispatcherSingleton;
use crate::master_context::version_provider::VersionProviderSingleton;
use crate::mbr::mbr_info::{ArrayBootRecord, DEVICE_UID_SIZE, deviceInfo, IntoVecOfU8, masterBootRecord, MAX_ARRAY_CNT, MAX_ARRAY_DEVICE_CNT};
use crate::mbr::mbr_map_manager::MbrMapManager;

const MBR_CHUNKS : i32 = 1;
const MBR_ADDRESS: u64 = 0;
const MBR_SIZE: u64 = CHUNK_SIZE;

pub struct MbrManager {
    mbrBuffer: Mutex<Vec<u8>>, // TODO: refactor to use "mbrLock" instead
    mbrLock: Mutex<Option<u8>>,
    systeminfo: masterBootRecord,
    version: i32,
    systemUuid: String,
    devMgr: DeviceManager,
    arrayIndexMap: HashMap<String, u32>,
    mapMgr: MbrMapManager,
}

impl MbrManager {
    pub fn new(config: Option<DeviceManagerConfig>) -> Self {
        let uuid = Uuid::new_v4(); // LoadMbr() 하면 overwrite 될 값임. 그 때까지만 임시로 랜덤 UUID 할당.
        let mut devMgr = DeviceManager::new(config);
        devMgr.ScanDevs();

        MbrManager {
            mbrBuffer: Mutex::new(vec![0 as u8; CHUNK_SIZE as usize]),
            mbrLock: Mutex::new(None),
            systeminfo: Default::default(),
            version: 0,
            systemUuid: uuid.to_string(),
            devMgr,
            arrayIndexMap: Default::default(),
            mapMgr: Default::default()
        }
    }

    pub fn GetMbr(&self) -> &masterBootRecord {
        &self.systeminfo
    }

    pub fn LoadMbr(&mut self) -> Result<(), PosEventId> {
        let mut _mbrLock = self.mbrLock.lock().unwrap();
        let ret = MbrManager::_ReadFromDevices(&self.devMgr, &mut self.version, &mut self.systeminfo);
        if let Err(e) = ret {
            error!("[{}] Failed to load MBR", e.to_string());
            return Err(e);
        }

        info!("[{}] read mbr data done", PosEventId::MBR_READ_DONE.to_string());
        MbrManager::_LoadIndexMap(&mut self.arrayIndexMap,
                                  &mut self.mapMgr,
                                  &mut self.systeminfo);
        if self.systeminfo.arrayNum != self.arrayIndexMap.len() as u32 {
            return Err(PosEventId::MBR_WRONG_ARRAY_INDEX_MAP);
        }
        info!("[{}] mbr_info: {}", PosEventId::POS_TRACE_MBR_LOADED.to_string(), self.systeminfo.to_string());
        Ok(())
    }

    pub fn SaveMbr(&mut self) -> Result<(), PosEventId> {
        let posVersion = VersionProviderSingleton.ver();
        self.systeminfo.mbrVersion = self.version as u32;
        let uuid = Uuid::from_str(self.systemUuid.as_str()).unwrap();
        let uuid_len = uuid.as_bytes().len();
        self.systeminfo.systemUuid[0..uuid_len].copy_from_slice(uuid.as_bytes());
        let result = self._WriteToDevices();
        match result {
            Ok(_) => {
                Ok(())
            }
            Err(event_id) => {
                warn!("Failed to write MBR to devices. Code = {:?}", event_id);
                Err(PosEventId::MBR_WRITE_ERROR)
            }
        }
    }
    pub fn ResetMbr(&self) -> Result<(), PosEventId> { todo!(); }

    pub fn InitDisk(&self, dev: Box<dyn UBlockDevice>) {
        let mut mbrBuffer = self.mbrBuffer.lock().unwrap(); //.borrow_mut();
        mbrBuffer.clear();
        let mut systeminfo = self.systeminfo.to_vec_u8();
        mbrBuffer.append(&mut systeminfo);
        self._SetParity(mbrBuffer.deref_mut());
        let diskIoCtxt = DiskIoContext::new(UbioDir::Write, mbrBuffer.clone());
        MbrManager::_DiskIo(dev, diskIoCtxt);
        info!("the mbr has been initialized");
    }

    pub fn CreateAbr(&mut self, meta: ArrayMeta) -> Result<u32 /* array idx */, PosEventId> {

        let name_validation = array_name_policy::CheckArrayName(&meta.arrayName);
        if let Err(e) = name_validation {
            error!("Array name double check failed: {}", meta.arrayName);
            return Err(e);
        }

        if self.systeminfo.arrayNum > MAX_ARRAY_CNT as u32 {
            return Err(MBR_MAX_ARRAY_CNT_EXCEED);
        }

        let _lock = self.mbrLock.lock().unwrap();
        if self.arrayIndexMap.contains_key(&meta.arrayName) {
            return Err(MBR_ABR_ALREADY_EXIST);
        }

        let ret = self.mapMgr.CheckAllDevices(&meta);
        if let Err(e) = ret {
            error!("CheckAllDevices has failed.");
            return Err(e);
        }

        for i in 0..MAX_ARRAY_CNT {
            if self.systeminfo.arrayValidFlag[i] == 0 {
                if let Some(existing_val) = self.arrayIndexMap.insert(meta.arrayName.clone(), i as u32) {
                    error!("[{}] Skipping the index {}", MBR_WRONG_ARRAY_INDEX_MAP.to_string(), i);
                    continue;
                }

                self.mapMgr.InsertDevices(&meta, i as u32);
                self.systeminfo.arrayValidFlag[i] = 1;
                self.systeminfo.arrayNum += 1;
                self.systeminfo.arrayInfo[i].update_array_name(&meta.arrayName).unwrap();
                self.systeminfo.arrayInfo[i].update_createTime().unwrap();
                self.systeminfo.arrayInfo[i].update_updateTime().unwrap();
                self.systeminfo.arrayInfo[i].uniqueId = meta.unique_id;
                info!("ArrayBootRecord for Array {} has been created", i);
                return Ok(i as u32)
            }
        }

        Err(MBR_WRONG_ARRAY_VALID_FLAG)
    }


    pub fn DeleteAbr(&self, name: &String) -> Result<(), PosEventId> { todo!(); }
    pub fn GetAbr(&self, name: &String) -> Option<(ArrayBootRecord, u32)> { todo!(); }
    pub fn GetAbrList(&self) -> Result<Vec::<ArrayBootRecord>, PosEventId> { todo!(); }
    pub fn GetMbrVersionInMemory(&self) -> Result<i32, PosEventId> { todo!(); }
    pub fn UpdateDeviceIndexMap(&self, arrayName: &String) -> Result<(), PosEventId> { todo!(); }
    pub fn FindArrayWithDeviceSN(&self, devSN: String) -> String { String::new() }
    pub fn Serialize(&self) -> String { todo!(); }

    fn _IterateReadFromDevices(dev: Box<dyn UBlockDevice>, ctx: &mut Vec<Vec<u8>>/*Box<dyn Any>*/) {
        // "ctx" is likely to be byte buffer, so can be refactored accordingly later.
        let mut mems = ctx;
        let mem = [0 as u8; CHUNK_SIZE as usize * MBR_CHUNKS as usize].to_vec();
        let diskIoCtxt = DiskIoContext::new(UbioDir::Read, mem);
        let result_buffer = MbrManager::_DiskIo(dev, diskIoCtxt)
            .expect("Failed to read MBR from a device"); // TODO: device id API 생기면 메시지에 추가
        if !MbrManager::_VerifyParity(&result_buffer) {
            warn!("Failed to verify MBR parity");
            return;
        }
        if !MbrManager::_VerifySystemUuid(&result_buffer) {
            warn!("Failed to verify System UUID from MBR");
            return;
        }
        mems.push(result_buffer);
    }

    // Unlike in pos-cpp, _DiskIo has become 'static' fn (i.e., without &self)
    fn _DiskIo(dev: Box<dyn UBlockDevice>, ctx: DiskIoContext) -> Option<Vec<u8>> {
        let result_buffer = Rc::new(Mutex::new(Vec::new()));
        let io_done_parker = Parker::new();
        let io_done_unparker = io_done_parker.unparker().clone();
        let io_dir = ctx.ubioDir.clone();
        let callback: CallbackClosure = match io_dir {
            UbioDir::Read => {
                let mut result_buffer = result_buffer.clone();
                Box::new(
                    move |read_buffer: &Vec<u8>| {
                        let mut result_buffer = result_buffer.lock().unwrap();
                        for the_byte in read_buffer {
                            result_buffer.push(the_byte.clone());
                        }
                        io_done_unparker.unpark();
                    }
                )
            },
            UbioDir::Write => {
                Box::new(
                    move |_: &Vec<u8>| {
                        io_done_unparker.unpark();
                    }
                )
            },
        };
        let mut bio = Ubio::new(io_dir.clone(), MBR_ADDRESS,
                                ctx.mem, callback);
        bio.uBlock = Some(dev);

        IODispatcherSingleton.lock().unwrap().Submit(bio, true /* not used */, false);

        io_done_parker.park(); // block synchronously here until we get "unparked"
        match io_dir {
            UbioDir::Read => {
                Some(result_buffer.lock().unwrap().clone())
            }
            UbioDir::Write => {
                None
            }
        }
    }

    fn _VerifyParity(mem: &Vec<u8>) -> bool {
        // TODO
        true
    }

    fn _VerifySystemUuid(mem: &Vec<u8>) -> bool {
        // TODO
        true
    }

    fn _SetParity(&self, mem: &mut Vec<u8>) {
        // TODO
    }

    fn _WriteToDevices(&mut self) -> Result<(), PosEventId> {
        self.version = self.version + 1;
        self.systeminfo.mbrVersion = self.version as u32;
        let mut mbrBuffer = {
            self.mbrBuffer.lock().unwrap()
        };
        mbrBuffer.clear();
        let mut systeminfo = self.systeminfo.to_vec_u8();
        mbrBuffer.append(&mut systeminfo);
        self._SetParity(mbrBuffer.deref_mut());

        let diskIoCtxt =  DiskIoContext::new(UbioDir::Write, mbrBuffer.clone());
        let diskIoFunc = Box::new(move |uBlockDev: &Box<dyn UBlockDevice>| {
            // 원래의 pos-cpp semantics 를 유지하려 굳이 이 구조 일단 사용함. 간단히 설명하면,
            // diskIoCtxt의 소유권은 FnMut closure에 넘겨주어, "여러번" clone() 할 수 있게 하고,
            // MbrManager는 DeviceManager에세 UBlockDevice를 "빌려와서", cloned diskIoCtxt를 보낸다.
            // uBlockDev의 clone_box() 비용은 굉장히 싸다. 내부적으로 arc mutex로 file handle의
            // reference만 복사하는 방식이기 때문.
            let diskIoFunc_cloned = diskIoCtxt.clone();
            MbrManager::_DiskIo(uBlockDev.clone_box(), diskIoFunc_cloned);
        });

        let result = self.devMgr.IterateDevicesAndDoFunc(diskIoFunc);
        match result {
            Ok(_) => {
                info!("MBR has been successfully written.");
                Ok(())
            }
            Err(event_id) => {
                self.version = self.version - 1;
                self.systeminfo.mbrVersion = self.version as u32;
                warn!("[MBR_DEVICE_NOT_FOUND] device not found. Code = {:?}", event_id);
                Err(PosEventId::MBR_DEVICE_NOT_FOUND)
            }
        }
    }

    fn _ReadFromDevices(devMgr: &DeviceManager, version: &mut i32, systeminfo: &mut masterBootRecord) -> Result<(), PosEventId> {
        let mut mems = Rc::new(Mutex::new(Vec::<Vec<u8>>::new()));
        let iterateReadFunc = {
            let mut mems = mems.clone();
            Box::new(move |uBlockDev: &Box<dyn UBlockDevice>| {
                let uBlock = uBlockDev.clone_box();
                let mut mems = mems.lock().unwrap();
                MbrManager::_IterateReadFromDevices(uBlock, &mut mems);
            })
        };

        let result = devMgr.IterateDevicesAndDoFunc(iterateReadFunc);
        if let Err(e) = result {
            return Err(e);
        }

        let mbr_list = mems.lock().unwrap();
        if mbr_list.len() == 0 {
            warn!("[{}] mbr data not found", PosEventId::MBR_DATA_NOT_FOUND.to_string());
            return Err(MBR_DATA_NOT_FOUND);
        }

        // Pick up the MBR of the highest version
        let mbr_latest = mbr_list
            .iter()
            .map(|mbrBytes| masterBootRecord::from_vec_u8(mbrBytes.clone()) )
            .filter(|mbr| mbr.is_some())
            .max_by_key(|mbr| mbr.as_ref().unwrap().mbrVersion);
        if let Some(Some(mbr)) = mbr_latest {
            // TODO: pos-cpp performs marjority voting on the MBRs with the latest version, which we don't do with rtype
            *version = mbr.mbrVersion.clone() as i32;
            *systeminfo = *mbr;
        } else {
            warn!("[{}] no mbr data has been extracted", PosEventId::MBR_DATA_NOT_FOUND.to_string());
            return Err(MBR_DATA_NOT_FOUND);
        }

        Ok(())
    }

    fn _LoadIndexMap(arrayIndexMap: &mut HashMap<String, u32>,
                     mapMgr: &mut MbrMapManager,
                     systeminfo: &mut masterBootRecord) {
        arrayIndexMap.clear();
        mapMgr.ResetMap();

        for i in 0..MAX_ARRAY_CNT {
            if systeminfo.arrayValidFlag[i] != 1 {
                continue;
            }
            let arrayName = String::from_utf8(
                systeminfo.arrayInfo[i].arrayName.to_vec());
            let arrayName = match arrayName {
                Ok(n) => n,
                Err(e) => {
                    error!("Failed to parse array name: {}", e);
                    continue;
                }
            };
            if arrayIndexMap.get(&arrayName).is_none() {
                arrayIndexMap.insert(arrayName, i as u32);
                let total_dev_num = systeminfo.arrayInfo[i].totalDevNum;
                for j in 0..total_dev_num {
                    let deviceUid: [u8; DEVICE_UID_SIZE] =
                        systeminfo.arrayInfo[i as usize].devInfo[j as usize].deviceUid;
                    let deviceUidString
                        = String::from_utf8(Vec::from(deviceUid)).unwrap();
                    mapMgr.InsertDevice(&deviceUidString, i as u32);
                }
            }
        }
    }
}

struct DiskIoContext {
    ubioDir: UbioDir,
    mem: Vec<u8>,
}

impl DiskIoContext {
    pub fn new(ubioDir: UbioDir, mem: Vec<u8>) -> DiskIoContext {
        DiskIoContext {
            ubioDir,
            mem
        }
    }
}

impl Clone for DiskIoContext {
    fn clone(&self) -> Self {
        DiskIoContext {
            ubioDir: self.ubioDir.clone(),
            mem: self.mem.clone(),
        }
    }
}

#[cfg(test)]
mod tests {
    use std::env::set_var;
    use std::fs;
    use std::fs::File;
    use std::io::{Read, Seek, SeekFrom, Write};
    use std::path::PathBuf;

    use log::info;

    use crate::array::meta::array_meta::ArrayMeta;
    use crate::array::meta::device_meta::DeviceMeta;
    use crate::array_models::dto::device_set::DeviceSet;
    use crate::bio::ubio::{CallbackClosure, Ubio, UbioDir};
    use crate::device::base::ublock_device::UBlockDevice;
    use crate::device::device_manager::{DeviceManager, DeviceManagerConfig};
    use crate::device::ufile::ufile_ssd::UfileSsd;
    use crate::include::array_device_state::ArrayDeviceState;
    use crate::include::pos_event_id::PosEventId;
    use crate::include::pos_event_id::PosEventId::MBR_DEVICE_ALREADY_IN_ARRAY;
    use crate::include::raid_type::RaidTypeEnum::RAID10;
    use crate::mbr::mbr_info::{ArrayBootRecord, DEVICE_UID_SIZE, deviceInfo, MBR_VERSION_OFFSET};
    use crate::mbr::mbr_manager::{DiskIoContext, MBR_ADDRESS, MbrManager};

    fn setup() {
        // set up the logger for the test context
        set_var("RUST_LOG", "DEBUG");
        env_logger::builder().is_test(true).try_init();
    }

    fn cleanup(dm_config: &DeviceManagerConfig) {
        for i in 0..dm_config.num_devices_to_create {
            let test_ufile_ssd = format!("{}/{}.{}", dm_config.dir_to_lookup,
                                         dm_config.device_prefix, i);
            info!("Cleaning up {} for test init.", test_ufile_ssd);
            fs::remove_file(PathBuf::from(test_ufile_ssd));
        }
    }

    #[test]
    fn test_if_reading_from_MBR_of_a_single_device_works() {
        setup();

        let test_ufile_ssd = "/tmp/test-ufile-ssd";
        fs::remove_file(PathBuf::from(test_ufile_ssd));

        // Given: a UBlockDevice with its MBR filled with a specific pattern
        let PATTERN = vec![0, 2, 4, 6, 1, 3, 5, 7];
        let empty_callback = Box::new(move |_: &Vec<u8>| {});
        let mut ubio = Ubio::new(UbioDir::Write, MBR_ADDRESS, PATTERN.clone(), empty_callback);
        let mut ublock_dev = UfileSsd::new(
            PathBuf::from(test_ufile_ssd), 100*1024*1024)
            .boxed();
        ublock_dev.Open();
        ublock_dev.SubmitAsyncIO(&mut ubio);

        let mut dm_config = DeviceManagerConfig::default();
        dm_config.num_devices_to_create = 0; // ublock_dev를 수동으로 만들기 때문에, 여기서 자동 생성하지 말자.
        let mbr_manager = MbrManager::new(Some(dm_config));
        let mut ctx : Vec<Vec<u8>> = Vec::new();

        // When: MBR manager reads MBR from the device
        MbrManager::_IterateReadFromDevices(ublock_dev, &mut ctx);

        // Then: "ctx" should contain the pattern
        assert_eq!(1, ctx.len());
        let mbr = &ctx[0];
        let expected = PATTERN.to_vec();
        let actual = mbr[0..PATTERN.len()].to_vec();
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_if_InitDisk_actually_writes_MBR_to_a_given_device() {
        setup();
        let test_ufile_ssd = "/tmp/test-ufile-ssd-initdisk";
        fs::remove_file(PathBuf::from(test_ufile_ssd));

        // Given: MBR manager with its MBR version filled with 123
        let mut dm_config = DeviceManagerConfig::default();
        dm_config.num_devices_to_create = 0; // 왜냐면 ublock_dev는 아래에서 수동으로 만들 것이기 때문에.

        let mut mbr_manager = MbrManager::new(Some(dm_config));
        let mbr = &mut mbr_manager.systeminfo;
        mbr.mbrVersion = 123;

        let mut ublock_dev = UfileSsd::new(
            PathBuf::from(test_ufile_ssd), 100*1024*1024)
            .boxed();
        ublock_dev.Open();

        // When: MBR manager initializes a given UBlockDevice
        mbr_manager.InitDisk(ublock_dev);

        // Then: We should be able to see the expected MBR version 123 at byte offset 32 within the UfileSsd
        let mut f = File::open(PathBuf::from(test_ufile_ssd)).unwrap();
        f.seek(SeekFrom::Start(MBR_VERSION_OFFSET as u64));
        let mut buf = [0 as u8; 4];
        let bytes_read = f.read(&mut buf).unwrap();
        assert_eq!(4, bytes_read);
        assert_eq!(buf.to_vec(), 123_u32.to_le_bytes().to_vec());
    }

    #[test]
    fn test_if_SaveMbr_increments_version_on_disk() {
        setup();

        // Given: MbrManager with 4 devices created
        let DEVICE_PREFIX = "TestUfileSsdForSaveMbr";
        let NUM_DEVICES = 4;
        let mut dm_config = DeviceManagerConfig::default();
        dm_config.dir_to_lookup = "/tmp/";
        dm_config.device_prefix = DEVICE_PREFIX;
        dm_config.num_devices_to_create = NUM_DEVICES;

        let mut mbr_manager = MbrManager::new(Some(dm_config));

        // When 1: MbrManager stores in-memory MBR representation into all managed devices
        assert_eq!(0, mbr_manager.systeminfo.mbrVersion);
        let res = mbr_manager.SaveMbr();

        // Then 1: The MBR version of all devices should be equal to 1
        res.unwrap();
        for i in 0..NUM_DEVICES {
            let test_ufile_ssd = format!("/tmp/{}.{}", DEVICE_PREFIX, i);
            verify_MBR_version(test_ufile_ssd.as_str(), 1);
        }

        // When 2: MbrManager stores MBR again
        let res = mbr_manager.SaveMbr();

        // Then 2: The MBR version of all devices should be equal to 2
        res.unwrap();
        for i in 0..NUM_DEVICES {
            let test_ufile_ssd = format!("/tmp/{}.{}", DEVICE_PREFIX, i);
            verify_MBR_version(test_ufile_ssd.as_str(), 2);
        }
    }

    fn verify_MBR_version(test_ufile_ssd: &str, expected: u32) {
        let mut f = File::open(PathBuf::from(test_ufile_ssd.clone()))
            .expect(format!("Failed to open {}!", test_ufile_ssd).as_str());
        f.seek(SeekFrom::Start(MBR_VERSION_OFFSET as u64));
        let mut buf = [0 as u8; 4];
        let bytes_read = f.read(&mut buf).unwrap();

        assert_eq!(4, bytes_read);
        assert_eq!(buf.to_vec(), expected.to_le_bytes().to_vec());
    }

    #[test]
    fn test_if_LoadMbr_succeeds_when_executed_with_uninitialized_devices_without_MBR_data() {
        // Given: MbrManager with 4 devices created (and no SaveMbr invoked)
        let DEVICE_PREFIX = "TestUfileSsdForLoadMbrOnUninitializedMBR.";
        let NUM_DEVICES = 4;
        let mut dm_config = DeviceManagerConfig::default();
        dm_config.dir_to_lookup = "/tmp/";
        dm_config.device_prefix = DEVICE_PREFIX;
        dm_config.num_devices_to_create = NUM_DEVICES;
        setup();
        cleanup(&dm_config);

        let mut mbr_manager = MbrManager::new(Some(dm_config));

        // When: MbrManager loads MBR
        let result = mbr_manager.LoadMbr();

        // Then: MbrManager should return Ok() successfully, and the MBR should be in uninitialized state
        // saying that 1) MBR version is set to 0, and 2) there's no known array
        result.unwrap();
        assert_eq!(0, mbr_manager.systeminfo.mbrVersion);
        assert_eq!(0, mbr_manager.systeminfo.arrayNum);
    }

    #[test]
    fn test_if_LoadMbr_extracts_MBR_written_by_SaveMbr() {

        // Given: MbrManager with 4 devices created and perform 2 SaveMbr()
        let DEVICE_PREFIX = "TestUfileSsdForLoadMbrOnInitializedMBR.";
        let NUM_DEVICES = 4;
        let mut dm_config = DeviceManagerConfig::default();
        dm_config.dir_to_lookup = "/tmp/";
        dm_config.device_prefix = DEVICE_PREFIX;
        dm_config.num_devices_to_create = NUM_DEVICES;
        setup();
        cleanup(&dm_config);
        let mut mbr_manager = MbrManager::new(Some(dm_config));

        mbr_manager.systeminfo.mbrParity = 1212;
        let result = mbr_manager.SaveMbr();
        result.unwrap();

        mbr_manager.systeminfo.mbrParity = 1213;
        let result = mbr_manager.SaveMbr();
        result.unwrap();

        // When: MbrManager loads MBR
        let result = mbr_manager.LoadMbr();

        // Then: MbrManager should be able to see what has been written by 2 SaveMbr()
        result.unwrap();
        let mbr = &mbr_manager.systeminfo;
        assert_eq!(2, mbr.mbrVersion);
        assert_eq!(1213, mbr.mbrParity);
    }

    #[test]
    fn test_if_LoadMbr_handles_MBR_with_existing_array_information() {
        // Given: MbrManager with 4 devices in total with each of Uid set to its device offset (e.g., 0 for devInfo[0], and so on)
        let DEVICE_PREFIX = "TestUfileSsdForLoadMbrOnMBRWithOneArray.";
        let NUM_DEVICES = 4;
        let mut dm_config = DeviceManagerConfig::default();
        dm_config.dir_to_lookup = "/tmp/";
        dm_config.device_prefix = DEVICE_PREFIX;
        dm_config.num_devices_to_create = NUM_DEVICES;
        setup();
        cleanup(&dm_config);
        let mut mbr_manager = MbrManager::new(Some(dm_config));

        let ARRAY_NAME = "POSArray1";
        mbr_manager.systeminfo.arrayValidFlag[0] = 1;
        mbr_manager.systeminfo.arrayNum = 1;
        let mut abr = ArrayBootRecord::default();
        abr.abrVersion = 17;
        abr.arrayName[..ARRAY_NAME.len()].copy_from_slice( ARRAY_NAME.as_bytes());
        abr.totalDevNum = NUM_DEVICES;
        abr.devInfo[0] = {
            let mut devInfo = deviceInfo::default();
            devInfo.deviceUid[0..4].copy_from_slice("dev0".as_bytes());
            devInfo
        };
        abr.devInfo[1] = {
            let mut devInfo = deviceInfo::default();
            devInfo.deviceUid[0..4].copy_from_slice("dev1".as_bytes());
            devInfo
        };
        abr.devInfo[2] = {
            let mut devInfo = deviceInfo::default();
            devInfo.deviceUid[0..4].copy_from_slice("dev2".as_bytes());
            devInfo
        };
        abr.devInfo[3] = {
            let mut devInfo = deviceInfo::default();
            devInfo.deviceUid[0..4].copy_from_slice("dev3".as_bytes());
            devInfo
        };
        mbr_manager.systeminfo.arrayInfo[0] = abr;

        let result = mbr_manager.SaveMbr();
        result.unwrap();

        // When: MbrManager loads MBR
        let result = mbr_manager.LoadMbr();

        // Then: MbrManager should be able to see 1 array with 4 devices attached
        result.unwrap();
        assert_eq!(1, mbr_manager.systeminfo.mbrVersion);
        let mut actual_array_name = String::from_utf8(mbr_manager.systeminfo.arrayInfo[0].arrayName.to_vec()).unwrap();
        actual_array_name.truncate(ARRAY_NAME.len());
        assert_eq!(ARRAY_NAME.to_string(), actual_array_name);
        assert_eq!(1, mbr_manager.systeminfo.arrayNum);

        // Then: MbrMapManager should also be able to find array indices for those 4 devices
        for dev_num in 0..4 {
            let mut actual_device_uid = [0; DEVICE_UID_SIZE];
            actual_device_uid[0..4].copy_from_slice(format!("dev{}", dev_num).as_bytes());
            let expected_array_idx = 0;
            let actual_array_idx = mbr_manager.mapMgr.FindArrayIndex(&String::from_utf8(actual_device_uid.to_vec()).unwrap()).unwrap();
            assert_eq!(expected_array_idx, actual_array_idx);
        }
    }

    #[test]
    fn test_if_CreateAbr_returns_newly_allocated_array_idx_and_rejects_if_the_same_device_is_used_by_other_array() {
        let mut dm_config = DeviceManagerConfig::default();
        setup();
        cleanup(&dm_config);

        // Given 1: MbrManager with 0 ABR (i.e., POS has never been used)
        let mut mbr_manager = MbrManager::new(Some(dm_config));

        // When 1: we create ABR for the first time,
        let devs = DeviceSet {
            nvm: vec![DeviceMeta {
                uid: "nvm1".to_string(),
                state: ArrayDeviceState::NORMAL
            }
            ],
            data: vec![DeviceMeta {
                uid: "data1".to_string(),
                state: ArrayDeviceState::NORMAL,
            }],
            spares: vec![DeviceMeta {
                uid: "spare1".to_string(),
                state: ArrayDeviceState::NORMAL,
            }],
        };
        let unique_id = 1213;
        let mut array_meta = ArrayMeta::new("array1".to_string(),
                                            devs, "RAID10".to_string(),
                                            "RAID5".to_string(), unique_id);

        let ret = mbr_manager.CreateAbr(array_meta.clone());

        // Then 1: the allocated array index should be 0 and its name be "array1"
        let actual_array_idx = ret.unwrap() as usize;
        assert_eq!(0, actual_array_idx);
        let mbr = &mut mbr_manager.systeminfo;
        assert_eq!(1, mbr.arrayNum);
        assert_eq!("array1".as_bytes(), &mbr.arrayInfo[actual_array_idx].arrayName[0..6]);
        assert_eq!(1, mbr.arrayValidFlag[actual_array_idx]);

        // When 2: we try to allocate a new array with the same devices
        array_meta.arrayName = "somenewarray".to_string();
        let ret = mbr_manager.CreateAbr(array_meta.clone());

        // Then 2: we should see an error saying that the devices are already part of other array
        match ret {
            Ok(_array_idx) => {
                assert!(false);
            }
            Err(e) => {
                assert_eq!(MBR_DEVICE_ALREADY_IN_ARRAY, e);
            }
        }
    }

    #[test]
    fn test_if_CreateAbr_allocates_an_empty_array_index_when_there_are_existing_arrays() {
        let mut dm_config = DeviceManagerConfig::default();
        setup();
        cleanup(&dm_config);

        // Given: Array index 0, 1, 2, 4 have been already allocated,
        let mut mbr_manager = MbrManager::new(Some(dm_config));
        let mbr = &mut mbr_manager.systeminfo;
        mbr.arrayValidFlag[0] = 1;
        mbr.arrayValidFlag[1] = 1;
        mbr.arrayValidFlag[2] = 1;
        mbr.arrayValidFlag[3] = 0;
        mbr.arrayValidFlag[4] = 1;
        mbr.arrayNum = 4;

        let devs = DeviceSet {
            nvm: vec![DeviceMeta {
                uid: "nvm1".to_string(),
                state: ArrayDeviceState::NORMAL
            }
            ],
            data: vec![DeviceMeta {
                uid: "data1".to_string(),
                state: ArrayDeviceState::NORMAL,
            }],
            spares: vec![DeviceMeta {
                uid: "spare1".to_string(),
                state: ArrayDeviceState::NORMAL,
            }],
        };
        let unique_id = 1213;
        let array_meta = ArrayMeta::new("mynewarray".to_string(),
                                            devs, "RAID10".to_string(),
                                            "RAID5".to_string(), unique_id);

        // When: we create ABR for a new array "mynewarray"
        let ret = mbr_manager.CreateAbr(array_meta);

        // Then: The newly allocated array index should be 3 and its name be "mynewarray"
        let mbr = &mut mbr_manager.systeminfo;
        let actual_array_idx = ret.unwrap() as usize;
        assert_eq!(3, actual_array_idx);
        assert_eq!(1, mbr.arrayValidFlag[actual_array_idx]);
        assert_eq!("mynewarray".as_bytes(), &mbr.arrayInfo[actual_array_idx].arrayName[0..10]);
        assert_eq!(5, mbr.arrayNum);
    }
}