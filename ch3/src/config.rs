//! Constants used in rCore
#![allow(missing_docs)]

pub const USER_STACK_SIZE: usize = 4096;
pub const KERNEL_STACK_SIZE: usize = 4096 * 2;
pub const APP_BASE_ADDRESS: usize = 0x80400000;
pub const APP_SIZE_LIMIT: usize = 0x2000;
pub const MAX_APP_NUM: usize = 5;

pub use crate::board::CLOCK_FREQ;