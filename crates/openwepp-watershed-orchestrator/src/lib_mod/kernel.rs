//! Kernel module for watershed orchestration.
//!
//! Production implementation is intentionally split into boundary-oriented chunks under
//! `kernel/{constants,types,helpers,routing,diagnostics,validation}.rs` and reassembled
//! in `kernel_core.rs`.
pub(crate) mod kernel_core;

pub use kernel_core::*;
