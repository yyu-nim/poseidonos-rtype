use crate::include::memory;
use crate::include::pos_event_id::PosEventId;

pub struct BlockAlignment {
    start_address: u64,
    head_size: u32,
    tail_size: u32,
    block_count: u32,
    head_position: u32,
}

impl BlockAlignment {

    pub fn new(original_start_address: u64, size: u64) -> BlockAlignment {
        let mut start_address = original_start_address;
        let mut end_address = start_address + size;
        let head_position = memory::GetByteOffsetInBlock(start_address);
        let mut head_size;
        if head_position > 0 {
            head_size = memory::BLOCK_SIZE - head_position as usize;

            if head_size > size as usize {
                head_size = size as usize;
            }

            start_address -= head_position;
        } else {
            head_size = 0;
        }

        let tail_size = memory::GetByteOffsetInBlock(end_address);
        if tail_size != 0 {
            end_address += memory::BLOCK_SIZE as u64 - tail_size;
        }

        let block_count = memory::ChangeByteToBlock(end_address - start_address);
        BlockAlignment {
            start_address,
            head_size: head_size as u32,
            tail_size: tail_size as u32,
            block_count: block_count as u32,
            head_position: head_position as u32,
        }
    }

    pub fn GetBlockCount(&self) -> u32 {
        self.block_count
    }

    pub fn GetHeadBlock(&self) -> Result<u64, PosEventId> {
        Ok(memory::ChangeByteToSector(self.start_address))
    }

    pub fn HasHead(&self) -> bool {
        self.head_size != 0
    }

    pub fn HasTail(&self) -> bool {
        self.tail_size != 0
    }

    pub fn GetTailSize(&self) -> u32 {
        self.tail_size
    }

    pub fn GetHeadSize(&self) -> u32 {
        self.head_size
    }

    pub fn GetHeadPosition(&self) -> u32 {
        self.head_position
    }

    pub fn GetTailBlock(&self) -> Result<u64, PosEventId> {
        return Ok(self.GetHeadBlock()? + self.block_count as u64 - 1);
    }
}