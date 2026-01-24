//! # Loss32
//! 
//! (Hopefully) safe wrappers for `windows-rs` crate.

pub use access_token_handle::*;
pub use process_handle::*;
pub use sid::*;
pub(self) use aligned_buffer::*;

pub mod access_token_handle;
pub mod process_handle;
pub mod sid;
pub(self) mod aligned_buffer;
