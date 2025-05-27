//! ZKVM-specific helper functions for zkemail
//!
//! This crate provides ZKVM-compatible utilities for email processing,
//! key format conversion, and data generation.

pub mod dkim;
pub mod generator;
pub mod io;

// Re-export main APIs
pub use dkim::*;
pub use generator::*;
pub use io::*;
