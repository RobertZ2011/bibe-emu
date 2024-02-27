use bibe_instr::rrr::Instruction;

use crate::{
	memory::Memory, state::Psr, target::Target, Interrupt, Result
};
use super::{
	csr::CsrCollection, shift, util::{
		check_binop, execute_binop, BinOpOverflow
	}, State
};

pub(super) fn execute<T, M, C>(s: &mut State<T, M, C>, instr: &Instruction) -> Result<()>
where
	T: Target,
	M: Memory,
	C: CsrCollection,
{
	let rs = s.core.borrow().read_reg(instr.lhs);
	let rq = shift(&instr.shift, s.core.borrow().read_reg(instr.rhs));

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
	s.core.borrow_mut().write_reg(instr.dest, res);
	Ok(())
}