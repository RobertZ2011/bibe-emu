use bibe_instr::memory::Width;

use crate::{
	Exception,
	memory::Memory,
	Result,
};

use std::io;

#[derive(Debug)]
pub struct Image {
	mem: Vec<u8>,
}

impl Image {
	pub fn load(r: &mut dyn io::Read) -> Image {
		let mut data = Vec::new();
		r.read_to_end(&mut data).expect("Failed to load image");

		Image {
			mem: data
		}
	}

	fn validate_addr(&self, addr: u32, width: Width) -> Result<()> {
		if addr as usize > self.mem.len() {
			return Err(Exception::MemFault);
		}

		let aligned = match width {
			Width::Short => addr & 0x1 == 0,
			Width::Word => addr & 0x3 == 0,
			_ => true,
		};

		if !aligned {
			return Err(Exception::UnalignedAccess);
		}

		Ok(())
	}

	#[inline(always)]
	fn get(&self, addr: u32) -> u32 {
		self.mem[addr as usize].into()
	}

	#[inline(always)]
	fn set(&mut self, addr: u32, value: u32) {
		self.mem[addr as usize] = value as u8;
	}
}

impl Memory for Image {
	fn read(&self, addr: u32, width: Width) -> Result<u32> {
		let _ = self.validate_addr(addr, width)?;

		Ok(match width {
			Width::Byte => self.get(addr),
			Width::Short => self.get(addr) | self.get(addr + 1) << 8,
			Width::Word => self.get(addr) | self.get(addr + 1) << 8 | 
						self.get(addr  + 2) << 16 | self.get(addr + 3) << 24,
		})
	}

	fn write(&mut self, addr: u32, width: Width, value: u32) -> Result<()> {
		let _ = self.validate_addr(addr, width)?;

		Ok(match width {
			Width::Byte => self.set(addr, value & 0xff),
			Width::Short => {
				self.set(addr, value & 0xff);
				self.set(addr + 1, (value >> 8) & 0xff);
			},
			Width::Word => {
				self.set(addr, value & 0xff);
				self.set(addr + 1, (value >> 8) & 0xff);
				self.set(addr + 2, (value >> 16) & 0xff);
				self.set(addr + 3, (value >> 24) & 0xff);
			},
		})
	}
}