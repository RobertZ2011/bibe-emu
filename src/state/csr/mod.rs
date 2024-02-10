use bibe_instr::{
	csr::Instruction,
	Width
};
use bibe_instr::csr::regs::*;

use crate::{
	Result,
	state::{
		Execute,
		State,
	}, Interrupt,
};

mod dbg_out;
mod isr;
mod psr;

pub use dbg_out::*;
pub use isr::*;
pub use psr::*;

pub trait CsrBlock {
	fn read(&mut self, state: &State, reg: u32, width: Width) -> Option<u32>;
	fn write(&mut self, state: &State, reg: u32, width: Width, value: u32) -> Option<()>;
	fn reset(&mut self);

	fn has_reg(&self, reg: u32) -> bool;
	fn base_reg(&self) -> u32;
	fn size(&self) -> u32;

	// Downcasting helpers, these should only be added for ISA defined blocks
	fn as_isr(&self) -> Option<&IsrBlock> { None }
	fn as_isr_mut(&mut self) -> Option<&mut IsrBlock> { None }
}

pub struct Register;

impl Execute for Register {
	type I = Instruction;

	fn execute(s: &mut State, instr: &Self::I) -> Result<()> {
		let width = instr.op.width;

		if  instr.op.is_load() {
			let value = s.read_csr(instr.imm, width).unwrap();
				s.write_reg(instr.reg, value);
		} else {
			//TODO: remove this hack
			if instr.imm == ISR_ENTER_REG {
				return Err(Interrupt::swi());
			}
			s.write_csr(instr.imm, s.read_reg(instr.reg), width).unwrap();
		}
	
		Ok(())
	}
}