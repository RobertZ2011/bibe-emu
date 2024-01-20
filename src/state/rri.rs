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
		execute_binop, BinOpOverflow, check_binop,
	},
};

use num_traits::FromPrimitive;

pub struct Rri;

impl Execute for Rri {
	type I = Instruction;

	fn execute(s: &mut State, instr: &Self::I) -> Result<()> {
		let src = s.read_reg(instr.src);
		let imm = (instr.imm as i32) as u32;
		let psr = Psr(s.read_psr());

		if !s.target().supports_binop(instr.op) {
			return Err(Interrupt::opcode());
		}

		if !psr.should_execute(instr.cond) {
			return Ok(());
		}

		let res = execute_binop(instr.op, src, imm)?;

		// cc instructions touch psr
		if instr.op.is_cc() {
			let mut psr = psr;
			let BinOpOverflow {
				overflow: overflow,
				carry: carry
			} = check_binop(instr.op, src, imm);

			if overflow {
				psr.set_v(1);
			}

			if carry {
				psr.set_c(1);
			}

			psr.set_zn(res);
			s.write_psr(psr.0);
		}
		s.write_reg(instr.dest, res);

		Ok(())
	}
}