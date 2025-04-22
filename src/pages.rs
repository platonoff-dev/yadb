//! This module defines the structure of a page in the database.
//! We should support multiple page types
//! - Metadata page - contains metadata about the file and the database, always the first page.
//! 



/// Page structure
pub struct Page {
    /// Page ID
    /// ❓Do we really need this? Theoretically, we can calculate it from the position in byte array.
    /// 
    pub id: u64,
    
    /// Number representing the type of the page. As we should support multiple page types, we need to store this information.
    pub page_type: u8,
}

impl Page {}