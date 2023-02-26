use bibe_instr::{
	BinOp,
	rrr::{
		Instruction,
		Shift,
		ShiftKind,
	},
};

use crate::{
	Result,
	state::{
		execute_binop,
		State,
	},
};

fn shift(s: &Shift, value: u32) -> u32 {
	let Shift {
		kind,
		shift: amount,
	} = s;

	match kind {
		ShiftKind::Shl => value << amount,
		ShiftKind::Shr => value >> amount,
		ShiftKind::Asl => ((value as i32) << amount) as u32,
		ShiftKind::Asr => ((value as i32) >> amount) as u32,
		ShiftKind::Rol => value.rotate_left(*amount as u32),
		ShiftKind::Ror => value.rotate_right(*amount as u32)
	}
}

pub fn execute(s: &mut State, instr: &Instruction) -> Result<()> {
	let rs = s.read_reg(instr.lhs);
	let rq = shift(&instr.shift, s.read_reg(instr.rhs));
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