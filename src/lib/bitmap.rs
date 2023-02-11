use std::sync::Mutex;
use bit_vec::BitVec;
use bitvec::{bitarr, bitvec};
use crate::include::address_type::StripeId;
use bitvec::prelude::*;

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
    map: bitvec::prelude::BitVec<u8> /* do not get confused with bit-vec's BitVec */,
    lastSetPosition: u64,
    numBits: u64,
    numBitsSet: u64,
}

impl Default for BitMap {
    fn default() -> Self {
        BitMap {
            map: Default::default(),
            lastSetPosition: 0,
            numBits: 0,
            numBitsSet: 0,
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
        bitmap.map = bitvec![u8, Lsb0; 0; num_bits as usize];
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

        let (_left, right) = self.map.split_at(begin as usize);
        let relative_first_zero = right.first_zero();
        match relative_first_zero {
            None => {
                return self.numBits;
            }
            Some(pos) => {
                let absolute_pos = begin + pos as u64;
                return absolute_pos;
            }
        }
    }

    pub fn IsSetBit(&self, bit_offset: u64) -> bool {
        if let Some(v) = self.map.get(bit_offset as usize) {
            *v
        } else {
            false
        }
    }

    pub fn SetNumBitsSet(&mut self, num_bits: u64) -> bool {
        self.numBitsSet = num_bits;
        true
    }
}

mod tests {
    use bitvec::prelude::*;
    use crate::lib::bitmap::BitMap;

    #[test]
    fn test_bitvec_first_zero_functionalities() {
        // Given: a bitmap of 0000111100, and the last_set_position was at offset 5
        let mut arr = bitvec![u8, Lsb0; 0; 10];
        arr.set(4, true);
        arr.set(5, true);
        arr.set(6, true);
        arr.set(7, true);
        let last_set_position = 5;
        let (_left, right) = arr.split_at(last_set_position); // left: 00001, right: 11100

        // When: first_zero() is called on the right split
        let rel_pos = right.first_zero().unwrap();

        // Then
        assert_eq!(8, rel_pos + last_set_position);
    }

    #[test]
    fn test_BitMap_FindFirstZero_functionality() {
        // Given: a bitmap of 0000111100, and the last_set_position of 5
        let mut bitmap = BitMap::new(10);
        bitmap.SetBit(4);
        bitmap.SetBit(5);
        bitmap.SetBit(6);
        bitmap.SetBit(7);
        let last_set_position = 5;

        // When: FindFirstZero() is called
        let actual = bitmap.FindFirstZero(last_set_position);

        // Then
        assert_eq!(8, actual);
    }

    #[test]
    fn test_BitMap_FindNextZero_functionality() {
        // Given: an initialized bitmap of 00000000
        let mut bitmap = BitMap::new(10);

        // When1: FindNextZero() is called
        let actual = bitmap.FindNextZero();

        // Then1
        assert_eq!(1, actual); // Note: pos-cpp 현 구현상 0 아닌 1 부터 할당을 시작하는 것 같은데, 일단 그대로 두기로 함.

        // When2: FindNextZero() is called again
        let actual = bitmap.FindNextZero();

        // Then2: we should get the same bit offset as before (because we haven't set the bit yet)
        assert_eq!(1, actual);

        // When3: we set the bit and call FindNextZero() again
        bitmap.SetBit(actual);
        let actual = bitmap.FindNextZero();

        // Then3: we should get the next bit offset, i.e., 2
        assert_eq!(2, actual);
    }
}