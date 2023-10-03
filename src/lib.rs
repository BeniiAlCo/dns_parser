//! DNS Parser/Builder.
//!
//! Provides methods to:
//! - Parse the byte-representation of a DNS packet into Rust types
//! - Build a byte-representation of a DNS packet from Rust types
//!
//! Should be fast, with mininimal (probably not zero?) necessary allocations.

// mod message;
pub mod header;
pub mod question;
pub mod shared;
// mod resource_record;
