use bibe_instr::{
	csr::{Instruction, Operation},
	Width
};
use log::debug;

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

pub const CSR_BLOCK_SIZE: u32 = 64;

pub struct Register;

impl Execute for Register {
	type I = Instruction;

	fn execute(s: &mut State, instr: &Self::I) -> Result<()> {
		let width = instr.op.width();

		if width.is_none() {
			debug!("No opcode width");
			return Err(Interrupt::opcode());
		}

		if  instr.op.is_read() {
			let value = s.read_csr(instr.imm, width.unwrap()).unwrap();
				s.write_reg(instr.reg, value);
		} else {
			s.write_csr(instr.imm, s.read_reg(instr.reg), width.unwrap()).unwrap();
		}
	
		Ok(())
	}
}