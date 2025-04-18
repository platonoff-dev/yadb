use std::mem::size_of;

type PageId = u64;

#[derive(Debug, Clone, PartialEq)]
struct DatabaseHeader {
    id: PageId,     // 8
    magic: [u8; 4], // 4
    version: u32,   // 4
    page_size: u32, // 4
    page_count: u64,// 8
}

impl DatabaseHeader {
    fn new(page_size: u32) -> DatabaseHeader {
        DatabaseHeader {
            id: 0,
            magic: *b"YADB",
            version: 1,
            page_count: 1,
            page_size,
        }
    }

    fn serialize(&self) -> Result<Vec<u8>, String> {
        let mut buffer = Vec::with_capacity(size_of::<DatabaseHeader>());

        buffer.extend_from_slice(&(self.id as u64).to_le_bytes());
        buffer.extend_from_slice(&self.magic);
        buffer.extend_from_slice(&self.version.to_le_bytes());
        buffer.extend_from_slice(&self.page_size.to_le_bytes());
        buffer.extend_from_slice(&self.page_count.to_le_bytes());

        buffer.resize(size_of::<DatabaseHeader>(), 0);

        Ok(buffer)
    }

    fn deserialize(bytes: &[u8]) -> Result<Self, String> {
        if bytes.len() < size_of::<Self>() {
            return Err("Insufficient data to deserialize DatabaseHeader".to_string());
        }

        Ok(Self {
            id: u64::from_le_bytes((&bytes[0..8]).try_into().unwrap()),
            magic: <[u8; 4]>::try_from(&bytes[8..12]).unwrap(),
            version: u32::from_le_bytes(bytes[12..16].try_into().unwrap()),
            page_size: u32::from_le_bytes(bytes[16..20].try_into().unwrap()),
            page_count: u64::from_le_bytes(bytes[20..28].try_into().unwrap()),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_serialize() {
        let header = DatabaseHeader::new(4096);
        let bytes = header.serialize().unwrap();
        
        // Expected sizes
        assert_eq!(bytes.len(), size_of::<DatabaseHeader>()); // 8 + 4 + 4 + 4 + 8 = 28 bytes
        
        // Check individual fields are serialized correctly
        assert_eq!(&bytes[0..8], &(0_u64).to_le_bytes()); // id
        assert_eq!(&bytes[8..12], b"YADB"); // magic
        assert_eq!(&bytes[12..16], &(1_u32).to_le_bytes()); // version
        assert_eq!(&bytes[16..20], &(4096_u32).to_le_bytes()); // page_size
        assert_eq!(&bytes[20..28], &(1_u64).to_le_bytes()); // page_count
    }

    #[test]
    fn test_deserialize() {
        let mut bytes = Vec::new();
        bytes.extend_from_slice(&(42_u64).to_le_bytes()); // id
        bytes.extend_from_slice(b"YADB"); // magic
        bytes.extend_from_slice(&(2_u32).to_le_bytes()); // version
        bytes.extend_from_slice(&(8192_u32).to_le_bytes()); // page_size
        bytes.extend_from_slice(&(100_u64).to_le_bytes()); // page_count
        bytes.resize(size_of::<DatabaseHeader>(), 0); // Ensure the buffer is the right size

        let header = DatabaseHeader::deserialize(&bytes).unwrap();
        
        assert_eq!(header.id, 42);
        assert_eq!(&header.magic, b"YADB");
        assert_eq!(header.version, 2);
        assert_eq!(header.page_size, 8192);
        assert_eq!(header.page_count, 100);
    }

    #[test]
    fn test_serialize_deserialize_roundtrip() {
        let original = DatabaseHeader {
            id: 123,
            magic: *b"YADB",
            version: 3,
            page_size: 16384,
            page_count: 500,
        };

        let bytes = original.serialize().unwrap();
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
