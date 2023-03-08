use bibe_instr::model::{Instruction, Operation};
use num_derive::{FromPrimitive, ToPrimitive};

use crate::{
	Result,
	state::{
		Execute,
		State,
	},
};

use num_traits::FromPrimitive;

#[derive(Clone, Copy, FromPrimitive, ToPrimitive)]
pub enum Msr {
	Psr,
	IsrBase,
	IsrSp,
	IsrOldSp,
	IsrOldPc,
	IsrErr1,
	IsrErr2,
	IsrSwi,
	IsrExit,
}

pub struct Model;

impl Execute for Model {
	type I = Instruction;

	fn execute(s: &mut State, instr: &Self::I) -> Result<()> {
		match instr.op {
			Operation::Read => {
				let value = s.read_msr(Msr::from_u32(instr.imm).unwrap()).unwrap();
				s.write_reg(instr.reg, value);
			},
			Operation::Write => s.write_msr(Msr::from_u32(instr.imm).unwrap(), s.read_reg(instr.reg)).unwrap(),
			_ => unreachable!(),
		}
	
		Ok(())
	}
}