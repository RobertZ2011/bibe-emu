use bibe_instr::{
	BinOp,
	rrr::Instruction,
};

use crate::{
	Exception,
	Result,
};
use super::{
	Execute,
	State,
	util::execute_binop,
	shift
};

pub struct Rrr;

impl Execute for Rrr {
	type I = Instruction;

	fn execute(s: &mut State, instr: &Self::I) -> Result<()> {
		let rs = s.read_reg(instr.lhs);
		let rq = shift(&instr.shift, s.read_reg(instr.rhs));

		if !s.target().supports_binop(instr.op) {
			return Err(Exception::opcode());
		}

		let res = execute_binop(instr.op, rs, rq)?;
	
		// The cmp instruction touches psr
		if instr.op == BinOp::Cmp {
			let mut psr = s.read_psr();
			psr.set_cmp_res(res);
			s.write_psr(psr);
		}
		s.write_reg(instr.dest, res);
		Ok(())
	}
}