use super::{ CSR_BLOCK_SIZE, CsrBlock, ISR_BASE, ISR_SIZE };
use crate::state::State;

use bibe_instr::Width;

pub const DBG_OUT_BASE: u32 = ISR_BASE + ISR_SIZE;
pub const DBG_OUT_SIZE: u32 = 4 * CSR_BLOCK_SIZE;

pub const DBG_OUT_STATUS_REG: u32 = DBG_OUT_BASE;

pub const DBG_OUT_CHAR_OUT0_REG: u32 = DBG_OUT_STATUS_REG + CSR_BLOCK_SIZE;
pub const DBG_OUT_CHAR_IN0_REG: u32 = DBG_OUT_CHAR_OUT0_REG + 1;

pub const DBG_OUT_BYTE_OUT0_REG: u32 = DBG_OUT_STATUS_REG + 2 * CSR_BLOCK_SIZE;
pub const DBG_OUT_BYTE_OUT1_REG: u32 = DBG_OUT_BYTE_OUT0_REG + 1;
pub const DBG_OUT_BYTE_OUT2_REG: u32 = DBG_OUT_BYTE_OUT0_REG + 2;
pub const DBG_OUT_BYTE_OUT3_REG: u32 = DBG_OUT_BYTE_OUT0_REG + 3;

pub const DBG_OUT_GPIO_OUT0_REG: u32 = DBG_OUT_STATUS_REG + 3 * CSR_BLOCK_SIZE;
pub const DBG_OUT_GPIO_IN0_REG: u32 = DBG_OUT_GPIO_OUT0_REG + 0x04;

pub struct DbgOutBlock(());

impl DbgOutBlock {
	pub fn new() -> DbgOutBlock {
		DbgOutBlock(())
	}
}

impl CsrBlock for DbgOutBlock {
	fn read(&mut self, state: &State, reg: u32, width: Width) -> Option<u32> {
		None
	}

	fn write(&mut self, state: &State, reg: u32, width: Width, value: u32) -> Option<()> {
		log::debug!("Dbg write {reg:08x} {value:08x}");
		if reg == DBG_OUT_CHAR_OUT0_REG {
			print!("{}", char::from_u32(value).unwrap());
			return Some(())
		}

		if reg == DBG_OUT_BYTE_OUT0_REG {
			print!("{:x}", value);
			return Some(())
		}

		None
	}

	fn reset(&mut self) {
	}

	fn has_reg(&self, reg: u32) -> bool {
		true
	}

	fn base_reg(&self) -> u32 {
		DBG_OUT_BASE
	}

	fn size(&self) -> u32 {
		DBG_OUT_SIZE
	}
}