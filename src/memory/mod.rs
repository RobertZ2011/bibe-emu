/* Copyright 2023 Robert Zieba, see LICENSE file for full license. */
use crate::{
	Exception,
	Result,
};

use bibe_instr::Width;

mod image;
mod mapped;
mod mock;

pub use image::Image;
pub use mapped::Mapped;
pub use mock::Mock;

pub struct RegionSlice<'a> {
	parent: &'a mut dyn Memory,
	_start: u32,
	size: u32,
}

/// Represents an interface into addressable memory
pub trait Memory {
	fn read(&self, addr: u32, width: Width) -> Result<u32> {
		if !self.contains(addr) || !self.validate_access(addr, width) {
			return Err(Exception::mem_fault(addr, false));
		}

		self.read_validated(addr, width).map(move |x| x & width.to_mask())
	}

	fn write(&mut self, addr: u32, width: Width, value: u32) -> Result<()> {
		if !self.contains(addr) || !self.validate_access(addr, width) {
			return Err(Exception::mem_fault(addr, false));
		}

		self.write_validated(addr, width, value & width.to_mask())
	}
	fn contains(&self, addr: u32) -> bool {
		addr < self.size()
	}

	/*fn slice<'a>(&self, start: u32, size: u32) -> Option<RegionSlice<'a>> {
		if start > self.size() || start + size > self.size() {

		}

		Some(RegionSlice {
			parent: self as &dyn Memory,
			start,
			size,
		})
	}*/

	/// Returns true if the access is entirely contained in the region
	fn validate_access(&self, addr: u32, width: Width) -> bool
	{
		addr + width - 1 < self.size()
	}

	fn size(&self) -> u32;

	/// Perform the memory read, addr has already been validated
	fn read_validated(&self, addr: u32, _width: Width) -> Result<u32> {
		Err(Exception::mem_fault(addr, false))
	}

	/// Perform the memory write, addr has already been validated
	fn write_validated(&mut self, addr: u32, _width: Width, _value: u32) -> Result<()> {
		Err(Exception::mem_fault(addr, false))
	}
}

impl Memory for RegionSlice<'_> {
	fn size(&self) -> u32 {
		self.size
	}

	fn read_validated(&self, addr: u32, width: Width) -> Result<u32> {
		self.parent.read_validated(addr, width)
	}

	fn write_validated(&mut self, addr: u32, width: Width, value: u32) -> Result<()> {
		self.parent.write_validated(addr, width, value)
	}
}

#[cfg(test)]
mod test {
	use super::*;

	#[test]
	fn test_validation() {
		let mut r = Mock::new(32);

		// `validate_access` tests
		// Test at beginning
		assert!(r.validate_access(0, Width::Byte));
		assert!(r.validate_access(0, Width::Short));
		assert!(r.validate_access(0, Width::Word));

		// Test at end
		assert!(r.validate_access(30, Width::Byte));
		assert!(r.validate_access(29, Width::Short));
		assert!(r.validate_access(27, Width::Word));

		// Test exact size
		r.resize(1);
		assert!(r.validate_access(0, Width::Byte));

		r.resize(2);
		assert!(r.validate_access(0, Width::Short));

		r.resize(4);
		assert!(r.validate_access(0, Width::Word));
	}
}