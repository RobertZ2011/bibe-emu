use bibe_instr::{
	BinOp,
	rri::{
		Instruction,
		Condition
	},
};

use crate::{
	Result,
	state::{
		CmpResult,
		execute_binop,
		State,
	}
};

pub fn execute(s: &mut State, instr: &Instruction) -> Result<()> {
	let cmp = CmpResult::from_psr(s.read_psr());

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
	let res = execute_binop(instr.op, src, imm);

	// The cmp instruction touches psr
	if instr.op == BinOp::Cmp {
		let mut psr = s.read_psr();
		psr &= !0x3;
		psr |= res;
		s.write_psr(psr);
	}
	s.write_reg(instr.dest, res);

	Ok(())
}