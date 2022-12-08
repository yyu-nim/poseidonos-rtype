use std::sync::{Arc, Mutex};
use log::{debug, error};
use crate::allocator::i_block_allocator::IBlockAllocator;
use crate::allocator_service::allocator_service::AllocatorServiceSingleton;
use crate::bio::ubio;
use crate::bio::ubio::{CallbackClosure, Ubio, UbioDir};
use crate::bio::volume_io::VolumeIo;
use crate::event_scheduler::callback::Callback;
use crate::event_scheduler::event::Event;
use crate::event_scheduler::io_completer::IoCompleter;
use crate::gc::flow_control::flow_control;
use crate::gc::flow_control::flow_control::{FlowControl, FlowControlType};
use crate::generated::bindings::user;
use crate::include::{address_type, array_config};
use crate::include::address_type::{BlkAddr, PhysicalBlkAddr, StripeId, VirtualBlkAddr, VirtualBlks};
use crate::include::backend_event::BackendEvent;
use crate::include::backend_event::BackendEvent::BackendEvent_FrontendIO;
use crate::include::io_error_type::IOErrorType;
use crate::include::memory::{ChangeBlockToSector, ChangeByteToSector, ChangeSectorToByte};
use crate::include::pos_event_id::PosEventId;
use crate::include::pos_event_id::PosEventId::{WRHDLR_FAIL_BY_SYSTEM_STOP, WRHDLR_FAIL_TO_UNLOCK, WRITE_FOR_PARITY_FAILED};
use crate::io::frontend_io::block_map_update_request::BlockMapUpdateRequest;
use crate::io::frontend_io::read_completion_for_partial_write::ReadCompletionForPartialWrite;
use crate::io::frontend_io::write_for_parity::WriteForParity;
use crate::io::general_io::rba_state_manager::RBAStateManagerSingleton;
use crate::io::general_io::translator::Translator;
use crate::io_scheduler::io_dispatcher::IODispatcherSingleton;
use crate::lib::block_alignment::BlockAlignment;
use crate::state::include::state_type::{StateEnum, StateType};
use crate::state::state_manager::StateManagerSingleton;

pub struct WriteSubmission {
    volume_io: VolumeIo,
    flow_control: Arc<Mutex<FlowControl>>,
    block_alignment: BlockAlignment,
    block_count: u32,
    allocated_block_count: u32,
    allocated_virtual_blks: Vec<(VirtualBlks, StripeId)>,
    processed_block_count: u32,
    i_block_allocator: Box<dyn IBlockAllocator>,
    split_volume_io_queue: Vec<VolumeIo>,
}

impl WriteSubmission {
    pub fn new(volume_io: VolumeIo) -> WriteSubmission {
        let io_size = {
            volume_io.ubio.as_ref().unwrap().lock().unwrap()
                .dataBuffer.as_ref().unwrap().size() as u64
        };
        let block_alignment = BlockAlignment::new(
            ChangeSectorToByte(volume_io.sector_rba.unwrap()),
            io_size,
        );
        let block_count = block_alignment.GetBlockCount();
        let allocated_block_count = 0;
        let processed_block_count = 0;
        let array_id = {
            volume_io.ubio.as_ref().unwrap().lock().unwrap().arrayId.clone()
        };

        let i_block_allocator = AllocatorServiceSingleton.GetIBlockAllocator(array_id as u32)
            .expect(format!("BlockAllocator for array {} doesn't exist yet", array_id).as_str());

        WriteSubmission {
            volume_io,
            flow_control: flow_control::get(),
            block_alignment,
            block_count,
            allocated_block_count,
            allocated_virtual_blks: Vec::new(),
            processed_block_count,
            i_block_allocator,
            split_volume_io_queue: Vec::new(),
        }
    }

    fn _ProcessOwnedWrite(&mut self) -> bool {
        self._AllocateFreeWriteBuffer();

        if self.block_count > self.allocated_block_count {
            return false;
        }

        self._PrepareBlockAlignment();
        if self.processed_block_count == 0
            && self.allocated_block_count == 1 {
            self._WriteSingleBlock();
        } else {
            self._WriteMultipleBlocks();
        }

        true
    }

    fn _AllocateFreeWriteBuffer(&mut self) -> Result<(), PosEventId> {
        let volume_id = self.volume_io.vol_id;
        if !self.i_block_allocator.TryRdLock(volume_id.unwrap()) {
            return Ok(());
        }

        let is_wt_enabled = true; // Write-back isn't considered at the moment
        let mut remain_block_count = self.block_count - self.allocated_block_count;
        while remain_block_count > 0 {
            let (virtual_blks, stripe_id) = self.i_block_allocator.AllocateWriteBufferBlks(volume_id.unwrap(), remain_block_count);
            let target_vsa_range: VirtualBlks = virtual_blks;

            if address_type::IsUnMapVsa(&target_vsa_range.start_vsa) {
                let array_id = {
                    self.volume_io.ubio.as_ref().unwrap().lock().unwrap().arrayId.clone() as u32
                };
                let state_control = StateManagerSingleton.lock().unwrap().GetStateControl(array_id);
                if state_control.GetState().ToStateType() == StateType::new(StateEnum::STOP) {
                    let event_id = WRHDLR_FAIL_BY_SYSTEM_STOP;
                    error!("[{}] System Stop incurs write fail", event_id.to_string());
                    if !self.i_block_allocator.Unlock(volume_id.unwrap()) {
                        let event_id = WRHDLR_FAIL_TO_UNLOCK;
                        debug!("[{}] volume_id = {}", event_id.to_string(), volume_id.unwrap());
                        return Err(event_id);
                    }
                    return Err(event_id);
                }
                break;
            }

            if is_wt_enabled {
                let mut start_vsa = target_vsa_range.start_vsa;
                let mut remaining_blks = target_vsa_range.num_blks;
                while remaining_blks > 0 {
                    let mut num_blks = array_config::BLOCKS_PER_CHUNK
                        - (start_vsa.offset as u32 % array_config::BLOCKS_PER_CHUNK);
                    if num_blks > remaining_blks {
                        num_blks = remaining_blks;
                    }
                    let vsa_info = VirtualBlks {
                        start_vsa,
                        num_blks
                    };
                    let info = (vsa_info, stripe_id);
                    self._AddVirtualBlks(info);
                    start_vsa.offset += num_blks as u64;
                    remaining_blks -= num_blks;
                }
            } else {
                let info = (virtual_blks, stripe_id);
                self._AddVirtualBlks(info);
            }
            remain_block_count -= target_vsa_range.num_blks;
        }

        if !self.i_block_allocator.Unlock(volume_id.unwrap()) {
            debug!("[{}] volume_id = {}", WRHDLR_FAIL_TO_UNLOCK.to_string(), volume_id.unwrap());
        }

        Ok(())
    }

    fn _AddVirtualBlks(&mut self, virtual_blks: (VirtualBlks, StripeId)) {
        self.allocated_block_count += virtual_blks.0.num_blks;
        self.allocated_virtual_blks.push( virtual_blks );
    }

    fn _PrepareBlockAlignment(&mut self) {
        if self.block_alignment.HasHead() {
            self._ReadOldHeadBlock();
        }

        if self.block_alignment.HasTail() {
            self._ReadOldTailBlock();
        }
    }

    fn _WriteSingleBlock(&mut self) {
        let virtual_blks_info = self.allocated_virtual_blks.first()
            .expect("allocated virtual blks must have at least one item");
        let virtual_blks_info_cloned = virtual_blks_info.clone();
        self._PrepareSingleBlock(&virtual_blks_info_cloned);
        self._SendVolumeIo(&self.volume_io);
    }

    fn _WriteMultipleBlocks(&mut self) {

        let cloned: Vec<(VirtualBlks, StripeId)> = self.allocated_virtual_blks.iter()
            .map(|b| b.clone()).collect();

        for virtual_blks_info in cloned {
            self._WriteDataAccordingToVsaRange(&virtual_blks_info);
        }
        self._SubmitVolumeIo();
    }

    fn _ReadOldHeadBlock(&mut self) {
        let head_rba = self.block_alignment.GetHeadBlock();
        let vsa = self._PopHeadVsa();
        self._ReadOldBlock(head_rba, vsa, false);
    }

    fn _ReadOldTailBlock(&mut self) {
        if self.processed_block_count == self.block_count {
            return;
        }

        let tail_rba = self.block_alignment.GetTailBlock();
        let vsa = self._PopTailVsa();

        self._ReadOldBlock(tail_rba, vsa, true);
    }

    fn _PopHeadVsa(&mut self) -> (VirtualBlkAddr, StripeId) {
        let (first_vsa_range, stripe_id) = self.allocated_virtual_blks.get_mut(0)
            .expect("Expected to have at least one (VirtualBlks, StripeId), but empty!");
        let mut head_vsa = first_vsa_range.start_vsa;
        first_vsa_range.num_blks -= 1;
        head_vsa.offset += 1;
        let stripe_id_cloned = stripe_id.clone();
        if first_vsa_range.num_blks == 0 {
            self.allocated_virtual_blks.pop();
        }

        return (head_vsa, stripe_id_cloned);
    }

    fn _PopTailVsa(&mut self) -> (VirtualBlkAddr, StripeId) {
        let last_allocated = self.allocated_virtual_blks.last().expect("allocated_virtual_blks must have at least one item");
        let mut last_vsa_range = last_allocated.0;
        let mut tail_vsa = last_vsa_range.start_vsa;
        let stripe_id = last_allocated.1;
        tail_vsa.offset += last_vsa_range.num_blks as u64 - 1;
        last_vsa_range.num_blks -= 1;
        if last_vsa_range.num_blks == 0 {
            let last_idx = self.allocated_virtual_blks.len() - 1;
            self.allocated_virtual_blks.remove(last_idx);
        }

        return (tail_vsa, stripe_id);
    }

    fn _ReadOldBlock(&mut self, rba: BlkAddr, vsa_info: (VirtualBlkAddr, StripeId), is_tail: bool) {
        let vsa = vsa_info.0;
        let user_lsid = vsa_info.1;
        let (alignment_size, alignment_offset) = match is_tail {
            true => (self.block_alignment.GetTailSize(), 0),
            false => (self.block_alignment.GetHeadSize(), self.block_alignment.GetHeadPosition()),
        };

        let mut split = self.volume_io.Split(ChangeByteToSector(alignment_size as u64) as u32,
                                             is_tail);
        let origin = self.volume_io.ubio
            .as_ref().unwrap().clone();
        {
            let mut ubio = split.ubio.as_ref().unwrap().lock().unwrap();
            ubio.origin = Some(origin);
            split.vsa = Some(vsa);
            let callback = BlockMapUpdateRequest::new(&self.volume_io).to_callback(); // TODO: need to check whether "Copy" is okay
            ubio.callback = Some(callback);
            let arrayId = ubio.arrayId.clone();
            let mut new_volume_io = VolumeIo::new(None, ubio::UNITS_PER_BLOCK, arrayId as u32);
            new_volume_io.vol_id = self.volume_io.vol_id;
            let split_bio = split.ubio.as_ref().unwrap().clone();
            let new_volume_io_ubio = new_volume_io.ubio.as_ref().unwrap().clone();
            new_volume_io_ubio.lock().unwrap().origin = Some(split_bio.clone());
            new_volume_io.stripe_id = Some(user_lsid as u64);
            let event = ReadCompletionForPartialWrite::new(&new_volume_io, alignment_size, alignment_offset).to_callback();
            new_volume_io_ubio.lock().unwrap().callback = Some(event);
            let sector_rba = ChangeBlockToSector(rba);
            new_volume_io.sector_rba = Some(sector_rba);
            new_volume_io.vsa = Some(vsa);
            let is_read = true;
            let old_data_translator = Translator::new(
                self.volume_io.vol_id.unwrap(), rba, arrayId, is_read
            );
            let old_lsid_entry = old_data_translator.GetLsidEntry(0);
            new_volume_io.old_lsid_entry = Some(old_lsid_entry);
            self.processed_block_count += 1;
            if old_data_translator.IsMapped() {
                let mut new_volume_io_ubio = new_volume_io_ubio.lock().unwrap();
                new_volume_io_ubio.dir = UbioDir::Read;
                let pba = old_data_translator.GetPba();
                new_volume_io_ubio.lba = pba.lba;
                new_volume_io_ubio.arrayDev = pba.array_dev;
            }
            self.split_volume_io_queue.push(new_volume_io);
        }

    }

    fn _PrepareSingleBlock(&mut self, virtual_blks_info: &(VirtualBlks, StripeId)) {
        let ubio = self.volume_io.ubio.as_ref().unwrap().clone();
        //let callback = ubio.lock().unwrap().callback.unwrap();
        let array_id = {
            ubio.lock().unwrap().arrayId
        };
        let sector_rba = {
            self.volume_io.sector_rba.unwrap()
        };
        let ubio_cloned = ubio.clone();
        let new_volume_io = &mut self.volume_io;
        Self::_SetupVolumeIo(
            array_id,
            sector_rba,
            ubio_cloned,
            new_volume_io,
            virtual_blks_info);
    }

    fn _SendVolumeIo(&self, volume_io: &VolumeIo) {
        let ubio = volume_io.ubio.as_ref().unwrap().clone();
        let is_read = match ubio.lock().unwrap().dir {
            UbioDir::Read => true,
            UbioDir::Write => false,
        };
        // Assumption: isWTEnabled is always true.
        if !is_read {
            let mut write_for_parity = WriteForParity::new(volume_io, true);
            let ret = Callback::Execute(&mut write_for_parity);
            if !ret {
                let event_id = WRITE_FOR_PARITY_FAILED;
                error!("[{}] Failed to copy user data to dram for parity", event_id.to_string());
            }
        }

        IODispatcherSingleton.lock().unwrap().Submit(ubio, false, true);
    }

    fn _WriteDataAccordingToVsaRange(&mut self, virtual_blks_info: &(VirtualBlks, StripeId)) {
        let new_volume_io = self._CreateVolumeIo(virtual_blks_info);
        self.split_volume_io_queue.push( new_volume_io );
    }

    fn _SubmitVolumeIo(&mut self) {
        let volume_io_count = self.split_volume_io_queue.len();
        let ubio = self.volume_io.ubio.as_ref().unwrap().clone();
        //let callback = ubio.callback;
        // callback.SetWaitingCount() // TODO

        while !self.split_volume_io_queue.is_empty() {
            let volume_io = self.split_volume_io_queue.first().unwrap();
            let skip_io_submission = !ubio.lock().unwrap().CheckPbaSet();
            if skip_io_submission {
                let io_completer = IoCompleter {};
                io_completer.CompleteUbioWithoutRecovery(IOErrorType::SUCCESS, true);
            } else {
                self._SendVolumeIo(volume_io);
            }
            self.split_volume_io_queue.pop();
        }
    }

    fn _CreateVolumeIo(&mut self, virtual_blks_info: &(VirtualBlks, StripeId)) -> VolumeIo {
        let vsa_range = virtual_blks_info.0;
        let sectors = ChangeBlockToSector(vsa_range.num_blks as u64);
        let mut split = self.volume_io.Split(sectors as u32, false);

        let array_id = {
            self.volume_io.ubio.as_ref().unwrap().lock().unwrap().arrayId
        };
        let sector_rba = {
            self.volume_io.sector_rba.unwrap()
        };
        let ubio_cloned = {
            self.volume_io.ubio.as_ref().unwrap().clone()
        };
        let new_volume_io = &mut split;

        Self::_SetupVolumeIo(
            array_id,
            sector_rba,
            ubio_cloned,
            &mut split,
            virtual_blks_info);
        split
    }

    fn _SetupVolumeIo(array_id: i32,
                      sector_rba: u64,
                      volume_io_ubio: Arc<Mutex<Ubio>>,
                      new_volume_io: &mut VolumeIo,
                      virtual_blks_info: &(VirtualBlks, StripeId)) {
        let vsa_range = virtual_blks_info.0;
        let user_lsid = virtual_blks_info.1;
        let start_vsa = vsa_range.start_vsa;
        // let array_id = self.volume_io.ubio.unwrap().lock().unwrap().arrayId;
        let translator = Translator::new_with_vsa(
            start_vsa, array_id, user_lsid, None);
        let mut mem = new_volume_io.ubio.as_ref().unwrap().lock().unwrap().GetBuffer().unwrap(); // Not Used By Translator! Why?
        let mut physical_entries = translator.GetPhysicalEntries(mem.buffer.clone(),
                                                                 vsa_range.num_blks);
        assert_eq!(1, physical_entries.len());

        new_volume_io.vsa = Some(start_vsa);
        //let mut pba = physical_entries.first().as_mut().unwrap();
        let pba = physical_entries.get_mut(0).unwrap();
        if sector_rba != new_volume_io.sector_rba.unwrap() {
            // TODO: pos-cpp was using "if (volumeIo != newVolumeIo)"
            //new_volume_io.ubio.unwrap().lock().unwrap().origin = Some(self.volume_io.ubio.unwrap().clone());
            new_volume_io.ubio.as_ref().unwrap().lock().unwrap().origin = Some(volume_io_ubio.clone());
        }
        // newVolumeIo->SetVsa(startVsa); // pos-cpp has a duplicate line
        {
            let mut ubio = new_volume_io.ubio.as_ref().unwrap().lock().unwrap();
            ubio.lba = pba.addr.clone().lba;
            let array_dev = pba.addr.clone().array_dev;
            ubio.arrayDev = array_dev;
            new_volume_io.stripe_id = Some(user_lsid as u64);
            let blockmap_update_request = BlockMapUpdateRequest::new(&new_volume_io); // , callback); => new_volume_io의 ubio에서 꺼내올 수 있음.
            ubio.callback = Some(blockmap_update_request.to_callback());
        }

        let lsid_entry = translator.GetLsidEntry(0);
        new_volume_io.lsid_entry = Some(lsid_entry);

        let sectors_to_increment = ChangeBlockToSector(vsa_range.num_blks as u64);
        pba.IncrementsLbaBy(sectors_to_increment);
    }
}

impl Event for WriteSubmission {
    fn GetEventType(&self) -> BackendEvent {
        BackendEvent_FrontendIO // TODO: need to check if it's right (vs. Unknown type)
    }

    fn Execute(&mut self) -> bool {
        let flow_control = self.flow_control.lock().unwrap();
        let token = flow_control.GetToken(FlowControlType::USER, self.block_count as i32);
        if token <= 0 {
            return false;
        }
        let start_rba = self.block_alignment.GetHeadBlock();
        let volume_id = self.volume_io.vol_id;
        let ownership_acquired = RBAStateManagerSingleton.BulkAcquireOwnership(volume_id.unwrap(), start_rba, self.block_count);
        if !ownership_acquired {
            if token > 0 {
                flow_control.ReturnToken(FlowControlType::USER, token);
            }
            return false;
        }
        std::mem::drop(flow_control); // flow_control로 인해 self가 immutable borrow가 되어 self._ProcessOwnedWrite() 컴파일이 안되므로, 강제로 drop
        let done = self._ProcessOwnedWrite();
        if !done {
            RBAStateManagerSingleton.BulkReleaseOwnership(volume_id.unwrap(), start_rba, self.block_count);
        } else {
            //let _array_id = self.volume_io.ubio.unwrap().lock().unwrap().arrayId;
        }
        done
    }
}