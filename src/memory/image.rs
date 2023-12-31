use std::{
	cell::RefCell,
};

use super::{
	Mapped,
	Memory,
	SimpleImage,
};

#[derive(Clone, Copy, Debug)]
pub enum PageSize {
	K4,
	M1,
	M4,
	M16,
	M32,
	M64,
	M128,
	M256,
}

impl Into<u32> for PageSize {
	fn into(self) -> u32 {
		match self {
			PageSize::K4 => 4096,
			PageSize::M1 => 1024 * 1024,
			PageSize::M4 => 4 * 1024 * 1024,
			PageSize::M16 => 16 * 1024 * 1024,
			PageSize::M32 => 32 * 1024 * 1024,
			PageSize::M64 => 64 * 1024 * 1024,
			PageSize::M128 => 128 * 1024 * 1024,
			PageSize::M256 => 256 * 1024 * 1024,
		}
	}
}

/// Efficient memory device, memory is not allocated until needed
pub struct Image {
	mapped: RefCell<Mapped>,
	page_size: PageSize,
}

impl Image {
	pub fn new(page_size: PageSize) -> Self {
		Self {
			mapped: RefCell::new(Mapped::new()),
			page_size
		}
	}

	fn create_page(&self, addr: u32) -> Option<()> {
		let mapped = &mut self.mapped.borrow_mut();
		let start_addr = addr;

		if self.contains(addr) {
			return None;
		}

		let image = Box::new(SimpleImage::new(self.page_size.into())) as Box<dyn Memory>;
		mapped.map(start_addr, image)?;

		Some(())
	}
}

impl Memory for Image {
	fn size(&self) -> u32 {
		self.mapped.borrow().size()
	}

	fn read(&self, addr: u32, width: bibe_instr::Width) -> crate::Result<u32> {
		if !self.mapped.borrow().contains(addr) {
			self.create_page(addr);
		}

		self.mapped.borrow().read(addr, width)
	}

	fn write(&mut self, addr: u32, width: bibe_instr::Width, value: u32) -> crate::Result<()> {
		if !self.mapped.borrow().contains(addr) {
			self.create_page(addr);
		}

		self.mapped.borrow_mut().write(addr, width, value)
	}
}