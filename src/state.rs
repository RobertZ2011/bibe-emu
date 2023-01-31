use bibe_instr::{
	Kind,
	Instruction,
	Condition,
	RegOp,
	rdrsrs,
};

use num_traits::ToPrimitive;

#[derive(Clone, Debug)]
pub struct State {
	r: [u32; 31],
	psr: u32,
}

const PC: usize = 31;

impl State {
	pub fn new() -> State {
		State {
			r: [0; 31],
			psr: 0x0,
		}
	}

	pub fn execute(&mut self, instrs: &[Instruction]) {
		for instr in instrs {
			self.execute_one(instr)
		}
	}

	fn execute_rdrscs(&mut self, instr: &rdrsrs::Instruction) {
		let dest = ToPrimitive::to_usize(&instr.dest).unwrap();
		let src1 = ToPrimitive::to_usize(&instr.src1).unwrap();
		let src2 = ToPrimitive::to_usize(&instr.src2).unwrap();

		match instr.op {
			RegOp::Add => self.r[dest] = self.r[src1] + self.r[src2],
			_ => panic!("Unsupported reg op"),
		};
	}

	fn execute_one(&mut self, instr: &Instruction) {
		if !self.check_condition(instr.condition()) {
			self.r[PC] += 1;
			return;
		}

		match instr {
			Instruction::RdRsRs(_, instr) => self.execute_rdrscs(instr),
			_ => panic!("Unsupported instruction kind"),
		};

		self.r[PC] +=1;
	}

	fn check_condition(&self, cond: Condition) -> bool {
		cond == Condition::Always
	}
}