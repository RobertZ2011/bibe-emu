use super::CsrBlock;
use crate::memory::Memory;
use crate::state::State;

use bibe_instr::Width;
use bibe_instr::csr::regs::*;

pub struct DbgOutBlock(());

impl DbgOutBlock {
	pub fn new() -> DbgOutBlock {
		DbgOutBlock(())
	}
}

impl<M> CsrBlock<M> for DbgOutBlock
where
	M: Memory
{
	fn read(&mut self, _state: &State<M>, _reg: u32, _width: Width) -> Option<u32> {
		None
	}

	fn write(&mut self, _state: &State<M>, reg: u32, _width: Width, value: u32) -> Option<()> {
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

	fn has_reg(&self, _reg: u32) -> bool {
		true
	}

	fn base_reg(&self) -> u32 {
		DBG_OUT_BASE
	}

	fn size(&self) -> u32 {
		DBG_OUT_SIZE
	}
}