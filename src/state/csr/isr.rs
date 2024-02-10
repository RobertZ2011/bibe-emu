/* Copyright 2023 Robert Zieba, see LICENSE file for full license. */
use super::CsrBlock;
use crate::state::State;

use bibe_instr::Width;
use bibe_instr::csr::regs::*;

pub struct IsrBlock(pub(crate) [u32; ISR_SIZE as usize / 4]);

impl IsrBlock {
	pub fn new() -> IsrBlock {
		IsrBlock([0; ISR_SIZE as usize / 4])
	}
}

impl CsrBlock for IsrBlock {
	fn read(&mut self, _state: &State, reg: u32, width: Width) -> Option<u32> {
		if width != Width::Word {
			return None;
		}

		Some(self.0[((reg - ISR_BASE) / 4) as usize])
	}

	fn write(&mut self, _state: &State, reg: u32, width: Width, value: u32) -> Option<()> {
		if width != Width::Word {
			return None;
		}

		self.0[((reg - ISR_BASE) / 4) as usize] = value;
		Some(())
	}

	fn reset(&mut self) {
		for reg in &mut self.0 {
			*reg = 0;
		}
	}

	fn has_reg(&self, reg: u32) -> bool {
		if reg < ISR_BASE && reg >= ISR_BASE + ISR_SIZE {
			return false;
		}

		(reg >= ISR_BASE_REG && reg <= ISR_EXIT_REG) || (reg >= ISR_R1_REG && reg <= ISR_PC_REG)
	}

	fn base_reg(&self) -> u32 {
		ISR_BASE
	}

	fn size(&self) -> u32 {
		ISR_SIZE
	}

	fn as_isr(&self) -> Option<&IsrBlock> {
		Some(self)
	}

	fn as_isr_mut(&mut self) -> Option<&mut IsrBlock> {
		Some(self)
	}
}