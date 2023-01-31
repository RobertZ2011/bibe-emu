use bibe_instr::{
	Condition,
	Instruction,
	Kind,
	Register,
	RegOp,
	rdrsrs,
	rdrsi,
};

use num_traits::ToPrimitive;

#[derive(Clone, Debug)]
pub struct State {
	r: [u32; 31],
	psr: u32,
}

const PC: usize = 30;

impl State {
	pub fn new() -> State {
		State {
			r: [0; 31],
			psr: 0x0,
		}
	}

	fn read_reg(&self, r: Register) -> u32 {
		if r == Register::R0 {
			0
		} else {
			self.r[ToPrimitive::to_usize(&r).unwrap() - 1]
		}
	}

	fn write_reg(&mut self, r: Register, value: u32){
		if r != Register::R0 {
			self.r[ToPrimitive::to_usize(&r).unwrap() - 1] = value;
		}
	}

	fn execute_rdrscs(&mut self, instr: &rdrsrs::Instruction) {
		let src1 = self.read_reg(instr.src1);
		let src2 = self.read_reg(instr.src2);

		let res = match instr.op {
			RegOp::Add => src1 + src2,
			_ => panic!("Unsupported reg op"),
		};
		self.write_reg(instr.dest, res);
	}

	fn execute_rdrsi(&mut self, instr: &rdrsi::Instruction) {
		let src = self.read_reg(instr.src);
		let imm = instr.imm;

		let res = match instr.op {
			RegOp::Add => src + imm,
			_ => panic!("Unsupported reg op"),
		};
		self.write_reg(instr.dest, res);
	}

	fn check_condition(&self, cond: Condition) -> bool {
		cond == Condition::Always
	}

	fn execute_one(&mut self, instr: &Instruction) {
		if !self.check_condition(instr.condition()) {
			self.r[PC] += 1;
			return;
		}

		match instr {
			Instruction::RdRsRs(_, instr) => self.execute_rdrscs(instr),
			Instruction::RdRsI(_, instr) => self.execute_rdrsi(instr),
			_ => panic!("Unsupported instruction kind"),
		};

		self.r[PC] +=1;
	}

	pub fn execute(&mut self, instrs: &[Instruction]) {
		for instr in instrs {
			self.execute_one(instr)
		}
	}
}