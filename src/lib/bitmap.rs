use std::sync::Mutex;
use bit_vec::BitVec;
use crate::include::address_type::StripeId;

const BITMAP_ENTRY_BITS: usize = 64;
const BITMAP_ENTRY_SIZE: usize = 8;

pub struct BitMapMutex {
    bitMap: Mutex<BitMap>,
}

impl Default for BitMapMutex {
    fn default() -> Self {
        BitMapMutex {
            bitMap: Mutex::new(Default::default()),
        }
    }
}

pub struct BitMap {
    map: BitVec,
    lastSetPosition: u64,
    numBits: u64,
    numBitsSet: u64,
    numEntry: u64,
}

impl Default for BitMap {
    fn default() -> Self {
        BitMap {
            map: Default::default(),
            lastSetPosition: 0,
            numBits: 0,
            numBitsSet: 0,
            numEntry: 0
        }
    }
}

impl BitMapMutex {
    pub fn new(num_bits: u64) -> BitMapMutex {
        let mut bitmap_mutex = BitMapMutex::default();
        bitmap_mutex.bitMap = Mutex::new(BitMap::new(num_bits));
        bitmap_mutex
    }

    pub fn SetNextZeroBit(&self) -> u64 {
        let mut bitmap = self.bitMap.lock().unwrap();
        let bit = bitmap.FindNextZero();
        bitmap.SetBit(bit);
        bit
    }

    pub fn IsValidBit(&self, stripe_id: StripeId) -> bool {
        let bitmap = self.bitMap.lock().unwrap();
        bitmap.IsValidBit(stripe_id as u64)
    }

    pub fn ClearBit(&self, bit: u64) -> bool {
        let mut bitmap = self.bitMap.lock().unwrap();
        bitmap.ClearBit(bit)
    }

    pub fn SetNumBitsSet(&self, num_bits: u64) {
        let mut bitmap = self.bitMap.lock().unwrap();
        bitmap.SetNumBitsSet(num_bits);
    }
}

impl BitMap {
    pub fn new(num_bits: u64) -> BitMap {
        let mut bitmap = BitMap::default();
        bitmap.numBits = num_bits;
        bitmap.map = BitVec::from_elem(num_bits as usize, false);
        bitmap
    }

    pub fn FindNextZero(&self) -> u64 {
        let pos_to_begin = (self.lastSetPosition + 1) % self.numBits;
        let found_pos = self.FindFirstZero(pos_to_begin);
        if found_pos == self.numBits {
            if self.numBitsSet >= self.numBits {
                self.numBits
            } else {
                self.FindFirstZero(0)
            }
        } else {
            found_pos
        }
    }

    pub fn IsValidBit(&self, bit_offset: u64) -> bool {
        bit_offset < self.numBits
    }

    pub fn SetBit(&mut self, bit_offset: u64) -> bool {
        if !self.IsValidBit(bit_offset) {
            return false;
        }

        if self.IsSetBit(bit_offset) {
            return true;
        }

        //let row = bit_offset / BITMAP_ENTRY_BITS;
        //let col = bit_offset % BITMAP_ENTRY_BITS; // pos-cpp way
        self.map.set(bit_offset as usize, true); // pos-rtype way
        self.numBitsSet += 1;
        self.lastSetPosition = bit_offset;
        true
    }

    pub fn ClearBit(&mut self, bit_offset: u64) -> bool {
        if !self.IsValidBit(bit_offset) {
            return false;
        }

        if !self.IsSetBit(bit_offset) {
            return true;
        }

        self.map.set(bit_offset as usize, false);
        self.numBitsSet -= 1;
        return true;
    }

    pub fn FindFirstZero(&self, begin: u64) -> u64 {
        if !self.IsValidBit(begin) {
            return self.numBits;
        }

        // TODO: find a quick way something similar to "ffzl" in pos-cpp
        let mut offset = begin;
        loop {
            //if offset >= self.numEntry { // pos-cpp
            if offset >= self.numBits {   // pos-rtype
                return self.numBits;
            }
            let the_value = self.map.get(offset as usize);
            if the_value.is_none() {
                break;
            }
            let the_value_inner = the_value.unwrap();
            if !the_value_inner {
                // found the first zero
                break;
            } else {
                offset += 1;
            }
        }

        if self.IsValidBit(offset) {
            return offset;
        } else {
            return self.numBits;
        }
    }

    pub fn IsSetBit(&self, bit_offset: u64) -> bool {
        if let Some(v) = self.map.get(bit_offset as usize) {
            v
        } else {
            false
        }
    }

    pub fn SetNumBitsSet(&mut self, num_bits: u64) -> bool {
        self.numBitsSet = num_bits;
        true
    }
}