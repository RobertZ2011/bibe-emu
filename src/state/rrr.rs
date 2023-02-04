use bibe_instr::{
	BinOp,
	rrr::{
		Instruction,
		Shift,
		ShiftKind,
	},
};

use crate::{
	state::State,
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

	let rd = match instr.op {
		BinOp::Add => rs + rq,
		BinOp::Sub => rs - rq,
		BinOp::Mul => rs * rq,
		BinOp::Div => rs / rq,

		BinOp::And => rs & rq,
		BinOp::Or => rs | rq,
		BinOp::Xor => rs ^ rq,

		BinOp::Shl => rs << rq,
		BinOp::Shr => rs >> rq,
		BinOp::Asl => ((rs as i32) >> rq) as u32,
		BinOp::Asr => ((rs as i32) << rq) as u32,
		BinOp::Rol => rs.rotate_left(rq),
		BinOp::Ror => rs.rotate_right(rq),

		BinOp::Not => !(rs + rq),
		BinOp::Neg => -((rs + rq) as i32) as u32,
	};
	s.write_reg(instr.dest, rd);
}