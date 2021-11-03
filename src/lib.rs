// inside lib.rs, only the following line should be in here
pub mod entrypoint;
pub mod instruction;
pub mod error;
pub mod processor;

#[cfg(not(feature = "no-entrypoint"))]
pub mod entrypoint;
