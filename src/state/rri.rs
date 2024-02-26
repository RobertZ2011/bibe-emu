use bibe_instr::rri::Instruction;

use crate::{
	memory::Memory, state::Psr, Interrupt, Result
};
use super::{
	util::{
		BinOpOverflow,
		check_binop,
		execute_binop
	},
	State
};

pub(super) fn execute<M: Memory>(s: &mut State<M>, instr: &Instruction) -> Result<()> {
	let src = s.core.borrow().read_reg(instr.src);
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
			overflow,
			carry
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
	s.core.borrow_mut().write_reg(instr.dest, res);

	Ok(())
}