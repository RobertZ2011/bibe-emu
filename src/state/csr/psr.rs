use super::CsrBlock;
use crate::state::CoreState;

use bibe_instr::Width;
use bibe_instr::csr::regs::*;

pub struct PsrBlock(pub(crate) u32);

impl PsrBlock {
	pub fn new() -> PsrBlock {
		PsrBlock(0)
	}
}

impl CsrBlock for PsrBlock
{
    fn base_reg(&self) -> u32 {
		PSR_BASE
	}

	fn size(&self) -> u32 {
		PSR_SIZE
	}

	fn has_reg(&self, reg: u32) -> bool {
		reg == PSR_PSR0_REG
	}

	fn read(&mut self, _state: &CoreState, reg: u32, width: Width) -> Option<u32> {
		if width != Width::Word {
			return None;
		}

		if reg - PSR_BASE == PSR_PSR0_REG {
			return Some(self.0);
		}

		None
	}

	fn write(&mut self, _state: &CoreState, reg: u32, width: Width, value: u32) -> Option<()> {
		if width != Width::Word {
			return None;
		}

		if reg - PSR_BASE == PSR_PSR0_REG {
			self.0 = value;
			return Some(());
		}

		None
	}

	fn reset(&mut self) {
		self.0 = 0;
	}
}