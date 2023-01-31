mod state;

use bibe_instr::{
	Instruction,
	Register,
	RegOp,
	Condition,
	rdrsrs,
	rdrsi,
};

fn main() {
	let mut state = state::State::new();

	state.execute(&[
		Instruction::RdRsI(Condition::Always, rdrsi::Instruction {
			op: RegOp::Add,
			dest: Register::R1,
			src: Register::R0,
			imm: 0x2
		}),
		Instruction::RdRsI(Condition::Always, rdrsi::Instruction {
			op: RegOp::Add,
			dest: Register::R2,
			src: Register::R0,
			imm: 0x3
		}),
		Instruction::RdRsRs(Condition::Always, rdrsrs::Instruction {
			op: RegOp::Add,
			dest: Register::R3,
			src1: Register::R1,
			src2: Register::R2,
		}),
	]);
	println!("{:?}", state);
}
