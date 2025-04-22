use std::{fs, io, path};
use std::io::{Read, Seek, Write};
use crate::db_header::DatabaseHeader;

/// DiskEngine is a struct that represents the disk-based storage engine for the database.
/// It handles reading and writing pages to a file on disk.
pub struct DiskEngine {
    file: fs::File,
    page_size: u64,
}

impl DiskEngine {
    /// Creates a new `DiskEngine` instance.
    /// 
    /// It opens the database file at the specified path and initializes the page size.
    pub fn new(path: &path::Path, page_size: u64) -> Result<Self, io::Error> {
        match fs::File::open(path) {
            Ok(file) => {
                let header = DatabaseHeader::new(page_size);
                Ok(DiskEngine{
                    file,
                    page_size,
                })
            },
            Err(e) => Err(e),
        }
    }
    
    /// Reads a page from the database file at the specified offset.
    /// Offset is the number of pages from the start of the file.
    pub fn read_page(&mut self, offset: u64) -> Result<Vec<u8>, io::Error> {
        let mut page_buffer = vec![0; self.page_size as usize];
        self.file.seek(io::SeekFrom::Start(offset))?;
        self.file.read_exact(page_buffer.as_mut_slice())?;
        Ok(page_buffer)
    }
    
    /// Writes a page to the database file at the specified offset.
    /// Offset is the number of pages from the start of the file.
    pub fn write_page(&mut self, offset: u64, data: &[u8]) -> Result<(), io::Error> {
        self.file.seek(io::SeekFrom::Start(offset))?;
        self.file.write_all(data)?;
        Ok(())
    }
    
    /// Sync the file to ensure all data is written to disk.
    pub fn close(&mut self) -> Result<(), io::Error> {
        self.file.sync_all()?;
        self.file.sync_data()?;
        Ok(())
    }
}

/// MemoryEngine is a struct that represents the in-memory storage engine for the database.
/// It handles reading and writing pages to a memory buffer. All data is stored in RAM.
/// 
pub struct MemoryEngine {
    
}

impl MemoryEngine {
    
}