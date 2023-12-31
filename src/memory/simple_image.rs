/* Copyright 2023 Robert Zieba, see LICENSE file for full license. */
use bibe_instr::Width;

use crate::Result;

use std::io;

use super::Memory;

#[derive(Debug)]
pub struct SimpleImage {
	mem: Vec<u8>,
}

/// Simple memory device that stores its data as a Vec
impl SimpleImage {
	pub fn new(size: u32) -> Self {
		Self {
			mem: vec![0; size as usize],
		}
	}

	pub fn load(r: &mut dyn io::Read) -> Self {
		let mut data = Vec::new();
		r.read_to_end(&mut data).expect("Failed to load image");

		Self {
			mem: data
		}
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

impl Memory for SimpleImage {
	fn size(&self) -> u32 {
		self.mem.len() as u32
	}

	fn read_validated(&self, addr: u32, width: Width) -> Result<u32> {
		Ok(match width {
			Width::Byte => self.get(addr),
			Width::Short => self.get(addr) | self.get(addr + 1) << 8,
			Width::Word => self.get(addr) | self.get(addr + 1) << 8 | 
						self.get(addr  + 2) << 16 | self.get(addr + 3) << 24,
		})
	}

	fn write_validated(&mut self, addr: u32, width: Width, value: u32) -> Result<()> {
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