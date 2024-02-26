use bibe_instr::Width;
use bibe_instr::csr::Instruction;
use bibe_instr::csr::regs::*;

use crate::memory::Memory;
use crate::{
	Result,
	state::State,
	Interrupt,
};

mod dbg_out;
mod isr;
mod psr;

pub use dbg_out::*;
pub use isr::*;
pub use psr::*;

use super::CoreState;

pub trait CsrBlock
{
	fn read(&mut self, state: &CoreState, reg: u32, width: Width) -> Option<u32>;
	fn write(&mut self, state: &CoreState, reg: u32, width: Width, value: u32) -> Option<()>;
	fn reset(&mut self);

	fn has_reg(&self, reg: u32) -> bool;
	fn base_reg(&self) -> u32;
	fn size(&self) -> u32;

	// Downcasting helpers, these should only be added for ISA defined blocks
	fn as_isr(&self) -> Option<&IsrBlock> { None }
	fn as_isr_mut(&mut self) -> Option<&mut IsrBlock> { None }
}

pub(super) fn execute<M: Memory>(s: &mut State<M>, instr: &Instruction) -> Result<()> {
	let width = instr.op.width;

	if  instr.op.is_load() {
		let value = s.read_csr(instr.imm, width).unwrap();
			s.core.borrow_mut().write_reg(instr.reg, value);
	} else {
		//TODO: remove this hack
		if instr.imm == ISR_ENTER_REG {
			return Err(Interrupt::swi());
		}
		let val = s.core.borrow().read_reg(instr.reg);
		s.write_csr(instr.imm, val, width).unwrap();
	}

	Ok(())
}