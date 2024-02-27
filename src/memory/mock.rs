/* Copyright 2024 Robert Zieba, see LICENSE file for full license. */
use bibe_instr::Width;

use crate::Result;
use super::Memory;

use core::cell::Cell;

/// Mock memory implementation for testing
pub struct Mock {
	pub value: u32,
	pub should_fail: bool,
	size: u32,
	last_addr: Cell<u32>,
}

impl Mock {
	pub fn new(size: u32) -> Self {
		Self {
			value: 0,
			size,
			last_addr: Cell::new(0),
			should_fail: false,
		}
	}

	pub fn last_addr(&self) -> u32 {
		self.last_addr.get()
	}

	pub fn resize(&mut self, new_size: u32) {
		self.size = new_size;
	}
}

impl Memory for Mock {
	fn size(&self) -> u32 {
		self.size
	}

	fn read_validated(&self, addr: u32, _width: Width) -> Result<u32> {
		if self.should_fail {
			Err(crate::Interrupt::mem_fault(addr))
		}
		else {
			self.last_addr.set(addr);
			Ok(self.value)
		}
	}

	fn write_validated(&mut self, addr: u32, _width: Width, value: u32) -> Result<()> {
		if self.should_fail {
			Err(crate::Interrupt::mem_fault(addr))
		}
		else {
			self.last_addr.set(addr);
			self.value = value;
			Ok(())
		}
	}
}