use std::fs::File;
use std::os::unix::fs::FileExt;

const PAGE_SIZE: usize = 4096;

struct PageManager {
    page_size: usize
}

impl PageManager {
    pub fn new(page_size: usize) -> PageManager {
        PageManager { page_size }
    }

    pub fn allocate_page(size: u16) -> u64 {
        0
    }

    pub fn free_page(page: u64) {}

    pub fn read_page(&self, path: String, page: u64) -> Result<Vec<u8>, Err> {
        let file = File::open(path)?;

        let buf: &mut [u8] = &mut [];
        let _ = file.read_at(buf, page * PAGE_SIZE as u64)?;

        Ok(buf.to_vec())
    }

    pub fn write_page(page: u64, buf: &[u8]) -> Result<(), Err> {

    }
}