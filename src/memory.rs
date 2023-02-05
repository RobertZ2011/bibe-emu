use crate::state::Result;

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Width {
	Byte,
	Short,
	Word
}

/// Trait that represents an interface into addressable memory
pub trait Memory {
	fn read(addr: u32, width: Width) -> Result<u32>;
	fn write(addr: u32, width: Width, value: u32) -> Result<()>;
}