use bibe_instr::{
	BinOp,
	rri::{
		Instruction,
		Condition
	},
};

use crate::{
	Interrupt,
	Result,
	state::Psr,
};
use super::{
	Execute,
	State,
	util::{
		CmpResult,
		execute_binop,
	},
};

use num_traits::FromPrimitive;

pub struct Rri;

impl Execute for Rri {
	type I = Instruction;

	fn execute(s: &mut State, instr: &Self::I) -> Result<()> {
		let cmp = CmpResult::from_u32(Psr(s.read_psr()).cmp_res()).unwrap();

		match instr.cond {
			Condition::Al => (),
			Condition::Eq => if cmp != CmpResult::Eq {
				return Ok(());
			},
			Condition::Ne => if cmp == CmpResult::Eq {
				return Ok(());
			},
			Condition::Gt => if cmp != CmpResult::Gt {
				return Ok(());
			},
			Condition::Ge => if cmp != CmpResult::Gt && cmp != CmpResult::Eq {
				return Ok(());
			},
			Condition::Lt => if cmp != CmpResult::Lt {
				return Ok(());
			},
			Condition::Le => if cmp != CmpResult::Lt && cmp != CmpResult::Eq {
				return Ok(());
			},
			Condition::Nv => return Ok(()),
		};

		let src = s.read_reg(instr.src);
		let imm = (instr.imm as i32) as u32;

		if !s.target().supports_binop(instr.op) {
			return Err(Interrupt::opcode());
		}

		let res = execute_binop(instr.op, src, imm)?;

		// The cmp instruction touches psr
		if instr.op == BinOp::Cmp {
			let mut psr = Psr(s.read_psr());
			psr.set_cmp_res(res);
			s.write_psr(psr.0);
		}
		s.write_reg(instr.dest, res);

		Ok(())
	}
}