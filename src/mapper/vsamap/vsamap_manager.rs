
use std::collections::HashMap;
use std::sync::atomic::{AtomicBool, AtomicI32, Ordering};

use log::{error, warn};

use crate::bio::ubio::MAX_PROCESSABLE_BLOCK_COUNT;
use crate::include::address_type::{BlkAddr, VirtualBlks, VirtualBlkAddr, UNMAP_VSA};
use crate::include::memory::{DivideUp, BLOCK_SIZE};
use crate::include::pos_event_id::PosEventId;
use crate::mapper::address::mapper_address_info::MapperAddressInfo;
use crate::mapper::include::mapper_const::*;
use crate::mapper::include::mpage_info::VsaArray;
use crate::volume::volume_base::MAX_VOLUME_COUNT;
use super::vsamap_content::VSAMapContent;

pub struct VSAMapManager {
    addr_info: MapperAddressInfo,
    vsa_maps: HashMap<i32, VSAMapContent>,
    _map_flush_state: [AtomicMapFlushState; MAX_VOLUME_COUNT],
    _map_load_state: [AtomicMapLoadState; MAX_VOLUME_COUNT],   
    is_vsa_map_accessable: [AtomicBool; MAX_VOLUME_COUNT],
    _is_vsa_map_internal_accessable: [AtomicBool; MAX_VOLUME_COUNT],
    _num_write_issued_count: AtomicI32,
    _num_load_issued_count: AtomicI32,
    // TODO EventScheduler* eventScheduler;
    // TODO TelemetryPublisher* tp;
}

impl VSAMapManager {
    pub fn new(addr_info: MapperAddressInfo) -> Self {
        const initial_flush_state: AtomicMapFlushState = AtomicMapFlushState::new(MapFlushState::FLUSH_DONE);
        const initial_load_state: AtomicMapLoadState = AtomicMapLoadState::new(MapLoadState::LOAD_DONE);
        const initial_accessable: AtomicBool = AtomicBool::new(false);
        const initial_internal_accessible: AtomicBool = AtomicBool::new(false);
        Self {
            addr_info,
            vsa_maps: HashMap::new(),
            _map_flush_state: [initial_flush_state; MAX_VOLUME_COUNT],
            _map_load_state: [initial_load_state; MAX_VOLUME_COUNT],
            is_vsa_map_accessable: [initial_accessable; MAX_VOLUME_COUNT],
            _is_vsa_map_internal_accessable: [initial_internal_accessible; MAX_VOLUME_COUNT],
            _num_write_issued_count: AtomicI32::new(0),
            _num_load_issued_count: AtomicI32::new(0),
        }
    }

    pub fn Init(&mut self) {
        for volume_id in 0 .. MAX_VOLUME_COUNT {
            self.is_vsa_map_accessable[volume_id].store(false, Ordering::SeqCst);
            self._map_flush_state[volume_id].set(MapFlushState::FLUSH_DONE);
            self._map_load_state[volume_id].set(MapLoadState::LOAD_DONE);
        }
        self._num_write_issued_count.store(0, Ordering::SeqCst);
        self._num_load_issued_count.store(0, Ordering::SeqCst);
    }

    pub fn Dispose(&mut self) {

    }

    pub fn CreateVsaMapContent(&mut self, vm: Option<VSAMapContent>, volume_id: i32, vol_size_byte: u64, del_vol: bool) -> Result<(), PosEventId> {
        assert!(self.vsa_maps.get(&volume_id).is_none() == true);

        if vm.is_some() {
            self.vsa_maps.insert(volume_id, vm.unwrap());
        } else {
            self.vsa_maps.insert(volume_id, VSAMapContent::new(volume_id, &self.addr_info));
        }

        let blk_cnt = DivideUp(vol_size_byte, BLOCK_SIZE as u64);
        loop {
            let map = self.vsa_maps.get_mut(&volume_id).unwrap();
            match map.InMemoryInit(volume_id as u64, blk_cnt, self.addr_info.mpage_size as u64) {
                Ok(()) => {}
                Err(e) => {
                    error!("[Mapper VSAMap] Vsa map In-memory Data Prepare Failed, volume:{} arrayId:{} err:{}",
                        volume_id, self.addr_info.array_id, e.to_string());
                    break;
                }
            }

            match map.OpenMapFile() {
                Ok(()) => {
                    return Ok(());
                },
                Err(e) => {
                    if del_vol == false && e == PosEventId::NEED_TO_INITIAL_STORE {
                        todo!("flush initialized vsamap");
                    } else {
                        error!("[Mapper VSAMap] failed to create Vsa map File, volume:{}, arrayId:{}",
                            volume_id, self.addr_info.array_id);
                        break;
                    }
                }
            }
        }

        self.vsa_maps.remove(&volume_id);
        Err(PosEventId::MAPPER_FAILED)
    }

    fn GetVSAs(&mut self, volume_id: i32, start_rba: BlkAddr, num_blks: u32) -> Result<VsaArray, PosEventId> {
        if self.is_vsa_map_accessable[volume_id as usize].load(Ordering::SeqCst) == false {
            warn!("[Mapper VSAMap] VolumeId:{}, arrayId:{} is not accessible, maybe unmounted",
                volume_id, self.addr_info.array_id);
            
            return Err(PosEventId::VSAMAP_NOT_ACCESSIBLE);
        }

        let map = self.vsa_maps.get_mut(&volume_id).unwrap();
        let mut vsa_array: VsaArray = [UNMAP_VSA; MAX_PROCESSABLE_BLOCK_COUNT as usize];
        for blk_idx in 0..num_blks as u64 {
            let target_rba = start_rba + blk_idx;
            vsa_array[blk_idx as usize] = map.GetEntry(target_rba);
        }

        Ok(vsa_array)
    }

    fn SetVSAs(&mut self, volume_id: i32, start_rba: BlkAddr, virtual_blks: VirtualBlks) -> Result<(), PosEventId> {
        if self.is_vsa_map_accessable[volume_id as usize].load(Ordering::SeqCst) == false {
            warn!("[Mapper VSAMap] VolumeId:{} is not accessible, maybe unmounted", volume_id);
            return Err(PosEventId::VSAMAP_NOT_ACCESSIBLE);
        }
        self._UpdateVsaMap(volume_id, start_rba, virtual_blks)
    }

    fn _UpdateVsaMap(&mut self, volume_id: i32, start_rba: BlkAddr, virtual_blks: VirtualBlks) -> Result<(), PosEventId> {
        let map = self.vsa_maps.get_mut(&volume_id).unwrap();

        for blk_idx in 0..virtual_blks.num_blks as u64 {
            let target_vsa = VirtualBlkAddr {
                stripe_id: virtual_blks.start_vsa.stripe_id,
                offset: virtual_blks.start_vsa.offset + blk_idx
            };
            let target_rba = start_rba + blk_idx;

            match map.SetEntry(target_rba, target_vsa) {
                Ok(()) => {},
                Err(e) => {
                    error!("[Mapper VSAMap] failed to update VSAMap Info, volumeId:{}  targetRba:{}  targetVsa.sid:{}  targetVsa.offset:{}",
                        volume_id, target_rba, target_vsa.stripe_id, target_vsa.offset);
                    
                    return Err(e);
                }
            }
        }

        Ok(())
    }

    fn IsVsaMapAccessible(&self, volume_id: i32) -> bool {
        self.is_vsa_map_accessable[volume_id as usize].load(Ordering::SeqCst)
    }

    fn EnableVsaMapAccess(&mut self, volume_id: i32) {
        self.is_vsa_map_accessable[volume_id as usize].store(true, Ordering::SeqCst);
    }

    fn DisableVsaMapAccess(&mut self, volume_id: i32) {
        self.is_vsa_map_accessable[volume_id as usize].store(false, Ordering::SeqCst);
    }

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simple_use() {
        let addr_info = MapperAddressInfo {
            max_vsid: 1024,
            blks_per_stripe: 256,
            num_wb_stripes: 2048,
            mpage_size: 4032,
            array_id: 0,
        };
        let mut vsamap_manager = VSAMapManager::new(addr_info);

        // Create VSA Map to use
        let volume_id: i32= 10;
        let volume_size_byte = 4 * 1024 * 1024; // 4KB blocks * 1024. MAX RBA is 1023
        let create_result = vsamap_manager.CreateVsaMapContent(None, volume_id, volume_size_byte, false);
        assert_eq!(create_result.is_ok(), true);

        // Should enable accessible
        vsamap_manager.EnableVsaMapAccess(volume_id);

        // Test vsa range is like this..
        let start_rba = 0;
        let virtual_blks = VirtualBlks {
            start_vsa: VirtualBlkAddr { stripe_id: 0, offset: 0 },
            num_blks: 10,
        };

        // Try to get VSAs, and result should be UNMAP VSAs
        let get_result = vsamap_manager.GetVSAs(volume_id, start_rba, 10);
        assert_eq!(get_result.is_ok(), true);
        let vsa_array = get_result.unwrap();
        for blk_idx in 0 .. 10 {
            assert_eq!(vsa_array[blk_idx as usize], UNMAP_VSA);
        }

        // Set VSAs as desired
        let start_rba = 0;
        let virtual_blks = VirtualBlks {
            start_vsa: VirtualBlkAddr { stripe_id: 0, offset: 0 },
            num_blks: 10,
        };
        let set_result = vsamap_manager.SetVSAs(volume_id, start_rba, virtual_blks);
        assert_eq!(set_result.is_ok(), true);

        // Get VSAs and verify 
        let get_result = vsamap_manager.GetVSAs(volume_id, start_rba, 10);
        assert_eq!(get_result.is_ok(), true);
        
        let vsa_array = get_result.unwrap();
        for blk_idx in 0 .. 10 {
            assert_eq!(vsa_array[blk_idx as usize].stripe_id, virtual_blks.start_vsa.stripe_id);
            assert_eq!(vsa_array[blk_idx as usize].offset, blk_idx);
        }
    }

    #[test]
    fn test_inaccessible_map() {
        let addr_info = MapperAddressInfo {
            max_vsid: 1024,
            blks_per_stripe: 256,
            num_wb_stripes: 2048,
            mpage_size: 4032,
            array_id: 0,
        };
        let mut vsamap_manager = VSAMapManager::new(addr_info);

        // Create VSA Map to use
        let volume_id: i32= 0;
        let volume_size_byte = 4 * 1024 * 1024; // 4KB blocks * 1024. MAX RBA is 1023
        let create_result = vsamap_manager.CreateVsaMapContent(None, volume_id, volume_size_byte, false);
        assert_eq!(create_result.is_ok(), true);

        // When access is disabled
        vsamap_manager.DisableVsaMapAccess(volume_id);

        let start_rba = 0;
        let virtual_blks = VirtualBlks {
            start_vsa: VirtualBlkAddr { stripe_id: 0, offset: 0 },
            num_blks: 10,
        };

        assert_eq!(vsamap_manager.SetVSAs(volume_id, start_rba, virtual_blks).is_err(), true);
        assert_eq!(vsamap_manager.GetVSAs(volume_id, start_rba, 10).is_err(), true);
    }
}