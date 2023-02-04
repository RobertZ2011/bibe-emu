use bibe_instr::{
	rrr::{
		Instruction,
		Shift,
		ShiftKind,
	},
};

use crate::state::{
	execute_binop,
	State,
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

pub fn execute(s: &mut State, instr: &Instruction) {
	let rs = s.read_reg(instr.src1);
	let rq = shift(&instr.shift, s.read_reg(instr.src2));
	s.write_reg(instr.dest, execute_binop(instr.op, rs, rq));
}