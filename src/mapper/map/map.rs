use log::error;

struct Mpage {
    pub mpage_num: u64,
    pub data: Vec<u8>,
}

pub struct Map {
    mpage_arr: Vec<Mpage>,
    page_size: u64,
    num_pages: u64,
}

impl Map {
    pub fn new() -> Self {
        Self {
            mpage_arr: Vec::new(),
            page_size: 0,
            num_pages: 0,
        }
    }

    pub fn Init(&mut self, num_mpages: u64, mpage_size: u64) {
        let mut mPageArr = Vec::new();
        for mpage in 0..num_mpages {
            mPageArr.push(Mpage {
                mpage_num: mpage,
                data: Vec::new(),
            })
        }

        self.mpage_arr = mPageArr;
        self.page_size = mpage_size;
        self.num_pages = num_mpages;
    }

    pub fn AllocateMpage(&mut self, page_num: u64) -> Option<&mut Vec<u8>> {
        assert!(page_num < self.num_pages);
        if self.mpage_arr[page_num as usize].data.len() != 0 {
            error!("mpage exists but tried to allocate, pageNr:{}", page_num);
            return None;
        }

        let new_mpage = vec![0xF; self.page_size as usize];
        self.mpage_arr[page_num as usize].data = new_mpage;

        Some(&mut self.mpage_arr[page_num as usize].data)
    }

    pub fn GetMpage(&mut self, page_num: u64) -> Option<&mut Vec<u8>> {
        return if page_num > self.num_pages {
            None
        } else {
            Some(&mut self.mpage_arr[page_num as usize].data)
        };
    }

    pub fn GetSize(&self) -> u64 {
        self.page_size
    }

    pub fn GetNumMpages(&self) -> u64 {
        self.num_pages
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_allocating_new_mpage() {
        let mut map = Map::new();
        map.Init(10, 4032);
        assert!(map.AllocateMpage(3).is_some());
        assert!(map.AllocateMpage(3).is_none()); // cannot allocate again

        let mpage = map.GetMpage(3);
        assert!(mpage.is_some());
    }

    #[test]
    fn test_updating_mpage() {
        let mut map = Map::new();
        map.Init(10, 4032);
        let mpage = map.AllocateMpage(2).unwrap();

        // Update mpage
        for index in 0..4032 {
            mpage[index] = 0xA;
        }

        let mpage = map.GetMpage(2).unwrap();
        for index in 0..4032 {
            assert_eq!(mpage[index], 0xA);
        }
    }
}
