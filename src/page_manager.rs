use std::fs::File;
use std::os::unix::fs::FileExt;
use std::path::Path;
use std::io::{Read, Write};

use crate::pages::{DatabaseHeader, SerializerError};

/// On-disk page identifier
pub type PageId = u32;

/// Error type for page manager
#[derive(Debug)]
pub enum PageManagerError {
    /// Error when page format is invalid
    BadPageFormat(SerializerError),
    /// Error when IO operation fails
    IoError(std::io::Error),
}

impl std::fmt::Display for PageManagerError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl std::error::Error for PageManagerError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        None
    }
}

impl From<SerializerError> for PageManagerError {
    fn from(error: SerializerError) -> Self {
        PageManagerError::BadPageFormat(error)
    }
}

impl From<std::io::Error> for PageManagerError {
    fn from(error: std::io::Error) -> Self {
        PageManagerError::IoError(error)
    }
}

/// Manages raw pages within the database file
pub struct DiskPageManager {
    file: File,
    page_size: u64,
    header: DatabaseHeader,
    // freelist: Vec<PageId>,
    // TODO: add buffer pool or cache for performance
}

impl DiskPageManager {
    /// Open or create a database file at `path`, setting the page size.
    pub fn open<P: AsRef<Path>>(path: P, page_size: u64) -> Result<Self, PageManagerError> {
        if !path.as_ref().exists() {
            let mut file = File::create(path.as_ref())?;
            let header = DatabaseHeader::new(page_size);
            file.write_all(&header.serialize())?;
            Ok(Self {
                file,
                page_size,
                header,
            })
        } else {
            let mut file = File::open(path.as_ref())?;
            let mut buf = vec![0u8; page_size as usize];
            file.read_exact(&mut buf)?;
            let header = DatabaseHeader::deserialize(&buf)?;
            Ok(Self {
                file,
                page_size: header.page_size,
                header,
            })
        }
    }

    /// Read the page `page_id` into `buf` (must be at least page_size bytes).
    pub fn read_page(&mut self, page_id: PageId, buf: &mut [u8]) -> Result<(), PageManagerError> {
        self.file.read_at(buf, page_id as u64 * self.page_size)?;
        Ok(())
    }

    /// Write `buf` (exactly page_size bytes) to page `page_id`.
    pub fn write_page(&mut self, page_id: PageId, buf: &[u8]) -> Result<(), PageManagerError> {
        self.file.write_at(buf, page_id as u64 * self.page_size)?;
        Ok(())
    }

    /// Allocate a new page (reuse from freelist or extend file).
    pub fn alloc_page(&mut self) -> Result<PageId, PageManagerError> {
        unimplemented!()
    }

    /// Free the given page, adding it to the freelist.
    pub fn free_page(&mut self, page_id: PageId) -> Result<(), PageManagerError> {
        unimplemented!()
    }

    /// Persist the freelist back to disk if using on-disk freelist pages.
    pub fn save_freelist(&mut self) -> Result<(), PageManagerError> {
        unimplemented!()
    }

    /// Flush all pending writes to disk.
    pub fn sync(&mut self) -> Result<(), PageManagerError> {
        self.file.sync_data()?;
        Ok(())
    }

    /// Close the page manager and its underlying file.
    pub fn close(&mut self) -> Result<(), PageManagerError> {
        self.sync()?;
        Ok(())
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::tempdir;

    #[test]
    fn test_open_new_file() {
        let dir = tempdir().unwrap();
        let db_path = dir.path().join("test_open.db");
        
        let page_size = 4096;
        let manager = DiskPageManager::open(&db_path, page_size).unwrap();
        
        assert_eq!(manager.page_size, page_size);
        assert!(db_path.exists());
        
        // Clean up
        drop(manager);
        fs::remove_file(db_path).unwrap();
    }
    
    #[test]
    fn test_write_and_read_page() {
        let dir = tempdir().unwrap();
        let db_path = dir.path().join("test_write_read.db");
        
        let page_size = 4096;
        let mut manager = DiskPageManager::open(&db_path, page_size).unwrap();
        
        // Allocate a new page
        let page_id = manager.alloc_page().unwrap();
        
        // Create test data
        let mut write_buf = vec![0u8; page_size as usize];
        for i in 0..page_size {
            write_buf[i as usize] = (i % 256) as u8;
        }
        
        // Write the page
        manager.write_page(page_id, &write_buf).unwrap();
        
        // Read the page back
        let mut read_buf = vec![0u8; page_size as usize];
        manager.read_page(page_id, &mut read_buf).unwrap();
        
        // Verify the data
        assert_eq!(write_buf, read_buf);
        
        // Clean up
        drop(manager);
        fs::remove_file(db_path).unwrap();
    }
    
    #[test]
    fn test_alloc_and_free_page() {
        let dir = tempdir().unwrap();
        let db_path = dir.path().join("test_alloc_free.db");
        
        let page_size = 4096;
        let mut manager = DiskPageManager::open(&db_path, page_size).unwrap();
        
        // Allocate some pages
        let page_id1 = manager.alloc_page().unwrap();
        let page_id2 = manager.alloc_page().unwrap();
        let page_id3 = manager.alloc_page().unwrap();
        
        // Free the middle page
        manager.free_page(page_id2).unwrap();
        
        // Allocate a new page - should reuse the freed page
        let page_id4 = manager.alloc_page().unwrap();
        
        // The new page should have the same ID as the freed page
        assert_eq!(page_id2, page_id4);
        
        // Clean up
        drop(manager);
        fs::remove_file(db_path).unwrap();
    }
    
    #[test]
    fn test_sync_and_persistence() {
        let dir = tempdir().unwrap();
        let db_path = dir.path().join("test_sync.db");
        
        let page_size = 4096;
        let mut manager = DiskPageManager::open(&db_path, page_size).unwrap();
        
        // Allocate and write to a page
        let page_id = manager.alloc_page().unwrap();
        let write_buf = vec![42u8; page_size as usize];
        manager.write_page(page_id, &write_buf).unwrap();
        
        // Sync to ensure data is written to disk
        manager.sync().unwrap();
        
        // Close the manager
        manager.close().unwrap();
        
        // Reopen the file
        let mut manager = DiskPageManager::open(&db_path, page_size).unwrap();
        
        // Read the page back
        let mut read_buf = vec![0u8; page_size as usize];
        manager.read_page(page_id, &mut read_buf).unwrap();
        
        // Verify the data persisted
        assert_eq!(write_buf, read_buf);
        
        // Clean up
        drop(manager);
        fs::remove_file(db_path).unwrap();
    }
    
    #[test]
    fn test_save_freelist() {
        let dir = tempdir().unwrap();
        let db_path = dir.path().join("test_freelist.db");
        
        let page_size = 4096;
        let mut manager = DiskPageManager::open(&db_path, page_size).unwrap();
        
        // Allocate some pages
        let page_ids: Vec<PageId> = (0..5).map(|_| manager.alloc_page().unwrap()).collect();
        
        // Free some pages
        manager.free_page(page_ids[1]).unwrap();
        manager.free_page(page_ids[3]).unwrap();
        
        // Save the freelist
        manager.save_freelist().unwrap();
        
        // Close and reopen
        manager.close().unwrap();
        let mut manager = DiskPageManager::open(&db_path, page_size).unwrap();
        
        // Allocate new pages - should reuse freed pages
        let new_page_id1 = manager.alloc_page().unwrap();
        let new_page_id2 = manager.alloc_page().unwrap();
        
        // Should match the previously freed pages
        assert!(page_ids.contains(&new_page_id1));
        assert!(page_ids.contains(&new_page_id2));
        
        // Clean up
        drop(manager);
        fs::remove_file(db_path).unwrap();
    }
}

