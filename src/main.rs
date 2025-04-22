#![warn(missing_docs)]

//! YADB - Yet Another Database

/// Database Header structure and serialization/deserialization functions.
pub mod db_header;

/// Engine module for the database, including disk and memory engines.
pub mod engine;
mod pages;
mod btree;

/// Main function for the YADB database engine.
fn main() {
    
}
