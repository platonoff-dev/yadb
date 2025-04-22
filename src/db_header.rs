use std::mem::size_of;


/// Database header structure.
/// This structure is used to store metadata about the database file.
/// It will be stored on the first page of the database file. As header actually uses less bytes
/// than page size, all other bytes will be filled with zeros.
/// So minimal DB size is 4096 bytes.
///
/// Definitely will be extended in the future.
#[derive(Debug, Clone, PartialEq)]
pub struct DatabaseHeader {
    /// Magic number to identify the file format
    pub magic: [u8; 4],
    
    /// Version of the database format. If this changes, the file format is incompatible.
    pub version: u32,   
    
    /// Size of each page in bytes. Might be too much for us but why not.
    /// Typical it is 4096 or 8192 bytes. but might be more.
    /// It is good to align page size with the filesystem block size or memory page size.
    /// 
    /// 📊 Research required. Might be good for pos like "Page size and performance"
    pub page_size: u64, 
    
    /// Total number of pages in the database.
    /// This is the number of pages that have been allocated.
    pub page_count: u64,
}

impl DatabaseHeader {
    /// Creates a new `DatabaseHeader` with the specified page size for a new database file.
    /// For existing files, read it from file and use `DatabaseHeader::deserialize`.
    pub fn new(page_size: u64) -> DatabaseHeader {
        DatabaseHeader {
            
            magic: *b"YADB", // Magic number for YADB
            version: 1,
            page_size,
            page_count: 0,
        }
    }
    
    /// Serializes the `DatabaseHeader` into a byte array.
    pub fn serialize(&self) -> Vec<u8> {
        let mut buffer = Vec::with_capacity(size_of::<DatabaseHeader>());

        buffer.extend_from_slice(&self.magic);
        buffer.extend_from_slice(&self.version.to_le_bytes());
        buffer.extend_from_slice(&self.page_size.to_le_bytes());
        buffer.extend_from_slice(&self.page_count.to_le_bytes());

        buffer.resize(size_of::<DatabaseHeader>(), 0);

        buffer
    }
    
    /// Deserializes a byte array into a `DatabaseHeader`.
    pub fn deserialize(bytes: &[u8]) -> Result<Self, String> {
        if bytes.len() < size_of::<Self>() {
            return Err("Insufficient data to deserialize DatabaseHeader".to_string());
        }

        Ok(Self {
            magic: <[u8; 4]>::try_from(&bytes[8..12]).unwrap(),
            version: u32::from_le_bytes(bytes[12..16].try_into().unwrap()),
            page_size: u64::from_le_bytes(bytes[16..24].try_into().unwrap()),
            page_count: u64::from_le_bytes(bytes[24..32].try_into().unwrap()),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_serialize() {
        let header = DatabaseHeader::new(4096);
        let bytes = header.serialize();
        
        // Expected sizes
        assert_eq!(bytes.len(), size_of::<DatabaseHeader>());
        
        // Check individual fields are serialized correctly
        assert_eq!(&bytes[0..8], &(0_u64).to_le_bytes()); // id
        assert_eq!(&bytes[8..12], b"YADB"); // magic
        assert_eq!(&bytes[12..16], &(1_u32).to_le_bytes()); // version
        assert_eq!(&bytes[16..24], &(4096_u64).to_le_bytes()); // page_size
        assert_eq!(&bytes[24..32], &(1_u64).to_le_bytes()); // page_count
    }

    #[test]
    fn test_deserialize() {
        let mut bytes = Vec::new();
        bytes.extend_from_slice(&(42_u64).to_le_bytes()); // id
        bytes.extend_from_slice(b"YADB"); // magic
        bytes.extend_from_slice(&(2_u32).to_le_bytes()); // version
        bytes.extend_from_slice(&(8192_u64).to_le_bytes()); // page_size
        bytes.extend_from_slice(&(100_u64).to_le_bytes()); // page_count
        bytes.extend_from_slice(&(0_u64).to_le_bytes()); // first_free_page
        bytes.resize(size_of::<DatabaseHeader>(), 0); // Ensure the buffer is the right size

        let header = DatabaseHeader::deserialize(&bytes).unwrap();
        
        assert_eq!(&header.magic, b"YADB");
        assert_eq!(header.version, 2);
        assert_eq!(header.page_size, 8192);
        assert_eq!(header.page_count, 100);
    }

    #[test]
    fn test_serialize_deserialize_roundtrip() {
        let original = DatabaseHeader {
            magic: *b"YADB",
            version: 3,
            page_size: 16384,
            page_count: 500,
        };

        let bytes = original.serialize();
        let deserialized = DatabaseHeader::deserialize(&bytes).unwrap();
        
        assert_eq!(original, deserialized);
    }

    #[test]
    fn test_deserialize_with_insufficient_data() {
        let bytes = [0u8; 20]; // Not enough bytes for a full header
        let result = DatabaseHeader::deserialize(&bytes);
        assert!(result.is_err());
    }
}
