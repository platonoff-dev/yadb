#![warn(missing_docs)]

//! YADB - Yet Another Database

/// Database Header structure and serialization/deserialization functions.
pub mod pages;

/// Engine module for the database, including disk and memory engines.
pub mod page_manager;
mod btree;

/// Main function for the YADB database engine.
fn main() {
    
}
