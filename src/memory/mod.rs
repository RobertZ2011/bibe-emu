use crate::Result;

use bibe_instr::Width;

pub mod image;

/// Trait that represents an interface into addressable memory
pub trait Memory {
	fn read(&self, addr: u32, width: Width) -> Result<u32>;
	fn write(&mut self, addr: u32, width: Width, value: u32) -> Result<()>;
}