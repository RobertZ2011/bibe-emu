use bibe_instr::rrr::Instruction;

use crate::{
	memory::Memory, state::Psr, Interrupt, Result
};
use super::{
	Execute,
	State,
	util::{
		execute_binop,
		check_binop,
		BinOpOverflow,
	},
	shift
};

pub struct Rrr;

impl<M> Execute<M> for Rrr
where
	M: Memory
{
	type I = Instruction;

	fn execute(s: &mut State<M>, instr: &Self::I) -> Result<()> {
		let rs = s.read_reg(instr.lhs);
		let rq = shift(&instr.shift, s.read_reg(instr.rhs));

		if !s.target().supports_binop(instr.op) {
			return Err(Interrupt::opcode());
		}

		let res = execute_binop(instr.op, rs, rq)?;
	
		// cc instructions touch psr
		if instr.op.is_cc() {
			let mut psr = Psr(s.read_psr());
			let BinOpOverflow {
				overflow,
				carry
			} = check_binop(instr.op, rs, rq);

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