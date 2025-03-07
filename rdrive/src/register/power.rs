use alloc::boxed::Box;
use core::error::Error;

pub use rdif_power::*;

pub type OnProbeFdt = fn(node: super::FdtInfo<'_>) -> Result<Hardware, Box<dyn Error>>;
