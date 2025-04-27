use std::sync::{Arc, Mutex};
use crate::page_manager::DiskPageManager;

/// Type alias for on-disk page identifiers
pub type PageId = u32;

/// Distinguishes leaf vs. internal nodes
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum NodeType {
    Leaf,
    Internal,
}

/// Result of splitting a full node
pub struct SplitResult {
    pub new_page: PageId,
    pub promoted_key: Vec<u8>,
}

/// In-memory representation of a B-Tree node
pub enum Node {
    Leaf {
        page_id: PageId,
        keys: Vec<Vec<u8>>,
        values: Vec<Vec<u8>>,
        next_leaf: Option<PageId>,
    },
    Internal {
        page_id: PageId,
        keys: Vec<Vec<u8>>,
        children: Vec<PageId>,
    },
}

/// Errors raised by BTreeEngine operations
#[derive(Debug)]
pub enum BTreeError {
    Io(std::io::Error),
    Corruption(String),
    NotFound,
    // Extend with SplitFailed, Underflow, etc.
}

/// Core engine driving B-Tree operations on top of a Pager
pub struct BTreeEngine {
    page_manager: Arc<Mutex<DiskPageManager>>,
    order: usize,
    root_page: PageId,
}

impl BTreeEngine {
    /// Create or open a B-Tree with given order (max children per internal node)
    pub fn new(
        page_manager: Arc<Mutex<DiskPageManager>>,
        order: usize,
    ) -> Result<Self, BTreeError> {
        // allocate or load root_page from header
        unimplemented!()
    }

    /// Insert a key/value pair into the tree
    pub fn insert(&mut self, key: &[u8], value: &[u8]) -> Result<(), BTreeError> {
        unimplemented!()
    }

    /// Search for a key, returning its value if found
    pub fn search(&self, key: &[u8]) -> Result<Option<Vec<u8>>, BTreeError> {
        unimplemented!()
    }

    /// Delete a key (and its value) from the tree
    pub fn delete(&mut self, key: &[u8]) -> Result<(), BTreeError> {
        unimplemented!()
    }

    //—— INTERNAL HELPERS —————————————————————————————————————————

    /// Split a full leaf node, returning new page and key to promote
    fn split_leaf(&mut self, page_id: PageId) -> Result<SplitResult, BTreeError> {
        unimplemented!()
    }

    /// Split a full internal node similarly
    fn split_internal(&mut self, page_id: PageId) -> Result<SplitResult, BTreeError> {
        unimplemented!()
    }

    /// Load a node into memory for in-page decoding and manipulation
    fn load_node(&self, page_id: PageId) -> Result<Node, BTreeError> {
        unimplemented!()
    }

    /// Write an in-memory node back to its on-disk page
    fn write_node(&self, node: Node) -> Result<(), BTreeError> {
        unimplemented!()
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::page_manager::DiskPageManager;
    use std::fs;
    use tempfile::tempdir;

    fn setup_btree() -> (BTreeEngine, tempfile::TempDir) {
        let dir = tempdir().unwrap();
        let db_path = dir.path().join("test_btree.db");
        
        let page_size = 4096;
        let page_manager = Arc::new(Mutex::new(
            DiskPageManager::open(&db_path, page_size).unwrap()
        ));
        
        let order = 4; // Small order for testing
        let btree = BTreeEngine::new(page_manager, order).unwrap();
        
        (btree, dir)
    }

    #[test]
    fn test_insert_and_search() {
        let (mut btree, _dir) = setup_btree();
        
        // Insert some key-value pairs
        btree.insert(b"key1", b"value1").unwrap();
        btree.insert(b"key2", b"value2").unwrap();
        btree.insert(b"key3", b"value3").unwrap();
        
        // Search for existing keys
        let result1 = btree.search(b"key1").unwrap();
        let result2 = btree.search(b"key2").unwrap();
        let result3 = btree.search(b"key3").unwrap();
        
        assert_eq!(result1, Some(b"value1".to_vec()));
        assert_eq!(result2, Some(b"value2".to_vec()));
        assert_eq!(result3, Some(b"value3".to_vec()));
        
        // Search for non-existent key
        let result4 = btree.search(b"key4").unwrap();
        assert_eq!(result4, None);
    }

    #[test]
    fn test_update_existing_key() {
        let (mut btree, _dir) = setup_btree();
        
        // Insert a key-value pair
        btree.insert(b"key1", b"value1").unwrap();
        
        // Update the value for the same key
        btree.insert(b"key1", b"updated_value").unwrap();
        
        // Verify the update
        let result = btree.search(b"key1").unwrap();
        assert_eq!(result, Some(b"updated_value".to_vec()));
    }

    #[test]
    fn test_delete() {
        let (mut btree, _dir) = setup_btree();
        
        // Insert some key-value pairs
        btree.insert(b"key1", b"value1").unwrap();
        btree.insert(b"key2", b"value2").unwrap();
        
        // Delete a key
        btree.delete(b"key1").unwrap();
        
        // Verify the key is deleted
        let result1 = btree.search(b"key1").unwrap();
        let result2 = btree.search(b"key2").unwrap();
        
        assert_eq!(result1, None);
        assert_eq!(result2, Some(b"value2".to_vec()));
        
        // Delete a non-existent key (should not error)
        btree.delete(b"key3").unwrap();
    }

    #[test]
    fn test_many_inserts_for_splits() {
        let (mut btree, _dir) = setup_btree();
        
        // Insert enough keys to force node splits
        for i in 0..20 {
            let key = format!("key{:03}", i);
            let value = format!("value{:03}", i);
            btree.insert(key.as_bytes(), value.as_bytes()).unwrap();
        }
        
        // Verify all keys can be found
        for i in 0..20 {
            let key = format!("key{:03}", i);
            let expected_value = format!("value{:03}", i);
            
            let result = btree.search(key.as_bytes()).unwrap();
            assert_eq!(result, Some(expected_value.into_bytes()));
        }
    }

    #[test]
    fn test_persistence() {
        let dir = tempdir().unwrap();
        let db_path = dir.path().join("test_persistence.db");
        
        let page_size = 4096;
        let order = 4;
        
        // Create a B-tree and insert data
        {
            let page_manager = Arc::new(Mutex::new(
                DiskPageManager::open(&db_path, page_size).unwrap()
            ));
            
            let mut btree = BTreeEngine::new(page_manager, order).unwrap();
            
            btree.insert(b"key1", b"value1").unwrap();
            btree.insert(b"key2", b"value2").unwrap();
            
            // Ensure data is flushed to disk
            btree.page_manager.lock().unwrap().sync().unwrap();
        }
        
        // Reopen the B-tree and verify data
        {
            let page_manager = Arc::new(Mutex::new(
                DiskPageManager::open(&db_path, page_size).unwrap()
            ));
            
            let btree = BTreeEngine::new(page_manager, order).unwrap();
            
            let result1 = btree.search(b"key1").unwrap();
            let result2 = btree.search(b"key2").unwrap();
            
            assert_eq!(result1, Some(b"value1".to_vec()));
            assert_eq!(result2, Some(b"value2".to_vec()));
        }
    }
}
