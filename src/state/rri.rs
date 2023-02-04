use bibe_instr::{
	BinOp,
	rri::{
		Instruction,
		Condition
	},
};

use crate::state::{
	CmpResult,
	execute_binop,
	State,
};

pub fn execute(s: &mut State, instr: &Instruction) {
	let cmp = CmpResult::from_psr(s.read_psr());
	if cmp == CmpResult::False {
		// Never execute in this case
		return;
	}
	
	match instr.cond {
		Condition::Al => (),
		Condition::Eq => if cmp != CmpResult::Eq {
			return;
		},
		Condition::Ne => if cmp == CmpResult::Eq {
			return;
		},
		Condition::Gt => if cmp != CmpResult::Gt {
			return;
		},
		Condition::Ge => if cmp != CmpResult::Gt && cmp != CmpResult::Eq {
			return;
		},
		Condition::Lt => if cmp != CmpResult::Lt {
			return;
		},
		Condition::Le => if cmp != CmpResult::Lt && cmp != CmpResult::Eq {
			return;
		},
		Condition::Nv => return,
	};

	let src = s.read_reg(instr.src);
	let imm = (instr.imm as i32) as u32;
	let res = execute_binop(instr.op, src, imm);

	// The cmp instruction touches psr
	if instr.op == BinOp::Cmp {
		let mut psr = s.read_psr();
		psr &= 0x3;
		psr |= res;
		s.write_psr(psr);
	}
	s.write_reg(instr.dest, res);
}